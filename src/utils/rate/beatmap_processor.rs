use rosu_map::section::hit_objects::{HitObject, HitObjectKind};
use rosu_map::Beatmap;

/// Traite les beatmaps en appliquant des modifications de rate
pub struct BeatmapProcessor;

impl BeatmapProcessor {
    /// Applique un rate sur un beatmap
    pub fn apply_rate(rate: f64, map: &mut Beatmap) {
        // Modifie le nom du fichier audio
        map.audio_file = map
            .audio_file
            .replace(".mp3", format!("_r{}.ogg", rate).as_str());

        let time_multiplier: f64 = 1.0 / rate;

        // Applique le multiplicateur de temps à tous les hit objects
        for hit_object in &mut map.hit_objects {
            Self::adjust_hit_object_timing(hit_object, time_multiplier);
        }

        // Applique le multiplicateur de temps aux timing points
        for timing_point in &mut map.control_points.timing_points {
            timing_point.time *= time_multiplier;
            timing_point.beat_len *= time_multiplier;
        }

        // Applique le multiplicateur de temps aux effect points
        for effect_point in &mut map.control_points.effect_points {
            effect_point.time *= time_multiplier;
        }

        // Applique le multiplicateur de temps aux difficulty points
        for difficulty_point in &mut map.control_points.difficulty_points {
            difficulty_point.time *= time_multiplier;
        }

        // Applique le multiplicateur de temps aux sample points
        for point in &mut map.control_points.sample_points {
            point.time *= time_multiplier;
        }

        // Ajoute le rate à la version
        map.version.push_str(&format!(" {}", rate));
    }

    /// Ajuste le timing d'un hit object selon le multiplicateur
    fn adjust_hit_object_timing(hit_object: &mut HitObject, time_multiplier: f64) {
        hit_object.start_time *= time_multiplier;

        if let HitObjectKind::Hold(hold) = &mut hit_object.kind {
            hold.duration *= time_multiplier;
        }
    }

    /// Applique un rate directement sur un beatmap (modifie le beatmap en place)
    pub fn apply_rate_to_beatmap(rate: f64, map: &mut Beatmap) {
        Self::apply_rate(rate, map);
    }
}
