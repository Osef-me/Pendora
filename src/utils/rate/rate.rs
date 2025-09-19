use super::hash::hash_md5;
use brotli::enc::BrotliEncoderParams;
use rayon::prelude::*;
use rosu_map::Beatmap;
use rosu_map::section::hit_objects::HitObject;
use rosu_map::section::hit_objects::HitObjectKind;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{debug, info};

/// Fonction principale pour générer et compresser toutes les variations
pub async fn bulk_rate(
    rates: &Vec<f64>,
    maps: Beatmap,
    beatmap_id: i32,
) -> Result<Vec<(f64, String)>, Box<dyn std::error::Error>> {
    create_directory_structure(beatmap_id)?;

    let total_original_size = AtomicUsize::new(0);
    let total_compressed_size = AtomicUsize::new(0);

    let valid_rates: Vec<f64> = rates.iter().copied().filter(|r| *r != 1.0).collect();

    let rates_and_hashes: Vec<(f64, String)> = valid_rates
        .par_iter()
        .map(|&rate_value| {
            process_single_rate(
                rate_value,
                &maps,
                beatmap_id,
                &total_original_size,
                &total_compressed_size,
            )
        })
        .collect();

    log_compression_summary(
        &total_original_size,
        &total_compressed_size,
        beatmap_id,
        rates_and_hashes.len(),
    );

    Ok(rates_and_hashes)
}

/// Crée les dossiers nécessaires
fn create_directory_structure(beatmap_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let paths = [
        "public",
        "public/beatmap",
        &format!("public/beatmap/{}", beatmap_id),
    ];

    for path in &paths {
        if !Path::new(path).exists() {
            fs::create_dir(path)?;
        }
    }
    Ok(())
}

/// Traite une seule rate : clone le beatmap, applique la rate, compresse et sauvegarde
fn process_single_rate(
    rate_value: f64,
    maps: &Beatmap,
    beatmap_id: i32,
    total_original_size: &AtomicUsize,
    total_compressed_size: &AtomicUsize,
) -> (f64, String) {
    let mut map = maps.clone();
    rate(rate_value, &mut map);

    let encoded = map.encode_to_string().unwrap();
    let hash = hash_md5(encoded.as_str()).unwrap();

    let (compressed_data, original_size, compressed_size) = compress_brotli(encoded.as_bytes());

    // Sauvegarder le fichier compressé
    let path = format!("public/beatmap/{}/{}.br", beatmap_id, hash);
    fs::write(&path, &compressed_data).unwrap();

    // Mettre à jour les totaux
    total_original_size.fetch_add(original_size, Ordering::Relaxed);
    total_compressed_size.fetch_add(compressed_size, Ordering::Relaxed);

    let compression_ratio = (compressed_size as f64 / original_size as f64) * 100.0;
    let saved_bytes = original_size - compressed_size;
    debug!(
        "Rate {}: {} bytes -> {} bytes ({}% compression, {} bytes saved)",
        rate_value,
        original_size,
        compressed_size,
        compression_ratio.round(),
        saved_bytes
    );

    (rate_value, hash)
}

/// Compresse les données en Brotli et retourne (compressed_data, original_size, compressed_size)
fn compress_brotli(data: &[u8]) -> (Vec<u8>, usize, usize) {
    let original_size = data.len();
    let mut compressed_data = Vec::new();
    let params = BrotliEncoderParams::default();
    brotli::enc::BrotliCompress(&mut data.clone(), &mut compressed_data, &params).unwrap();
    let compressed_size = compressed_data.len();
    (compressed_data, original_size, compressed_size)
}

/// Affiche le résumé global de la compression
fn log_compression_summary(
    total_original_size: &AtomicUsize,
    total_compressed_size: &AtomicUsize,
    beatmap_id: i32,
    num_variations: usize,
) {
    let total_original_size = total_original_size.load(Ordering::Relaxed);
    let total_compressed_size = total_compressed_size.load(Ordering::Relaxed);

    if total_original_size > 0 {
        let total_compression_ratio =
            (total_compressed_size as f64 / total_original_size as f64) * 100.0;
        let total_saved_bytes = total_original_size - total_compressed_size;

        info!("=== COMPRESSION SUMMARY ===");
        info!(
            "Total: {} bytes -> {} bytes ({}% compression, {} bytes saved)",
            total_original_size,
            total_compressed_size,
            total_compression_ratio.round(),
            total_saved_bytes
        );
        info!("Space saved: {:.2} KB", total_saved_bytes as f64 / 1024.0);
    }

    info!(
        "bulk_rate completed for beatmap_id: {}, generated {} variations",
        beatmap_id, num_variations
    );
}

/// Applique le rate sur un beatmap
pub fn rate(rate: f64, map: &mut Beatmap) {
    map.audio_file = map.audio_file.replace(".mp3", format!("_r{}.ogg", rate).as_str());
    let time_multiplier: f64 = 1.0 / rate;

    for hit_object in &mut map.hit_objects {
        match_hit_object(hit_object, time_multiplier);
    }

    for timing_point in &mut map.control_points.timing_points {
        timing_point.time *= time_multiplier;
        timing_point.beat_len *= time_multiplier;
    }

    for effect_point in &mut map.control_points.effect_points {
        effect_point.time *= time_multiplier;
    }

    for difficulty_point in &mut map.control_points.difficulty_points {
        difficulty_point.time *= time_multiplier;
    }

    for point in &mut map.control_points.sample_points {
        point.time *= time_multiplier;
    }

    map.version.push_str(&format!(" {}", rate));
}

fn match_hit_object(hit_object: &mut HitObject, time_multiplier: f64) {
    hit_object.start_time *= time_multiplier;
    if let HitObjectKind::Hold(hold) = &mut hit_object.kind {
        hold.duration *= time_multiplier;
    }
}
