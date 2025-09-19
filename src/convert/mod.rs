pub mod beatmap;
pub mod beatmapset;
use anyhow::Result;
use minacalc_rs::Ssr;
use rosu_v2::prelude::BeatmapExtended;
use rosu_map::Beatmap;
use crate::utils::rate::rate::rate;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Rates {
    pub osu_hash: Option<String>,
    pub centirate: i32,
    pub drain_time: i32,
    pub total_time: i32,
    pub bpm: f32,
    pub rating: Vec<Rating>,
}


impl Rates {
    pub async fn from_skillset_scores(
        skillset_scores: Ssr,
        osu_map: &str,
        key: String,
        drain_time: f64,
        total_time: f64,
        bpm: f32,
    ) -> Result<Rates> {



        let rate = key.parse::<f64>().unwrap();
        let centirate = rate * 100.0;
        let drain_time = drain_time as f64 * rate;
        let total_time = total_time as f64 * rate;
        let bpm = bpm as f64 * rate;
        let overall = skillset_scores.overall as f64;
        let stream_proportion = skillset_scores.stream as f64 / overall;
        let jumpstream_proportion =
            skillset_scores.jumpstream as f64 / overall;
        let handstream_proportion =
            skillset_scores.handstream as f64 / overall;
        let stamina_proportion = skillset_scores.stamina as f64 / overall;
        let jackspeed_proportion =
            skillset_scores.jackspeed as f64 / overall;
        let chordjack_proportion =
            skillset_scores.chordjack as f64 / overall;
        let technical_proportion =
            skillset_scores.technical as f64 / overall;
        let stars = beatmap.stars.clone() as f64;

        let rates = Rates {
            osu_hash: beatmap.checksum.clone(),
            centirate: centirate as i32,
            drain_time:  drain_time as i32,
            total_time: total_time as i32,
            bpm: bpm as f32,
            rating: vec![
                Rating {
                    rates_id: None,
                    rating: skillset_scores.overall as f64,
                    rating_type: "etterna".to_string(),
                    mania_rating: ManiaRating {
                        stream: skillset_scores.stream as f64,
                        jumpstream: skillset_scores.jumpstream as f64,
                        handstream: skillset_scores.handstream as f64,
                        stamina: skillset_scores.stamina as f64,
                        jackspeed: skillset_scores.jackspeed as f64,
                        chordjack: skillset_scores.chordjack as f64,
                        technical: skillset_scores.technical as f64,
                    },
                },
                Rating {
                    rates_id: None,
                    rating: beatmap.stars.clone() as f64,
                    rating_type: "osu".to_string(),
                    mania_rating: ManiaRating {
                        stream: stars as f64 * stream_proportion as f64,
                        jumpstream: stars as f64 * jumpstream_proportion as f64,
                        handstream: stars as f64 * handstream_proportion as f64,
                        stamina: stars as f64 * stamina_proportion as f64,
                        jackspeed: stars as f64 * jackspeed_proportion as f64,
                        chordjack: stars as f64 * chordjack_proportion as f64,
                        technical: stars as f64 * technical_proportion as f64,
                    },
                },
            ],
        };
        Ok(rates)
    }
}

#[derive(Debug, Clone)]
pub struct Rating {
    pub rates_id: Option<i32>,
    pub rating: f64,
    pub rating_type: String,
    pub mania_rating: ManiaRating,
}

#[derive(Debug, Clone)]
pub struct ManiaRating {
    pub stream: f64,
    pub jumpstream: f64,
    pub handstream: f64,
    pub stamina: f64,
    pub jackspeed: f64,
    pub chordjack: f64,
    pub technical: f64,
}
