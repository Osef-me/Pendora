use brotli::enc::BrotliEncoderParams;
use tracing::debug;

/// Résultat d'une compression
#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub compressed_data: Vec<u8>,
    pub original_size: usize,
    pub compressed_size: usize,
}

impl CompressionResult {
    /// Calcule le ratio de compression en pourcentage
    pub fn compression_ratio(&self) -> f64 {
        (self.compressed_size as f64 / self.original_size as f64) * 100.0
    }

    /// Calcule le nombre d'octets économisés
    pub fn saved_bytes(&self) -> usize {
        self.original_size - self.compressed_size
    }

    /// Log les détails de la compression
    pub fn log_compression_details(&self, rate: f64) {
        debug!(
            "Rate {}: {} bytes -> {} bytes ({}% compression, {} bytes saved)",
            rate,
            self.original_size,
            self.compressed_size,
            self.compression_ratio().round(),
            self.saved_bytes()
        );
    }
}

/// Gestionnaire de compression Brotli
pub struct CompressionManager;

impl CompressionManager {
    /// Compresse des données en utilisant Brotli
    pub fn compress_brotli(data: &[u8]) -> Result<CompressionResult, Box<dyn std::error::Error>> {
        let original_size = data.len();
        let mut compressed_data = Vec::new();
        let params = BrotliEncoderParams::default();

        brotli::enc::BrotliCompress(&mut data.clone(), &mut compressed_data, &params)
            .map_err(|e| format!("Brotli compression failed: {}", e))?;

        let compressed_size = compressed_data.len();

        Ok(CompressionResult {
            compressed_data,
            original_size,
            compressed_size,
        })
    }

    /// Compresse une chaîne de caractères
    pub fn compress_string(data: &str) -> Result<CompressionResult, Box<dyn std::error::Error>> {
        Self::compress_brotli(data.as_bytes())
    }
}

/// Statistiques de compression pour plusieurs fichiers
#[derive(Debug, Default)]
pub struct CompressionStats {
    pub total_original_size: usize,
    pub total_compressed_size: usize,
    pub file_count: usize,
}

impl CompressionStats {
    /// Ajoute les statistiques d'un fichier
    pub fn add_file(&mut self, result: &CompressionResult) {
        self.total_original_size += result.original_size;
        self.total_compressed_size += result.compressed_size;
        self.file_count += 1;
    }

    /// Calcule le ratio de compression global
    pub fn total_compression_ratio(&self) -> f64 {
        if self.total_original_size > 0 {
            (self.total_compressed_size as f64 / self.total_original_size as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Calcule le total d'octets économisés
    pub fn total_saved_bytes(&self) -> usize {
        self.total_original_size - self.total_compressed_size
    }

    /// Calcule l'espace économisé en KB
    pub fn saved_kb(&self) -> f64 {
        self.total_saved_bytes() as f64 / 1024.0
    }
}
