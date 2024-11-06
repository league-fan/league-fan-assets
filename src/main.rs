use std::sync::Arc;

use league_fan_assets::preludes::*;
use log::{error, info};
#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();
    let config = Arc::new(Config::new(None, LanguageType::ChineseChina, None));

    let summoner_icons = SummonerIcons::from_url(config.as_ref()).await.unwrap();
    let handles = summoner_icons.collect_tasks(config);
    let client = DownloadClient::default();
    let mut join_handles = vec![];

    for handle in handles {
        let client = client.clone();
        let join_handle = tokio::spawn(async move {
            if let Err(e) = client.do_task(&handle).await {
                error!("Error: {}", e);
            } else {
                info!("Downloaded: {}", handle.path);
            }
        });
        join_handles.push(join_handle);
    }
    for join_handle in join_handles {
        join_handle.await.unwrap();
    }
}
