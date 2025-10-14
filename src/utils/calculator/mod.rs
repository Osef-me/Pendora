use ssrrr::algorithm::process::process::calculate;
use ssrrr::preprocess;
use std::str::FromStr;

// removed unused get_quaver_rating

pub fn get_sunnyxxy_rating(osu_map: &str, centirate: i64) -> f64 {
    let preprocess = preprocess(osu_map, "None", centirate).unwrap();
    let b = calculate(&preprocess).unwrap();
    b.rating
}

pub fn get_star_rating(osu_map: &str, centirate: i64) -> f64 {
    let map = rosu_pp::Beatmap::from_str(osu_map).unwrap();
    let diff_attrs = rosu_pp::Difficulty::new().clock_rate(centirate as f64 / 100.0).calculate(&map);
    diff_attrs.stars()
}
