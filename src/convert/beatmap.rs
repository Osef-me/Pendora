use crate::utils::rank_status_to_string;
use bigdecimal::BigDecimal;
use bigdecimal::FromPrimitive;
use db::models::beatmaps::beatmap::types::BeatmapRow;
use rosu_v2::prelude::BeatmapExtended;

pub fn convert_beatmap_extended(b: BeatmapExtended) -> BeatmapRow {
    BeatmapRow {
        id: 0,
        osu_id: Some(b.map_id as i32),
        beatmapset_id: None,
        difficulty: b.version,
        count_circles: b.count_circles as i32,
        count_sliders: b.count_sliders as i32,
        count_spinners: b.count_spinners as i32,
        max_combo: b.max_combo.unwrap_or(0) as i32,
        cs: BigDecimal::from_f32(b.cs).unwrap(),
        ar: BigDecimal::from_f32(b.ar).unwrap(),
        od: BigDecimal::from_f32(b.od).unwrap(),
        hp: BigDecimal::from_f32(b.hp).unwrap(),
        mode: b.mode as i32,
        status: rank_status_to_string(&b.status),
        main_pattern: serde_json::to_value("[]").unwrap(),
        created_at: None,
        updated_at: None,
    }
}
