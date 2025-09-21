use minacalc_rs::Ssr;

pub struct RatesMaker<'a> {
    pub skillset_scores: Ssr,
    pub osu_map: &'a str,
    pub rate: String,
    pub drain_time: f64,
    pub total_time: f64,
    pub bpm: f32,

    pub osu_rating: f64,
    pub quaver_rating: f64,
    pub sunnyxxy_rating: f64,
}
