use dto::models::beatmaps::full::types::Beatmapset;
use rosu_v2::prelude::BeatmapsetExtended;
use chrono::DateTime;

pub fn beatmapset_from_beatmapset_extended(beatmapset: &BeatmapsetExtended) -> Beatmapset {
    Beatmapset {
        id: None,
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
        osu_status_changed_at: Some(DateTime::from_timestamp(
            beatmapset.last_updated.unix_timestamp(),
            0
        )
        .unwrap_or_default()
        .naive_utc()),
    }
}
