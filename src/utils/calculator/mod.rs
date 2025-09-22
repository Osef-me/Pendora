use std::str::FromStr;
use ssrrr::preprocess;
use ssrrr::algorithm::process::process::calculate;

pub fn get_quaver_rating(osu_map: &str) -> f64 {
    0.0
}

pub fn get_sunnyxxy_rating(osu_map: &str) -> f64 {
    let preprocess = preprocess(osu_map, "None").unwrap();
    let b = calculate(&preprocess).unwrap();
    b.rating
}

pub fn get_star_rating(osu_map: &str) -> f64 {
    let map = rosu_pp::Beatmap::from_str(osu_map).unwrap();
    let diff_attrs = rosu_pp::Difficulty::new().calculate(&map);
    diff_attrs.stars()
}
