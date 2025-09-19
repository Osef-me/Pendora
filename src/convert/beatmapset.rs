use db::models::beatmaps::beatmapset::types::BeatmapsetRow;
use rosu_v2::prelude::BeatmapsetExtended;
use crate::convert::beatmap::Beatmap;

#[derive(Debug, Clone)]
pub struct Beatmapset {
    pub osu_id: Option<i32>,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub title: String,
    pub title_unicode: Option<String>,
    pub creator: String,
    pub source: Option<String>,
    pub tags: Option<String>,
    pub has_video: bool,
    pub has_storyboard: bool,
    pub is_explicit: bool,
    pub is_featured: bool,
    pub cover_url: Option<String>,
    pub preview_url: Option<String>,
    pub osu_file_url: Option<String>,
    pub beatmaps: Vec<Beatmap>,
}
pub fn convert_to_row_beatmapset(beatmapset: &BeatmapsetExtended) -> Beatmapset {
    Beatmapset {
        osu_id: Some(beatmapset.mapset_id as i32),
        artist: beatmapset.artist.clone(),
        artist_unicode: Some(
            beatmapset
                .artist_unicode
                .clone()
                .unwrap_or("Unknown".to_string()),
        ),
        title: beatmapset.title.clone(),
        title_unicode: Some(
            beatmapset
                .title_unicode
                .clone()
                .unwrap_or("Unknown".to_string()),
        ),
        creator: beatmapset.creator_name.to_string(),
        source: Some(beatmapset.source.to_string()),
        tags: None,
        has_video: beatmapset.video,
        has_storyboard: beatmapset.storyboard,
        is_explicit: beatmapset.nsfw,
        is_featured: false,
        cover_url: Some(beatmapset.covers.cover.to_string()),
        preview_url: Some(beatmapset.preview_url.clone()),
        osu_file_url: Some(beatmapset.source.to_string()),
        beatmaps: Vec::new(),
    }
}
