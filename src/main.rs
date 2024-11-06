use std::sync::Arc;

use league_fan_assets::preludes::*;
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
    let handles = loot.collect_tasks(config);
    let client = DownloadClient::default();
    let mut join_handles = vec![];

    for handle in handles {
        let client = client.clone();
        let join_handle = tokio::spawn(async move {
            if let Err(e) = client.do_task(&handle).await {
                error!("Error: {}", e);
            }
        });
        join_handles.push(join_handle);
    }
    for join_handle in join_handles {
        join_handle.await.unwrap();
    }
}
