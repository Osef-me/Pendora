use crate::config::Config;
use crate::errors::config::ConfigError;
use db::config::DatabaseConfig;
use db::db::DatabaseManager;
use dotenvy::dotenv;
use std::env;

impl Config {
    /// Charge la configuration depuis les variables d'environnement
    pub async fn load() -> Result<Self, ConfigError> {
        // Charger le fichier .env s'il existe
        dotenv().ok();

        let osu_client_id = env::var("OSU_CLIENT_ID")
            .map_err(|_| ConfigError::MissingVariable("OSU_CLIENT_ID".to_string()))?;

        let osu_client_secret = env::var("OSU_CLIENT_SECRET")
            .map_err(|_| ConfigError::MissingVariable("OSU_CLIENT_SECRET".to_string()))?;

        let discord_bot_token = env::var("DISCORD_BOT_TOKEN")
            .map_err(|_| ConfigError::MissingVariable("DISCORD_BOT_TOKEN".to_string()))?;

        let database_config = DatabaseConfig::load();
        let mut database = DatabaseManager::new();
        database.connect(&database_config).await.unwrap();
        Ok(Config {
            database: database,
            osu_client_id,
            osu_client_secret,
            discord_bot_token,
        })
    }

    /// Charge la configuration avec des valeurs par défaut pour les variables manquantes
    #[allow(dead_code)]
    pub async fn load_with_defaults() -> Result<Self, ConfigError> {
        // Charger le fichier .env s'il existe
        dotenv().ok();

        let osu_client_id = env::var("OSU_CLIENT_ID").unwrap_or_else(|_| "".to_string());

        let osu_client_secret = env::var("OSU_CLIENT_SECRET").unwrap_or_else(|_| "".to_string());

        let discord_bot_token = env::var("DISCORD_BOT_TOKEN").unwrap_or_else(|_| "".to_string());

        let database_config = DatabaseConfig::load();
        let mut database = DatabaseManager::new();
        database.connect(&database_config).await.unwrap();
        Ok(Config {
            database: database,
            osu_client_id,
            osu_client_secret,
            discord_bot_token,
        })
    }
}
