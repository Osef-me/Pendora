use minacalc_rs::Ssr;
// Keep only Ssr; osu_map is now a String

pub struct RatesMaker {
    pub skillset_scores: Ssr,
    pub osu_map: String,
    pub centirate: i32,
    pub drain_time: f64,
    pub total_time: f64,
    pub bpm: f32,
}
