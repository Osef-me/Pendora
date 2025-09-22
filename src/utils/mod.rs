pub mod calculator;
pub mod rate;
use anyhow::Result;
use reqwest;
use rosu_v2::prelude::GameMode;
use rosu_v2::prelude::RankStatus;

pub fn rank_status_to_string(status: &RankStatus) -> String {
    match status {
        RankStatus::Pending => "pending".to_string(),
        RankStatus::Ranked => "ranked".to_string(),
        RankStatus::Approved => "approved".to_string(),
        RankStatus::Qualified => "qualified".to_string(),
        RankStatus::Loved => "loved".to_string(),
        RankStatus::Graveyard => "graveyard".to_string(),
        RankStatus::WIP => "wip".to_string(),
    }
}

pub fn build_file_path(beatmap_id: u32) -> String {
    format!("https://osu.ppy.sh/osu/{}", beatmap_id)
}

pub async fn osu_file_from_url(path_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(path_url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn is_allowed_beatmap(mode: GameMode, cs: f32) -> bool {
    if mode != GameMode::Mania {
        return false;
    }

    if cs != 4.0 {
        return false;
    }

    true
}
