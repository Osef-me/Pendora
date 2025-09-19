use db::models::beatmaps::pending_beatmap::PendingBeatmapRow;
use minacalc_rs::{hashmap::HashMapCalcExt, osu::OsuCalcExt, Calc};
use rosu_map::Beatmap as RmBeatmap;
use rosu_v2::prelude::BeatmapExtended;
use crate::convert::beatmap::Beatmap;
use std::str::FromStr;
use crate::utils::osu_file_from_url;
use crate::utils::rate::rate::bulk_rate;
use crate::convert::Rates;
use crate::errors::BeatmapWorkerError;
use crate::config::Config;

pub(crate) async fn process_beatmap(
    config: &Config,
    beatmap: &BeatmapExtended,
    calc: &Calc,
    osu_path: String,
    beatmap_row: &mut Beatmap,
) -> Result<(), BeatmapWorkerError> {
    let osu_map = osu_file_from_url(&osu_path).await.unwrap();
    let parsed_beatmap = RmBeatmap::from_str(&osu_map).unwrap();
    let rates = vec![0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0]; 

    
    let hash = bulk_rate(&rates, parsed_beatmap);

    let skillset_scores = calc
        .calculate_msd_from_string(osu_map)
        .map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?
        .as_hashmap()
        .map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?;

    for (key, value) in skillset_scores {
        let rates: Rates = Rates::from_skillset_scores(value, &osu_map, key, beatmap.seconds_drain as f64, beatmap.seconds_total as f64, beatmap.bpm as f32)
            .await
            .unwrap();
        beatmap_row.rates.push(rates);
    }
    Ok(())
}
