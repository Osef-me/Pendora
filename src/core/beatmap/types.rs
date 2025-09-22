use crate::core::rating::types::Rates;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Beatmap {
    pub osu_id: Option<i32>,
    pub beatmapset_id: Option<i32>,
    pub difficulty: String,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub max_combo: i32,
    pub cs: f64,
    pub ar: f64,
    pub od: f64,
    pub hp: f64,
    pub mode: i32,
    pub status: String,
    pub main_pattern: serde_json::Value,
    pub rates: Vec<Rates>,
}
