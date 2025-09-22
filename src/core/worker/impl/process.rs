use dto::models::beatmaps::full::types::Beatmap;
use crate::core::rating::from::rates_from_skillset_scores;
use crate::core::rating::make_rates::RatesMaker;
use dto::models::beatmaps::full::types::Rates;
use crate::errors::BeatmapWorkerError;
use crate::utils::osu_file_from_url;
use crate::utils::rate::beatmap_processor::BeatmapProcessor;
use crate::utils::rate::compression::CompressionManager;
use crate::utils::rate::file_manager::FileManager;
use minacalc_rs::{hashmap::HashMapCalcExt, osu::OsuCalcExt, Calc, Ssr};
use rosu_map::Beatmap as RmBeatmap;
use rosu_v2::prelude::BeatmapExtended;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;
use tracing::info;

/// Structure unifiée contenant à la fois le beatmap et les scores de skillset
#[derive(Debug, Clone)]
struct BeatmapWithScores {
    beatmap: RmBeatmap,
    skillset_scores: Ssr,
}

pub fn apply_rate(rate: f64, map: &mut RmBeatmap) {
    BeatmapProcessor::apply_rate(rate, map);
}
pub(crate) async fn process_beatmap(
    beatmap: &BeatmapExtended,
    calc: &Calc,
    osu_path: String,
    beatmap_row: &mut Beatmap,
) -> Result<(), BeatmapWorkerError> {
    let start_all = Instant::now();
    let osu_map = osu_file_from_url(&osu_path).await.unwrap();
    let parsed_beatmap = RmBeatmap::from_str(&osu_map).unwrap();
    let rates = vec![
        0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0,
    ];

    // Calculer les scores de skillset pour tous les rates
    let skillset_scores = calc
        .calculate_msd_from_string(osu_map.clone())
        .map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?
        .as_hashmap()
        .map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?;

    // Créer une hash map unifiée contenant beatmaps + scores
    let mut beatmaps_with_scores: HashMap<String, BeatmapWithScores> = HashMap::new();

    for rate in rates {
        let mut processed_beatmap = parsed_beatmap.clone();
        apply_rate(rate, &mut processed_beatmap);
        // Les clés de skillset_scores sont au format "1.7", "1.0", etc. (une décimale)
        let rate_string = format!("{:.1}", rate);

        // Récupérer les scores correspondants
        if let Some(scores) = skillset_scores.get(&rate_string) {
            beatmaps_with_scores.insert(
                rate_string.clone(),
                BeatmapWithScores {
                    beatmap: processed_beatmap,
                    skillset_scores: scores.clone(),
                },
            );
        }
    }

    // Traiter chaque entrée unifiée
    for (rate_key, beatmap_with_scores) in beatmaps_with_scores {
        let mut rates_maker = RatesMaker {
            skillset_scores: beatmap_with_scores.skillset_scores,
            osu_map: beatmap_with_scores.beatmap,
            rate: rate_key,
            drain_time: beatmap.seconds_drain as f64,
            total_time: beatmap.seconds_total as f64,
            bpm: beatmap.bpm as f32,
        };
        let rates: Rates = rates_from_skillset_scores(&mut rates_maker).await.unwrap();

        // Sauvegarder le beatmap compressé en Brotli, nommé par le hash calculé
        if let (Some(osu_id), Some(hash)) = (beatmap_row.osu_id, &rates.osu_hash) {
            // S'assurer que l'arborescence existe
            let _ = FileManager::create_beatmap_directory_structure(osu_id);

            // Recréer la chaîne .osu à partir du beatmap (après rate)
            let osu_string = rates_maker.osu_map.encode_to_string().unwrap();
            if let Ok(result) = CompressionManager::compress_string(&osu_string) {
                let _ = FileManager::save_compressed_file(osu_id, hash, &result.compressed_data);
            }
        }

        beatmap_row.rates.push(rates);
    }

    let elapsed = start_all.elapsed();
    info!(
        "process_beatmap done: osu_id={}, elapsed_ms={}",
        beatmap_row.osu_id.unwrap_or_default(),
        elapsed.as_millis()
    );
    Ok(())
}
