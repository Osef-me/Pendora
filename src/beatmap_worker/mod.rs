use crate::api::osu::OsuApiService;
use crate::config::Config;
use crate::convert::beatmap::convert_beatmap_extended;
use crate::convert::beatmapset::convert_to_row_beatmapset;
use crate::errors::BeatmapWorkerError;
use crate::utils::{build_file_path, osu_file_from_url};
use anyhow::Result;
use db::models::beatmaps::pending_beatmap::PendingBeatmapRow;
use std::str::FromStr;
use rosu_map::Beatmap;
use minacalc_rs::{hashmap::HashMapCalcExt, osu::OsuCalcExt, Calc};

pub(crate) async fn start(
    config: &Config,
    osu_api_service: OsuApiService,
) -> Result<(), BeatmapWorkerError> {
    tracing::info!("Beatmap worker started");

    let calc = Calc::new().map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?;
    loop {
        let pending = PendingBeatmapRow::last_pending_beatmap(config.database.get_pool())
            .await
            .unwrap();
        if pending.is_none() {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            continue;
        }
        let pending = pending.unwrap();
        let beatmap = osu_api_service
            .beatmap_by_checksum(pending.osu_hash.clone())
            .await
            .unwrap();
        let beatmapset = beatmap.mapset.clone().unwrap();
        let mut beatmapset_row = convert_to_row_beatmapset(&beatmapset);
        let mut beatmap_row = convert_beatmap_extended(beatmap);

        let osu_path = build_file_path(beatmap_row.osu_id.clone().unwrap() as u32);
        process_beatmap(config, &calc, osu_path).await.unwrap();
    }
    Ok(())
}

pub(crate) async fn process_beatmap(
    config: &Config,
    calc: &Calc,
    osu_path: String,
) -> Result<(), BeatmapWorkerError> {
    let osu_map = osu_file_from_url(&osu_path).await.unwrap();
    let skillset_scores = calc.calculate_msd_from_string(osu_map).map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?
        .as_hashmap().map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?;
    Ok(())
}
