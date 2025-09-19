use crate::utils::rank_status_to_string;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use db::models::beatmaps::beatmap::types::BeatmapRow;
use rosu_v2::prelude::BeatmapExtended;
use crate::convert::Rates;
#[derive(Debug, Clone)]
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

pub fn convert_beatmap_extended(b: &BeatmapExtended) -> Beatmap {
    Beatmap {
        osu_id: Some(b.map_id.clone() as i32),
        beatmapset_id: None,
        difficulty: b.version.clone(),
        count_circles: b.count_circles.clone() as i32,
        count_sliders: b.count_sliders.clone() as i32,
        count_spinners: b.count_spinners.clone() as i32,
        max_combo: b.max_combo.unwrap_or(0) as i32,
        cs: b.cs.clone() as f64,
        ar: b.ar.clone() as f64,
        od: b.od.clone() as f64,
        hp: b.hp.clone() as f64,
        mode: b.mode.clone() as i32,
        status: rank_status_to_string(&b.status.clone()),
        main_pattern: serde_json::to_value("[]").unwrap(),
        rates: Vec::new(),
    }
}
