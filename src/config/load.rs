use crate::config::Config;
use crate::errors::config::ConfigError;
use dotenvy::dotenv;
use std::env;

impl Config {
    /// Charge la configuration depuis les variables d'environnement
    pub fn load() -> Result<Self, ConfigError> {
        // Charger le fichier .env s'il existe
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVariable("DATABASE_URL".to_string()))?;

        let osu_client_id = env::var("OSU_CLIENT_ID")
            .map_err(|_| ConfigError::MissingVariable("OSU_CLIENT_ID".to_string()))?;

        let osu_client_secret = env::var("OSU_CLIENT_SECRET")
            .map_err(|_| ConfigError::MissingVariable("OSU_CLIENT_SECRET".to_string()))?;

        let discord_bot_token = env::var("DISCORD_BOT_TOKEN")
            .map_err(|_| ConfigError::MissingVariable("DISCORD_BOT_TOKEN".to_string()))?;

        Ok(Config {
            database_url,
            osu_client_id,
            osu_client_secret,
            discord_bot_token,
        })
    }

    /// Charge la configuration avec des valeurs par défaut pour les variables manquantes
    #[allow(dead_code)]
    pub fn load_with_defaults() -> Result<Self, ConfigError> {
        // Charger le fichier .env s'il existe
        dotenv().ok();

        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:///pendora.db".to_string());

        let osu_client_id = env::var("OSU_CLIENT_ID").unwrap_or_else(|_| "".to_string());

        let osu_client_secret = env::var("OSU_CLIENT_SECRET").unwrap_or_else(|_| "".to_string());

        let discord_bot_token = env::var("DISCORD_BOT_TOKEN").unwrap_or_else(|_| "".to_string());

        Ok(Config {
            database_url,
            osu_client_id,
            osu_client_secret,
            discord_bot_token,
        })
    }
}
