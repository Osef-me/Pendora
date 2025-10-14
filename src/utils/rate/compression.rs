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
        let mut input = data;

        brotli::enc::BrotliCompress(&mut input, &mut compressed_data, &params)
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

// removed unused CompressionStats
