use std::{path::Path, sync::Arc};

use league_fan_assets::types::preludes::*;
use tokio::fs;

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    let config = Arc::new(Config::new(None, LanguageType::ChineseChina));

    let loot = Loot::get(&config).await.unwrap();
    let chest = loot
        .loot_items
        .iter()
        .filter(|x| x.type_field == TypeField::Chest)
        .cloned() // 克隆数据
        .collect::<Vec<_>>();

    let mut handles = vec![];

    for x in chest {
        let config = Arc::clone(&config);
        let handle = tokio::spawn(async move {
            if x.image.trim_start_matches(' ').is_empty() {
                return;
            }
            let path = Path::new(".").join(x.image.trim_start_matches('/'));
            if fs::metadata(&path).await.is_ok() {
                eprintln!("🔥  {:?} skiped", &path);
                return;
            }
            let dir = path.parent().unwrap();
            fs::create_dir_all(dir).await.unwrap();

            let url = get_cdragon_url(x.image.as_str(), &config);
            eprintln!("🚀  {} -> {:?}", url, &path);
            let body = reqwest::get(&url).await.unwrap().bytes().await.unwrap();
            fs::write(&path, &body).await.unwrap();
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        handle.await.unwrap();
    }
}
