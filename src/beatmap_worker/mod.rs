use crate::api::osu::OsuApiService;
use crate::config::Config;
use crate::convert::beatmap::convert_beatmap_extended;
use crate::convert::beatmapset::convert_to_row_beatmapset;
use crate::errors::BeatmapWorkerError;
use crate::utils::{build_file_path, is_allowed_beatmap};
use anyhow::Result;
pub mod process;
use crate::beatmap_worker::process::process_beatmap;

pub(crate) async fn start(
    config: &Config,
    osu_api_service: OsuApiService,
    hash: String, // testing purpose
) -> Result<(), BeatmapWorkerError> {
    tracing::info!("Beatmap worker started");

    let calc = minacalc_rs::Calc::new().map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?;
    loop {
        /* 
        let pending = PendingBeatmapRow::last_pending_beatmap(config.database.get_pool())
            .await
            .unwrap();
        if pending.is_none() {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            continue;
        }
        */
        let pending = hash.clone();
        let beatmap = osu_api_service
            .beatmap_by_checksum(pending.clone())
            .await
            .unwrap();
        if !is_allowed_beatmap(beatmap.mode, beatmap.cs).await {
            continue;
        }
        let beatmapset = beatmap.mapset.clone().unwrap();
        let mut beatmapset_row = convert_to_row_beatmapset(&beatmapset);
        let mut beatmap_row = convert_beatmap_extended(&beatmap);
        let osu_path = build_file_path(beatmap_row.osu_id.clone().unwrap() as u32);
        process_beatmap(config, &beatmap, &calc, osu_path, &mut beatmap_row)
            .await
            .unwrap();

        beatmapset_row.beatmaps.push(beatmap_row);
        println! ("{:?}", beatmapset_row);
        break;
    }
    println!("BeatmapWorker finished");
    Ok(())
}

