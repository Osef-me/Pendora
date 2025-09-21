use crate::api::osu::OsuApiService;
use crate::config::Config;

pub struct BeatmapWorker {
    pub config: Config,
    pub osu_api_service: OsuApiService,
}
