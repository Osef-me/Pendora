use crate::core::rating::from::rates_from_skillset_scores;
use crate::core::rating::make_rates::RatesMaker;
use crate::errors::BeatmapWorkerError;
use crate::utils::determine_main_pattern;
use crate::utils::osu_file_from_url;
use crate::utils::rate::beatmap_processor::BeatmapProcessor;
use crate::utils::rate::compression::CompressionManager;
use crate::utils::rate::file_manager::FileManager;
use dto::models::beatmaps::full::types::Beatmap;
use dto::models::rate::Rates;
use minacalc_rs::{hashmap::HashMapCalcExt, osu::OsuCalcExt, Calc, Ssr};
use rosu_map::Beatmap as RmBeatmap;
use rosu_v2::prelude::BeatmapExtended;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Instant;
use tracing::{debug, info, warn};
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
    debug!("Starting beatmap processing for osu_id: {}", beatmap.map_id);

    debug!("Fetching osu file from URL: {}", osu_path);
    let osu_map = match osu_file_from_url(&osu_path).await {
        Ok(map) => {
            debug!("Osu file fetched, length: {} bytes", map.len());
            map
        }
        Err(err) => {
            warn!("Failed to fetch osu file from URL {}: {:?}. Skipping this beatmap.", osu_path, err);
            return Ok(()); // Skip this beatmap and continue processing others
        }
    };

    let parsed_beatmap = match RmBeatmap::from_str(&osu_map) {
        Ok(map) => {
            debug!("Beatmap parsed successfully");
            map
        }
        Err(err) => {
            warn!("Failed to parse beatmap from osu file: {:?}. Skipping this beatmap.", err);
            return Ok(()); // Skip this beatmap and continue processing others
        }
    };

    let rates = vec![
        0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0,
    ];
    debug!("Processing {} rates: {:?}", rates.len(), rates);

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
    // Créer une hash map unifiée contenant beatmaps + scores
    let mut beatmaps_with_scores: HashMap<String, BeatmapWithScores> = HashMap::new();

    for rate in rates {
        let mut processed_beatmap = parsed_beatmap.clone();
        apply_rate(rate, &mut processed_beatmap);
        // Les clés de skillset_scores sont au format "1.7", "1.0", etc. (une décimale)
        let rate_string = format!("{:.1}", rate);

        // Récupérer les scores correspondants
        if let Some(scores) = skillset_scores.get(&rate_string) {
            debug!(
                "Processing rate {} with scores: overall={:.2}, stream={:.2}, jumpstream={:.2}",
                rate_string, scores.overall, scores.stream, scores.jumpstream
            );
            beatmaps_with_scores.insert(
                rate_string.clone(),
                BeatmapWithScores {
                    beatmap: processed_beatmap,
                    skillset_scores: scores.clone(),
                },
            );
        } else {
            warn!("No skillset scores found for rate: {}", rate_string);
        }
    }

    // Traiter chaque entrée unifiée
    debug!(
        "Processing {} beatmaps with scores",
        beatmaps_with_scores.len()
    );
    for (rate_key, beatmap_with_scores) in beatmaps_with_scores {
        debug!("Creating rates for rate: {}", rate_key);
        let mut rates_maker = RatesMaker {
            skillset_scores: beatmap_with_scores.skillset_scores,
            osu_map: beatmap_with_scores.beatmap,
            rate: rate_key.clone(),
            drain_time: beatmap.seconds_drain as f64,
            total_time: beatmap.seconds_total as f64,
            bpm: beatmap.bpm as f32,
        };
        let rates: Rates = match rates_from_skillset_scores(&mut rates_maker).await {
            Ok(rates) => rates,
            Err(err) => {
                warn!("Failed to create rates for beatmap {}: {:?}. Skipping this rate.", beatmap.map_id, err);
                continue; // Skip this rate and continue with the next one
            }
        };
        debug!(
            "Rates created for {}: centirate={}, hash={}",
            rate_key,
            rates.centirate,
            rates.osu_hash.as_deref().unwrap_or("none")
        );

        // Sauvegarder le beatmap compressé en Brotli, nommé par le hash calculé
        if let (Some(osu_id), Some(hash)) = (beatmap_row.osu_id, &rates.osu_hash) {
            debug!(
                "Saving compressed beatmap for osu_id={}, hash={}",
                osu_id, hash
            );
            // S'assurer que l'arborescence existe
            let _ = FileManager::create_beatmap_directory_structure(osu_id);

            // Recréer la chaîne .osu à partir du beatmap (après rate)
            let osu_string = match rates_maker.osu_map.encode_to_string() {
                Ok(s) => s,
                Err(err) => {
                    warn!("Failed to encode beatmap to string: {:?}. Skipping compression.", err);
                    continue; // Skip compression for this rate
                }
            };
            if let Ok(result) = CompressionManager::compress_string(&osu_string) {
                let _ = FileManager::save_compressed_file(osu_id, hash, &result.compressed_data);
                debug!(
                    "Compressed file saved: {} bytes",
                    result.compressed_data.len()
                );
            } else {
                warn!("Failed to compress beatmap for rate: {}", rate_key);
            }
        } else {
            warn!("Missing osu_id or hash for rate: {}", rate_key);
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
