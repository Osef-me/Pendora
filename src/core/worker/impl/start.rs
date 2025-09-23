use crate::core::beatmap::from::beatmap_from_beatmap_extended;
use crate::core::beatmapset::from::beatmapset_from_beatmapset_extended;
use crate::core::worker::process::process_beatmap;
use crate::core::worker::r#impl::insert::insert_full_beatmapset;
use crate::core::worker::types::BeatmapWorker;
use crate::errors::BeatmapWorkerError;
use crate::utils::{build_file_path, is_allowed_beatmap};
use anyhow::Result;
use db::models::beatmaps::pending_beatmap::PendingBeatmapRow;

impl BeatmapWorker {
    pub async fn start(&self) -> Result<(), BeatmapWorkerError> {
        tracing::info!("Beatmap worker started");

        let calc = minacalc_rs::Calc::new()
            .map_err(|e| BeatmapWorkerError::MinacalcError(e.to_string()))?;

        loop {
            tracing::debug!("Checking for pending beatmaps...");
            let pool = self.config.database.get_pool();
            let pending_beatmaps = PendingBeatmapRow::last_pending_beatmap(&pool)
                .await
                .map_err(|e| BeatmapWorkerError::DatabaseError(e.to_string()))?;

            let Some(pending_beatmap) = pending_beatmaps else {
                tracing::debug!("No pending beatmaps found, sleeping for 10 seconds");
                std::thread::sleep(std::time::Duration::from_secs(10));
                continue;
            };

            tracing::info!(
                "Processing pending beatmap: id={}, hash={}",
                pending_beatmap.id,
                pending_beatmap.osu_hash
            );
            PendingBeatmapRow::delete_by_id(&pool, pending_beatmap.id)
                .await
                .unwrap();

            tracing::debug!(
                "Fetching beatmap from osu! API for hash: {}",
                pending_beatmap.osu_hash
            );
            let beatmap = self
                .osu_api_service
                .beatmap_by_checksum(pending_beatmap.osu_hash.clone())
                .await
                .unwrap();

            tracing::debug!(
                "Beatmap fetched: osu_id={}, mode={}, cs={}",
                beatmap.map_id,
                beatmap.mode,
                beatmap.cs
            );
            if !is_allowed_beatmap(beatmap.mode, beatmap.cs).await {
                tracing::warn!(
                    "Beatmap not allowed: mode={}, cs={}",
                    beatmap.mode,
                    beatmap.cs
                );
                continue;
            }

            let beatmapset = beatmap.mapset.clone().unwrap();
            tracing::debug!(
                "Beatmapset: id={}, artist={}, title={}",
                beatmapset.mapset_id,
                beatmapset.artist,
                beatmapset.title
            );

            let mut beatmapset_row = beatmapset_from_beatmapset_extended(&beatmapset);
            let mut beatmap_row = beatmap_from_beatmap_extended(&beatmap);
            let osu_path = build_file_path(beatmap_row.osu_id.clone().unwrap() as u32);

            tracing::info!(
                "Processing beatmap: osu_id={}, difficulty={}",
                beatmap_row.osu_id.unwrap(),
                beatmap_row.difficulty
            );
            process_beatmap(&beatmap, &calc, osu_path, &mut beatmap_row)
                .await
                .unwrap();

            beatmapset_row.beatmaps.push(beatmap_row);
            tracing::info!(
                "Inserting beatmapset into database: osu_id={}",
                beatmapset_row.osu_id.unwrap()
            );
            insert_full_beatmapset(&self, &beatmapset_row)
                .await
                .unwrap();
            tracing::info!(
                "Successfully processed and inserted beatmapset: osu_id={}",
                beatmapset_row.osu_id.unwrap()
            );
        }
    }
}
