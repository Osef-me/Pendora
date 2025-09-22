use std::str::FromStr;

pub fn get_quaver_rating(osu_map: &str) -> f64 {
    0.0
}

pub fn get_sunnyxxy_rating(osu_map: &str) -> f64 {
    0.0
}

pub fn get_star_rating(osu_map: &str) -> f64 {
    let map = rosu_pp::Beatmap::from_str(osu_map).unwrap();
    let diff_attrs = rosu_pp::Difficulty::new().calculate(&map);
    diff_attrs.stars()
}
