use crate::core::worker::types::BeatmapWorker;
use crate::errors::BeatmapWorkerError;
use anyhow::Result;
use bigdecimal::{BigDecimal, FromPrimitive};
use db::models::beatmaps::beatmap::BeatmapRow;
use db::models::beatmaps::beatmapset::BeatmapsetRow;
use db::models::beatmaps::rates::RatesRow;
use db::models::rating::beatmap_mania_rating::BeatmapManiaRatingRow;
use db::models::rating::beatmap_rating::BeatmapRatingRow;
use dto::models::beatmaps::full::types::{
    Beatmapset as DtoBeatmapset,
};
use dto::models::rate::ModeRating;

/// Insert a full beatmapset hierarchy into the database using database-lib only.
/// Order:
/// - beatmapset -> beatmap(s) -> rates -> rating(s) -> mania rating(s)
pub async fn insert_full_beatmapset(
    worker: &BeatmapWorker,
    dto: &DtoBeatmapset,
) -> Result<i32, BeatmapWorkerError> {
    let pool = worker.config.database.get_pool();

    // Insert beatmapset (ignore if duplicate by osu_id and reuse existing)
    let beatmapset_row = BeatmapsetRow {
        id: 0,
        osu_id: dto.osu_id,
        artist: dto.artist.clone(),
        artist_unicode: dto.artist_unicode.clone(),
        title: dto.title.clone(),
        title_unicode: dto.title_unicode.clone(),
        creator: dto.creator.clone(),
        source: dto.source.clone(),
        // database expects Option<Vec<String>> while dto has Option<String>
        // minimal handling: split by space when provided
        tags: dto.tags.as_ref().map(|s| {
            s.split(' ')
                .filter(|t| !t.is_empty())
                .map(|t| t.to_string())
                .collect()
        }),
        has_video: dto.has_video,
        has_storyboard: dto.has_storyboard,
        is_explicit: dto.is_explicit,
        is_featured: dto.is_featured,
        cover_url: dto.cover_url.clone(),
        preview_url: dto.preview_url.clone(),
        osu_file_url: dto.osu_file_url.clone(),
        created_at: None,
        updated_at: None,
    };

    let beatmapset_id = if let Some(osu_id) = beatmapset_row.osu_id {
        match BeatmapsetRow::find_by_osu_id(pool, osu_id).await {
            Ok(Some(existing)) => existing.id,
            Ok(None) => BeatmapsetRow::insert(beatmapset_row, pool)
                .await
                .map_err(|e| BeatmapWorkerError::ProcessingFailed(e.to_string()))?,
            Err(e) => return Err(BeatmapWorkerError::ProcessingFailed(e.to_string())),
        }
    } else {
        BeatmapsetRow::insert(beatmapset_row, pool)
            .await
            .map_err(|e| BeatmapWorkerError::ProcessingFailed(e.to_string()))?
    };

    // Insert each beatmap and its rates/ratings
    for dto_b in &dto.beatmaps {
        let beatmap_row = BeatmapRow {
            id: 0,
            osu_id: dto_b.osu_id,
            beatmapset_id: Some(beatmapset_id),
            difficulty: dto_b.difficulty.clone(),
            count_circles: dto_b.count_circles,
            count_sliders: dto_b.count_sliders,
            count_spinners: dto_b.count_spinners,
            max_combo: dto_b.max_combo,
            main_pattern: dto_b.main_pattern.clone(),
            cs: BigDecimal::from_f64(dto_b.cs).unwrap_or_else(|| BigDecimal::from(0)),
            ar: BigDecimal::from_f64(dto_b.ar).unwrap_or_else(|| BigDecimal::from(0)),
            od: BigDecimal::from_f64(dto_b.od).unwrap_or_else(|| BigDecimal::from(0)),
            hp: BigDecimal::from_f64(dto_b.hp).unwrap_or_else(|| BigDecimal::from(0)),
            mode: dto_b.mode,
            status: dto_b.status.clone(),
            created_at: None,
            updated_at: None,
        };

        // Insert beatmap (ignore if duplicate by osu_id and reuse existing)
        let beatmap_id = if let Some(osu_id) = beatmap_row.osu_id {
            match BeatmapRow::find_by_osu_id(pool, osu_id).await {
                Ok(Some(existing)) => existing.id,
                Ok(None) => BeatmapRow::insert(beatmap_row, pool)
                    .await
                    .map_err(|e| BeatmapWorkerError::ProcessingFailed(e.to_string()))?,
                Err(e) => return Err(BeatmapWorkerError::ProcessingFailed(e.to_string())),
            }
        } else {
            BeatmapRow::insert(beatmap_row, pool)
                .await
                .map_err(|e| BeatmapWorkerError::ProcessingFailed(e.to_string()))?
        };

        for dto_r in &dto_b.rates {
            let rates_row = RatesRow {
                id: 0,
                beatmap_id,
                osu_hash: dto_r.osu_hash.clone().unwrap_or_default(),
                centirate: dto_r.centirate,
                drain_time: dto_r.drain_time,
                total_time: dto_r.total_time,
                bpm: BigDecimal::from_f32(dto_r.bpm).unwrap_or_else(|| BigDecimal::from(0)),
                created_at: None,
            };

            let rates_id = RatesRow::insert(rates_row, pool)
                .await
                .map_err(|e| BeatmapWorkerError::ProcessingFailed(e.to_string()))?;

            for dto_rating in &dto_r.rating {
                let rating_row = BeatmapRatingRow {
                    id: 0,
                    rates_id: Some(rates_id),
                    rating: BigDecimal::from_f64(dto_rating.rating)
                        .unwrap_or_else(|| BigDecimal::from(0)),
                    rating_type: dto_rating.rating_type.clone(),
                    created_at: None,
                };

                let rating_id = BeatmapRatingRow::insert(rating_row, pool)
                    .await
                    .map_err(|e| BeatmapWorkerError::ProcessingFailed(e.to_string()))?;

                if let ModeRating::Mania(mr) = &dto_rating.mode_rating {
                    let mania_row = BeatmapManiaRatingRow {
                        id: 0,
                        rating_id: Some(rating_id),
                        stream: Some(
                            BigDecimal::from_f64(mr.stream).unwrap_or_else(|| BigDecimal::from(0)),
                        ),
                        jumpstream: Some(
                            BigDecimal::from_f64(mr.jumpstream)
                                .unwrap_or_else(|| BigDecimal::from(0)),
                        ),
                        handstream: Some(
                            BigDecimal::from_f64(mr.handstream)
                                .unwrap_or_else(|| BigDecimal::from(0)),
                        ),
                        stamina: Some(
                            BigDecimal::from_f64(mr.stamina).unwrap_or_else(|| BigDecimal::from(0)),
                        ),
                        jackspeed: Some(
                            BigDecimal::from_f64(mr.jackspeed)
                                .unwrap_or_else(|| BigDecimal::from(0)),
                        ),
                        chordjack: Some(
                            BigDecimal::from_f64(mr.chordjack)
                                .unwrap_or_else(|| BigDecimal::from(0)),
                        ),
                        technical: Some(
                            BigDecimal::from_f64(mr.technical)
                                .unwrap_or_else(|| BigDecimal::from(0)),
                        ),
                        created_at: None,
                        updated_at: None,
                    };
                    let _ = BeatmapManiaRatingRow::insert(mania_row, pool)
                        .await
                        .map_err(|e| BeatmapWorkerError::ProcessingFailed(e.to_string()))?;
                }
            }
        }
    }

    Ok(beatmapset_id)
}
