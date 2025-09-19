mod default;
mod load;
use db::db::DatabaseManager;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseManager,
    #[allow(dead_code)]
    pub osu_client_id: String,
    #[allow(dead_code)]
    pub osu_client_secret: String,
    pub discord_bot_token: String,
}
