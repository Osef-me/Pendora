use super::beatmap_processor::BeatmapProcessor;
use super::compression::CompressionManager;
use super::file_manager::FileManager;
use super::hash::hash_md5;
use rosu_map::Beatmap;


/// Traite une seule rate : clone le beatmap, applique la rate, compresse et sauvegarde
pub fn process_single_rate(centirate: i64, maps: &Beatmap, beatmap_id: i32) -> String {
    // 1. Cloner et traiter le beatmap avec le rate
    let mut processed_map = maps.clone();
    BeatmapProcessor::apply_rate_to_beatmap(centirate, &mut processed_map);

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
    compression_result.log_compression_details(centirate as f64 / 100.0);

    hash
}
