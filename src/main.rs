use std::{path::Path, sync::Arc};

use league_fan_assets::types::preludes::*;
#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    let config = Arc::new(Config::new(None, LanguageType::ChineseChina, None));

    let loot = Loot::from_url(config.as_ref()).await.unwrap();
    let handles = loot.collect_download_tasks(config);
    for handle in handles {
        match handle.await {
            Ok(_) => println!("✅ Downloaded"),
            Err(e) => eprintln!("❌ Failed to download: {:?}", e),
        }
    }
}
