use crate::core::beatmap::types::Beatmap;
use crate::core::beatmapset::types::Beatmapset;
use crate::core::worker::process::process_beatmap;
use crate::core::worker::types::BeatmapWorker;
use crate::errors::BeatmapWorkerError;
use crate::utils::{build_file_path, is_allowed_beatmap};
use anyhow::Result;

impl BeatmapWorker {
    pub async fn start(&self, hash: String) -> Result<(), BeatmapWorkerError> {
        tracing::info!("Beatmap worker started");

        let calc = minacalc_rs::Calc::new()
            .map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?;

        loop {
            let beatmap = self
                .osu_api_service
                .beatmap_by_checksum(hash.clone())
                .await
                .unwrap();

            if !is_allowed_beatmap(beatmap.mode, beatmap.cs).await {
                continue;
            }

            let beatmapset = beatmap.mapset.clone().unwrap();
            let mut beatmapset_row = Beatmapset::from_beatmapset_extended(&beatmapset);
            let mut beatmap_row = Beatmap::from_beatmap_extended(&beatmap);
            let osu_path = build_file_path(beatmap_row.osu_id.clone().unwrap() as u32);

            // TODO: Implement process_beatmap function
            process_beatmap(&beatmap, &calc, osu_path, &mut beatmap_row)
                .await
                .unwrap();

            beatmapset_row.beatmaps.push(beatmap_row);
            match serde_json::to_string_pretty(&beatmapset_row) {
                Ok(json) => println!("done"),
                Err(e) => eprintln!("Failed to serialize beatmapset_row to JSON: {}", e),
            }
            break;
        }

        println!("BeatmapWorker finished");
        Ok(())
    }
}
