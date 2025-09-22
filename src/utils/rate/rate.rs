use super::beatmap_processor::BeatmapProcessor;
use super::compression::CompressionManager;
use super::file_manager::FileManager;
use super::hash::hash_md5;
use rayon::prelude::*;
use rosu_map::Beatmap;
use tracing::info;

/// Fonction principale pour générer et compresser toutes les variations
pub async fn bulk_rate(
    rates: &Vec<f64>,
    maps: Beatmap,
    beatmap_id: i32,
) -> Result<Vec<(f64, String)>, Box<dyn std::error::Error>> {
    // Créer la structure de dossiers
    FileManager::create_beatmap_directory_structure(beatmap_id)?;

    // Filtrer les rates valides (exclure 1.0)
    let valid_rates: Vec<f64> = rates.iter().copied().filter(|r| *r != 1.0).collect();

    // Traiter en parallèle
    let rates_and_hashes: Vec<(f64, String)> = valid_rates
        .par_iter()
        .map(|&rate_value| process_single_rate(rate_value, &maps, beatmap_id))
        .collect();

    info!(
        "bulk_rate completed for beatmap_id: {}, generated {} variations",
        beatmap_id,
        rates_and_hashes.len()
    );

    Ok(rates_and_hashes)
}

/// Traite une seule rate : clone le beatmap, applique la rate, compresse et sauvegarde
fn process_single_rate(rate_value: f64, maps: &Beatmap, beatmap_id: i32) -> (f64, String) {
    // 1. Cloner et traiter le beatmap avec le rate
    let mut processed_map = maps.clone();
    BeatmapProcessor::apply_rate_to_beatmap(rate_value, &mut processed_map);

    // 2. Encoder le beatmap en string
    let encoded = processed_map.encode_to_string().unwrap();

    // 3. Générer le hash
    let hash = hash_md5(&encoded).unwrap();

    // 4. Compresser les données
    let compression_result = CompressionManager::compress_string(&encoded).unwrap();

    // 5. Sauvegarder le fichier compressé
    let _file_path =
        FileManager::save_compressed_file(beatmap_id, &hash, &compression_result.compressed_data)
            .unwrap();

    // 6. Logger les détails
    compression_result.log_compression_details(rate_value);

    (rate_value, hash)
}

/// Fonction de compatibilité pour appliquer un rate sur un beatmap
/// Utilise maintenant le BeatmapProcessor refactorisé
pub fn rate(rate: f64, map: &mut rosu_map::Beatmap) {
    super::beatmap_processor::BeatmapProcessor::apply_rate(rate, map);
}
