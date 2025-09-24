pub mod calculator;
pub mod rate;
use anyhow::Result;
use minacalc_rs::Ssr;
use reqwest;
use rosu_map::Beatmap;
use rosu_v2::prelude::GameMode;
use rosu_v2::prelude::RankStatus;
use tracing::debug;

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
    // TODO: delete those and accept STD and 7K
    println!("mode: {:?}, cs: {}", mode, cs);
    if mode != GameMode::Mania {
        return false;
    }

    if cs != 4.0 {
        return false;
    }

    true
}

/// Détermine le pattern principal d'une beatmap basé sur les skillset scores et les LN
pub fn determine_main_pattern(skillset_scores: &Ssr, beatmap: &Beatmap) -> serde_json::Value {
    debug!("Determining main pattern for beatmap");

    // Compter les LN (Long Notes) dans la beatmap
    let ln_count = count_long_notes(beatmap);
    let total_objects = beatmap.hit_objects.len();
    let ln_ratio = if total_objects > 0 {
        ln_count as f64 / total_objects as f64
    } else {
        0.0
    };

    debug!(
        "LN count: {}, total objects: {}, LN ratio: {:.3}",
        ln_count, total_objects, ln_ratio
    );

    // Déterminer le pattern basé sur les LN
    let ln_pattern = if ln_ratio > 0.8 {
        "LN"
    } else if ln_ratio > 0.5 {
        "Hybrid"
    } else {
        ""
    };

    // Récupérer les deux valeurs les plus élevées des skillset scores (sans overall)
    let mut scores = vec![
        ("stream", skillset_scores.stream),
        ("jumpstream", skillset_scores.jumpstream),
        ("handstream", skillset_scores.handstream),
        ("technical", skillset_scores.technical),
        ("chordjack", skillset_scores.chordjack),
        ("jackspeed", skillset_scores.jackspeed),
    ];

    // Trier par valeur décroissante
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Prendre les deux premiers
    let top_two: Vec<&str> = scores.iter().take(2).map(|(name, _)| *name).collect();

    debug!("Top two skillset scores: {:?}", top_two);

    // Combiner le pattern LN avec les deux meilleurs skillsets
    let mut pattern_parts = Vec::new();

    if !ln_pattern.is_empty() {
        pattern_parts.push(serde_json::Value::String(ln_pattern.to_string()));
    }

    pattern_parts.extend(
        top_two
            .iter()
            .map(|s| serde_json::Value::String(s.to_string())),
    );

    debug!("Final main pattern array: {:?}", pattern_parts);

    serde_json::Value::Array(pattern_parts)
}

/// Compte le nombre de Long Notes (Hold objects) dans une beatmap
fn count_long_notes(beatmap: &Beatmap) -> usize {
    use rosu_map::section::hit_objects::HitObjectKind;

    beatmap
        .hit_objects
        .iter()
        .filter(|hit_object| matches!(hit_object.kind, HitObjectKind::Hold(_)))
        .count()
}
