#[derive(Debug, Clone, serde::Serialize)]
pub struct Rates {
    pub osu_hash: Option<String>,
    pub centirate: i32,
    pub drain_time: i32,
    pub total_time: i32,
    pub bpm: f32,
    pub rating: Vec<Rating>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Rating {
    pub rates_id: Option<i32>,
    pub rating: f64,
    pub rating_type: String,
    pub mania_rating: ManiaRating,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ManiaRating {
    pub stream: f64,
    pub jumpstream: f64,
    pub handstream: f64,
    pub stamina: f64,
    pub jackspeed: f64,
    pub chordjack: f64,
    pub technical: f64,
}
