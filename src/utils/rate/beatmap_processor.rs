use rosu_map::section::hit_objects::{HitObject, HitObjectKind};
use rosu_map::Beatmap;

/// Traite les beatmaps en appliquant des modifications de rate
pub struct BeatmapProcessor;

impl BeatmapProcessor {
    /// Applique un centirate sur un beatmap (100 == 1.0x)
    pub fn apply_rate(centirate: i64, map: &Beatmap) -> Beatmap {
        // Cloner pour travailler sur une copie
        let mut map = map.clone();

        let audio = map.audio_file.clone();
        let formatted_rate = format!("{:.1}", centirate as f64 / 100.0);
        let new_audio = match audio.rfind('.') {
            Some(dot_idx) => {
                let (base, ext) = audio.split_at(dot_idx);
                format!("{}_r{}{}", base, formatted_rate, ext)
            }
            None => format!("{}_r{}", audio, formatted_rate),
        };
        map.audio_file = new_audio;

        // Utiliser directement centirate pour éviter les conversions inutiles
        let time_multiplier: f64 = 100.0 / centirate as f64;

        
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

        // Ajoute le rate à la version sous forme normalisée (ex: " 1.2x")
        map.version.push_str(&format!(" {}x", formatted_rate));

        return map;
    }

    /// Ajuste le timing d'un hit object selon le multiplicateur
    fn adjust_hit_object_timing(hit_object: &mut HitObject, time_multiplier: f64) {
        hit_object.start_time *= time_multiplier;

        if let HitObjectKind::Hold(hold) = &mut hit_object.kind {
            hold.duration *= time_multiplier;
        }
    }

    /// Applique un centirate directement sur un beatmap (modifie le beatmap en place)
    pub fn apply_rate_to_beatmap(centirate: i64, map: &mut Beatmap) {
        let new_map = Self::apply_rate(centirate, map);
        *map = new_map;
    }
}
