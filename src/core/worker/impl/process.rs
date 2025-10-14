use crate::core::rating::from::rates_from_skillset_scores;
use crate::core::rating::make_rates::RatesMaker;
use crate::errors::BeatmapWorkerError;
use crate::utils::determine_main_pattern;
use crate::utils::osu_file_from_url;
// use crate::utils::rate::file_manager::FileManager;
use dto::models::beatmaps::full::types::Beatmap;
use minacalc_rs::{hashmap::HashMapCalcExt, osu::OsuCalcExt, Calc};
use rosu_map::Beatmap as RmBeatmap;
use rosu_v2::prelude::BeatmapExtended;
use std::str::FromStr;
use std::time::Instant;
use tracing::{debug, info, warn};
use crate::utils::rate::rate::process_single_rate;

pub(crate) async fn process_beatmap(
    beatmap: &BeatmapExtended,
    calc: &Calc,
    osu_path: String,
    beatmap_row: &mut Beatmap,
) -> Result<(), BeatmapWorkerError> {
    let start_all = Instant::now();
    debug!("Starting beatmap processing for osu_id: {}", beatmap.map_id);

    debug!("Fetching osu file from URL: {}", osu_path);
    let osu_map = osu_file_from_url(&osu_path).await.unwrap();
    info!("Osu file fetched, length: {} bytes", osu_map.len());

    let parsed_beatmap = RmBeatmap::from_str(&osu_map).unwrap();
    debug!("Beatmap parsed successfully");

    let rates_centirate = vec![
        70, 80, 90, 100, 110, 120, 130, 140, 150, 160, 170, 180, 190, 200,
    ];
    debug!(
        "Processing {} rates (centirate): {:?}",
        rates_centirate.len(),
        rates_centirate
    );

    // Calculer les scores de skillset pour tous les rates
    debug!("Calculating skillset scores with minacalc...");
    let skillset_scores = calc
        .calculate_msd_from_string(osu_map.clone())
        .map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?
        .as_hashmap()
        .map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?;
    debug!(
        "Skillset scores calculated for {} rates",
        skillset_scores.len()
    );

    beatmap_row.main_pattern = determine_main_pattern(&skillset_scores["1.0"], &parsed_beatmap);
    // Boucle simple: calculer et stocker le résultat (apply rate déporté dans RatesMaker)
    for centirate in rates_centirate {
        let rate = centirate as f64 / 100.0;
        let rate_string = format!("{:.1}", rate);

        if let Some(scores) = skillset_scores.get(&rate_string) {
            debug!(
                "Processing centirate {} ({}x) with scores: overall={:.2}, stream={:.2}, jumpstream={:.2}",
                centirate, rate_string, scores.overall, scores.stream, scores.jumpstream
            );

            let mut rates_maker = RatesMaker {
                skillset_scores: scores.clone(),
                osu_map: osu_map.clone(),
                centirate,
                drain_time: beatmap.seconds_drain as f64,
                total_time: beatmap.seconds_total as f64,
                bpm: beatmap.bpm as f32,
            };

            let hash = process_single_rate(centirate as i64, &parsed_beatmap, beatmap.map_id as i32);
            
            let rates =
                rates_from_skillset_scores(&mut rates_maker, hash)
                    .await
                    .unwrap();

            beatmap_row.rates.push(rates);
        } else {
            warn!("No skillset scores found for rate: {}", rate_string);
        }
    }

    let elapsed = start_all.elapsed();
    info!(
        "process_beatmap done: osu_id={}, elapsed_ms={}",
        beatmap_row.osu_id.unwrap_or_default(),
        elapsed.as_millis()
    );
    Ok(())
}
