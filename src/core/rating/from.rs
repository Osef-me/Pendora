use crate::core::rating::make_rates::RatesMaker;
use crate::core::rating::proportion::Proportion;
use crate::utils::calculator::get_star_rating;
use crate::utils::calculator::get_sunnyxxy_rating;
use anyhow::Result;
use dto::models::rate::{ManiaRating, ModeRating, Rates, Rating};
use tracing::debug;

pub async fn rates_from_skillset_scores(
    make_rates: &mut RatesMaker,
    hash: String,
) -> Result<Rates> {


    let rate_data = calculate_rate_data(make_rates);
    let proportions = calculate_proportions(make_rates);
    let osu_hash = hash;

    let ratings = create_all_ratings(make_rates, &proportions, &make_rates.osu_map);

    let rates = Rates {
        id: None,
        osu_hash: Some(osu_hash),
        centirate: rate_data.centirate,
        drain_time: rate_data.drain_time,
        total_time: rate_data.total_time,
        bpm: rate_data.bpm,
        rating: ratings,
    };

    debug!(
        "Rates created successfully: centirate={}, drain_time={}, total_time={}, bpm={:.1}",
        rates.centirate, rates.drain_time, rates.total_time, rates.bpm
    );
    Ok(rates)
}

#[derive(Debug)]
struct RateData {
    centirate: i32,
    drain_time: i32,
    total_time: i32,
    bpm: f32,
}

fn calculate_rate_data(make_rates: &RatesMaker) -> RateData {
    let centirate = make_rates.centirate as f64;
    let rate = centirate / 100.0;
    let proportion_rate = 100.0 / centirate;
    let drain_time = make_rates.drain_time * proportion_rate;
    let total_time = make_rates.total_time * proportion_rate;
    let bpm = make_rates.bpm as f64 * rate;

    RateData {
        centirate: make_rates.centirate,
        drain_time: drain_time as i32,
        total_time: total_time as i32,
        bpm: bpm as f32,
    }
}

fn calculate_proportions(make_rates: &RatesMaker) -> Proportion {
    let overall = make_rates.skillset_scores.overall as f64;
    debug!(
        "Skillset scores - overall: {:.2}, stream: {:.2}, jumpstream: {:.2}, stamina: {:.2}",
        overall,
        make_rates.skillset_scores.stream,
        make_rates.skillset_scores.jumpstream,
        make_rates.skillset_scores.stamina
    );

    Proportion {
        stream: make_rates.skillset_scores.stream as f64 / overall,
        jumpstream: make_rates.skillset_scores.jumpstream as f64 / overall,
        handstream: make_rates.skillset_scores.handstream as f64 / overall,
        stamina: make_rates.skillset_scores.stamina as f64 / overall,
        jackspeed: make_rates.skillset_scores.jackspeed as f64 / overall,
        chordjack: make_rates.skillset_scores.chordjack as f64 / overall,
        technical: make_rates.skillset_scores.technical as f64 / overall,
    }
}



// removed unused generate_beatmap_hash

fn create_all_ratings(
    make_rates: &RatesMaker,
    proportions: &Proportion,
    osu_map: &str,
) -> Vec<Rating> {
    let etterna_rating = create_etterna_rating(make_rates);
    let osu_rating = create_osu_rating(osu_map, proportions, make_rates.centirate as i64);
    let sunny_rating = create_sunny_rating(osu_map, proportions, make_rates.centirate as i64);

    vec![etterna_rating, sunny_rating, osu_rating]
}

fn create_etterna_rating(make_rates: &RatesMaker) -> Rating {
    Rating {
        id: None,
        rates_id: None,
        rating: make_rates.skillset_scores.overall as f64,
        rating_type: "etterna".to_string(),
        mode_rating: ModeRating::Mania(ManiaRating {
            id: None,
            stream: make_rates.skillset_scores.stream as f64,
            jumpstream: make_rates.skillset_scores.jumpstream as f64,
            handstream: make_rates.skillset_scores.handstream as f64,
            stamina: make_rates.skillset_scores.stamina as f64,
            jackspeed: make_rates.skillset_scores.jackspeed as f64,
            chordjack: make_rates.skillset_scores.chordjack as f64,
            technical: make_rates.skillset_scores.technical as f64,
        }),
    }
}

fn create_osu_rating(osu_map: &str, proportions: &Proportion, centirate: i64) -> Rating {
    debug!("Calculating star rating...");
    let stars = get_star_rating(osu_map, centirate);

    rating_new("osu".to_string(), stars, proportions.clone())
}

fn create_sunny_rating(osu_map: &str, proportions: &Proportion, centirate: i64) -> Rating {
    debug!("Calculating sunnyxxy rating...");
    let sunny_rating_value = get_sunnyxxy_rating(osu_map, centirate);

    rating_new(
        "sunnyxxy".to_string(),
        sunny_rating_value,
        proportions.clone(),
    )
}

pub fn rating_new(rating_type: String, rating: f64, proportion: Proportion) -> Rating {
    Rating {
        id: None,
        rates_id: None,
        rating,
        rating_type,
        mode_rating: ModeRating::Mania(ManiaRating {
            id: None,
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
