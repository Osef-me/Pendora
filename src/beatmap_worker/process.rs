use minacalc_rs::{hashmap::HashMapCalcExt, osu::OsuCalcExt, Calc, Ssr};
use rosu_map::Beatmap as RmBeatmap;
use rosu_v2::prelude::BeatmapExtended;
use crate::convert::beatmap::Beatmap;
use std::str::FromStr;
use crate::utils::osu_file_from_url;
use crate::convert::Rates;
use crate::errors::BeatmapWorkerError;
use crate::utils::rate::beatmap_processor::BeatmapProcessor;
use std::collections::HashMap;

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
    let osu_map = osu_file_from_url(&osu_path).await.unwrap();
    let parsed_beatmap = RmBeatmap::from_str(&osu_map).unwrap();
    let rates = vec![0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0]; 

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
        let rate_string = format!("{:.2}", rate);
        
        // Récupérer les scores correspondants
        if let Some(scores) = skillset_scores.get(&rate_string) {
            beatmaps_with_scores.insert(rate_string.clone(), BeatmapWithScores {
                beatmap: processed_beatmap,
                skillset_scores: scores.clone(),
            });
        }
    }

    // Traiter chaque entrée unifiée
    for (rate_key, beatmap_with_scores) in beatmaps_with_scores {
        let rates: Rates = Rates::from_skillset_scores(
            beatmap_with_scores.skillset_scores, 
            &osu_map, 
            rate_key, 
            beatmap.seconds_drain as f64, 
            beatmap.seconds_total as f64, 
            beatmap.bpm as f32
        )
        .await
        .unwrap();
        beatmap_row.rates.push(rates);
    }
    
    Ok(())
}
