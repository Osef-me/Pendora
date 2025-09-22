use crate::core::rating::make_rates::RatesMaker;
use crate::core::rating::proportion::Proportion;
use dto::models::beatmaps::full::types::{ManiaRating, ModeRating, Rates, Rating};
use crate::utils::calculator::get_star_rating;
use crate::utils::rate::hash::hash_md5;
use crate::utils::calculator::get_sunnyxxy_rating;
use anyhow::Result;

pub async fn rates_from_skillset_scores(make_rates: &mut RatesMaker) -> Result<Rates> {
        let rate = make_rates.rate.parse::<f64>().unwrap();
        let centirate = rate * 100.0;
        let proportion_rate = centirate / 100.0;
        let drain_time = make_rates.drain_time * proportion_rate;
        let total_time = make_rates.total_time * proportion_rate;
        let bpm = make_rates.bpm as f64 * rate;
        let overall = make_rates.skillset_scores.overall as f64;
        let stream_proportion = make_rates.skillset_scores.stream as f64 / overall;
        let jumpstream_proportion = make_rates.skillset_scores.jumpstream as f64 / overall;
        let handstream_proportion = make_rates.skillset_scores.handstream as f64 / overall;
        let stamina_proportion = make_rates.skillset_scores.stamina as f64 / overall;
        let jackspeed_proportion = make_rates.skillset_scores.jackspeed as f64 / overall;
        let chordjack_proportion = make_rates.skillset_scores.chordjack as f64 / overall;
        let technical_proportion = make_rates.skillset_scores.technical as f64 / overall;

        let osu_map: String = make_rates.osu_map.encode_to_string().unwrap();
        let osu_hash = hash_md5(&osu_map).unwrap();

        let stars = get_star_rating(&osu_map);
        // For etterna rating, we use direct values from Ssr
        let etterna_rating = Rating {
            rates_id: None,
            rating: make_rates.skillset_scores.overall as f64,
            rating_type: "etterna".to_string(),
            mode_rating: ModeRating::Mania(ManiaRating {
                stream: make_rates.skillset_scores.stream as f64,
                jumpstream: make_rates.skillset_scores.jumpstream as f64,
                handstream: make_rates.skillset_scores.handstream as f64,
                stamina: make_rates.skillset_scores.stamina as f64,
                jackspeed: make_rates.skillset_scores.jackspeed as f64,
                chordjack: make_rates.skillset_scores.chordjack as f64,
                technical: make_rates.skillset_scores.technical as f64,
            }),
        };

        // Create proportion for osu rating (calculated proportions)
        let osu_proportion = Proportion {
            stream: stream_proportion,
            jumpstream: jumpstream_proportion,
            handstream: handstream_proportion,
            stamina: stamina_proportion,
            jackspeed: jackspeed_proportion,
            chordjack: chordjack_proportion,
            technical: technical_proportion,
        };

        let sunny_rating = rating_new("sunnyxxy".to_string(), get_sunnyxxy_rating(&osu_map), osu_proportion.clone());
        let osu_rating = rating_new("osu".to_string(), stars, osu_proportion);
        let rates = Rates {
            osu_hash: Some(osu_hash),
            centirate: centirate as i32,
            drain_time: drain_time as i32,
            total_time: total_time as i32,
            bpm: bpm as f32,
            rating: vec![
                etterna_rating,
                sunny_rating,
                osu_rating,
            ],
        };
        Ok(rates)
    }

pub fn rating_new(rating_type: String, rating: f64, proportion: Proportion) -> Rating {
        Rating {
            rates_id: None,
            rating,
            rating_type,
            mode_rating: ModeRating::Mania(ManiaRating {
                stream: rating * proportion.stream,
                jumpstream: rating * proportion.jumpstream,
                handstream: rating * proportion.handstream,
                stamina: rating * proportion.stamina,
                jackspeed: rating * proportion.jackspeed,
                chordjack: rating * proportion.chordjack,
                technical: rating * proportion.technical,
            }),
        }
    }
