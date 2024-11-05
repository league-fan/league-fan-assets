use std::sync::Arc;

use league_fan_assets::types::preludes::*;
use log::error;
#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();
    let config = Arc::new(Config::new(None, LanguageType::ChineseChina, None));

    let loot = Loot::from_url(config.as_ref()).await.unwrap();
    let handles = loot.collect_download_tasks(config);
    for handle in handles {
        match handle.await {
            Ok(result) => match result {
                Ok(_) => {}
                Err(e) => error!("❌ Failed to download: {:?}", e),
            },
            Err(e) => error!("❌ Task join error: {:?}", e),
        }
    }
}
