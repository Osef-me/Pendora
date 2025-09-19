mod beatmap_worker;
mod errors;
mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tokio::select! {
        result = beatmap_worker::start() => {
            println!("BeatmapWorker finished: {:?}", result);
        }
    }
}
