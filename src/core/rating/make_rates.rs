use minacalc_rs::Ssr;
use rosu_map::Beatmap as RmBeatmap;

pub struct RatesMaker {
    pub skillset_scores: Ssr,
    pub osu_map: RmBeatmap,
    pub rate: String,
    pub drain_time: f64,
    pub total_time: f64,
    pub bpm: f32,
}
