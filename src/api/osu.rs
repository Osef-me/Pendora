use anyhow::Result;
use rosu_v2::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct OsuApiService {
    client: Arc<Osu>,
}

impl OsuApiService {
    pub async fn new(client_id: String, client_secret: String) -> Result<Self> {
        let client = Arc::new(Osu::new(client_id.parse::<u64>().unwrap(), client_secret).await?);

        Ok(Self { client })
    }

    // checksum = hash of the beatmap file
    pub async fn beatmap_by_checksum(&self, checksum: String) -> Result<BeatmapExtended> {
        let beatmap = self.client.beatmap().checksum(checksum).await?;
        Ok(beatmap)
    }

    pub async fn beatmap_by_osu_id(&self, osu_id: i32) -> Result<BeatmapExtended> {
        let beatmap = self.client.beatmap().map_id(osu_id as u32).await?;
        Ok(beatmap)
    }
}
