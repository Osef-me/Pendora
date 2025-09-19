pub mod beatmap;
pub mod beatmapset;
use anyhow::Result;
use rosu_v2::prelude::BeatmapExtended;

pub struct Rates{
    pub osu_hash: Option<String>,
    pub centirate: i32,
    pub drain_time: i32,
    pub total_time: i32,
    pub bpm: f32,
}

impl Rates {
    pub async fn from_beatmap(beatmap: BeatmapExtended) -> Result<Rates> {
        let rates = Rates {
            osu_hash: beatmap.checksum.clone(),
            centirate: 100,
            drain_time: beatmap.seconds_drain as i32,
            total_time: beatmap.seconds_total as i32,
            bpm: beatmap.bpm,
        };
        Ok(rates)
    }
}


pub struct Rating {
    pub rates_id: Option<i32>,
    pub rating: f64,
    pub rating_type: String,
}

