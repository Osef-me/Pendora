use std::fs;

/// Gestionnaire de fichiers et dossiers pour les beatmaps
pub struct FileManager;

impl FileManager {
    pub fn save_compressed_file(
        beatmap_id: i32,
        hash: &str,
        compressed_data: &[u8],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let file_path = format!("public/beatmap/{}/{}.br", beatmap_id, hash);
        fs::write(&file_path, compressed_data)?;
        Ok(file_path)
    }
}
