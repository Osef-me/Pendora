use std::fs;
use std::path::Path;
use tracing::info;

/// Gestionnaire de fichiers et dossiers pour les beatmaps
pub struct FileManager;

impl FileManager {
    /// Crée la structure de dossiers nécessaire pour un beatmap
    pub fn create_beatmap_directory_structure(beatmap_id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let paths = [
            "public",
            "public/beatmap",
            &format!("public/beatmap/{}", beatmap_id),
        ];

        for path in &paths {
            if !Path::new(path).exists() {
                fs::create_dir(path)?;
                info!("Created directory: {}", path);
            }
        }
        Ok(())
    }

    /// Sauvegarde des données compressées dans un fichier .br
    pub fn save_compressed_file(
        beatmap_id: i32,
        hash: &str,
        compressed_data: &[u8],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let file_path = format!("public/beatmap/{}/{}.br", beatmap_id, hash);
        fs::write(&file_path, compressed_data)?;
        Ok(file_path)
    }

    /// Génère le chemin d'un fichier de beatmap
    pub fn get_beatmap_file_path(beatmap_id: i32, hash: &str) -> String {
        format!("public/beatmap/{}/{}.br", beatmap_id, hash)
    }

    /// Vérifie si un fichier de beatmap existe déjà
    pub fn beatmap_file_exists(beatmap_id: i32, hash: &str) -> bool {
        let file_path = Self::get_beatmap_file_path(beatmap_id, hash);
        Path::new(&file_path).exists()
    }

    /// Crée un dossier s'il n'existe pas
    pub fn ensure_directory_exists(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(path).exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }
}
