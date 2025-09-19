use crate::config::Config;

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "sqlite:///pendora.db".to_string(),
            osu_client_id: "".to_string(),
            osu_client_secret: "".to_string(),
            discord_bot_token: "".to_string(),
        }
    }
}

impl Config {
    /// Crée une configuration avec des valeurs par défaut
    #[allow(dead_code)]
    pub fn with_defaults() -> Self {
        Self::default()
    }
}
