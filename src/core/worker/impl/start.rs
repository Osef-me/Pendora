use crate::core::beatmap::from::beatmap_from_beatmap_extended;
use crate::core::beatmapset::from::beatmapset_from_beatmapset_extended;
use crate::core::worker::process::process_beatmap;
use crate::core::worker::r#impl::insert::insert_full_beatmapset;
use crate::core::worker::types::BeatmapWorker;
use crate::errors::BeatmapWorkerError;
use crate::utils::{build_file_path, is_allowed_beatmap};
use anyhow::Result;
use db::models::beatmaps::pending_beatmap::PendingBeatmapRow;
use minacalc_rs::Calc;
use rosu_v2::prelude::{BeatmapExtended, BeatmapsetExtended};

impl BeatmapWorker {
    pub async fn start(&self) -> Result<(), BeatmapWorkerError> {
        tracing::info!("Beatmap worker started");


        // Lancer un seul worker séquentiel
        self.start_worker(0).await;

        Ok(())
    }

    /// Fonction dédiée pour chaque worker individuel
    async fn start_worker(&self, worker_id: usize) {
        tracing::info!("Worker {} started", worker_id);

        // Créer une instance locale de calculateur
        let calc = Calc::new().unwrap();

        loop {
            tracing::debug!("Worker {}: Checking for pending beatmaps...", worker_id);
            let pool = self.config.database.get_pool();
            let pending_beatmaps = PendingBeatmapRow::last_pending_beatmap(&pool)
                .await;

            let pending_beatmap = match pending_beatmaps {
                Ok(Some(beatmap)) => beatmap,
                Ok(None) => {
                    tracing::debug!("Worker {}: No pending beatmaps found, sleeping for 10 seconds", worker_id);
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    continue;
                }
                Err(e) => {
                    tracing::error!("Worker {}: Database error: {}", worker_id, e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }
            };

            if let Err(e) = PendingBeatmapRow::delete_by_id(&pool, pending_beatmap.id).await {
                tracing::error!("Worker {}: Failed to delete pending beatmap: {}", worker_id, e);
                continue;
            }

            tracing::debug!(
                "Worker {}: Fetching beatmap from osu! API for hash: {}",
                worker_id,
                pending_beatmap.osu_hash
            );

            let beatmap = match self
                .osu_api_service
                .beatmap_by_checksum(pending_beatmap.osu_hash.clone())
                .await
            {
                Ok(b) => b,
                Err(e) => {
                    tracing::error!("Worker {}: Failed to fetch beatmap by checksum: {}", worker_id, e);
                    continue;
                }
            };

            tracing::debug!(
                "Worker {}: Beatmap fetched: osu_id={}, mode={}, cs={}",
                worker_id,
                beatmap.map_id,
                beatmap.mode,
                beatmap.cs
            );

            if !is_allowed_beatmap(beatmap.mode, beatmap.cs).await {
                tracing::warn!(
                    "Worker {}: Beatmap not allowed: mode={}, cs={}",
                    worker_id,
                    beatmap.mode,
                    beatmap.cs
                );
                continue;
            }

            let Some(beatmapset) = &beatmap.mapset else {
                tracing::warn!("Worker {}: beatmap has no mapset, skipping", worker_id);
                continue;
            };

            tracing::debug!(
                "Worker {}: Beatmapset: id={}, artist={}, title={}",
                worker_id,
                beatmapset.mapset_id,
                beatmapset.artist,
                beatmapset.title
            );

            if let Err(e) = self.process_single_beatmap(&beatmap, beatmapset, worker_id, &calc).await {
                tracing::error!("Worker {}: Failed to process beatmap: {}", worker_id, e);
                continue;
            }

            tracing::info!(
                "Worker {}: Successfully processed and inserted beatmapset: osu_id={}",
                worker_id,
                beatmapset.mapset_id
            );
        }
    }

    /// Traite une seule beatmap avec son beatmapset
    async fn process_single_beatmap(
        &self,
        beatmap: &BeatmapExtended,
        beatmapset: &BeatmapsetExtended,
        worker_id: usize,
        calc: &Calc,
    ) -> Result<(), BeatmapWorkerError> {
        let mut beatmapset_row = beatmapset_from_beatmapset_extended(beatmapset);
        let mut beatmap_row = beatmap_from_beatmap_extended(beatmap);
        let Some(osu_id) = beatmap_row.osu_id else {
            return Err(BeatmapWorkerError::DatabaseError("beatmap has no osu_id".to_string()));
        };
        let osu_path = build_file_path(osu_id as u32);

        // Utiliser le calculateur local passé en paramètre
        let result = process_beatmap(beatmap, calc, osu_path, &mut beatmap_row).await;

        result?;

        beatmapset_row.beatmaps.push(beatmap_row);

        insert_full_beatmapset(self, &beatmapset_row).await?;

        Ok(())
    }
}
