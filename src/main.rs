use std::{path::Path, sync::Arc};

use league_fan_assets::types::preludes::*;
use tokio::{
    fs,
    time::{sleep, Duration},
};

const MAX_RETRIES: u32 = 2;

async fn download_file(url: &str) -> Result<Vec<u8>, reqwest::Error> {
    let mut retries = 0;
    loop {
        match reqwest::get(url).await?.bytes().await {
            Ok(bytes) => return Ok(bytes.to_vec()),
            Err(e) => {
                if retries >= MAX_RETRIES {
                    return Err(e);
                }
                retries += 1;
                eprintln!(
                    "⚠️  Failed to download {}, retry {}/{}",
                    url, retries, MAX_RETRIES
                );
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn save_file(path: &Path, content: &[u8]) -> tokio::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    fs::write(path, content).await
}

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    let config = Arc::new(Config::new(None, LanguageType::ChineseChina));

    let loot = Loot::get(&config).await.unwrap();
    let chest = loot
        .loot_items
        .iter()
        .filter(|x| x.type_field == TypeField::Chest)
        .cloned()
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
                eprintln!("🔥  {:?} skipped", &path);
                return;
            }

            let url = get_cdragon_url(x.image.as_str(), &config);
            eprintln!("🚀  {} -> {:?}", url, &path);

            match download_file(&url).await {
                Ok(body) => {
                    if let Err(e) = save_file(&path, &body).await {
                        eprintln!("❌  Failed to save {:?}: {}", path, e);
                    }
                }
                Err(e) => {
                    eprintln!("❌  Failed to download {}: {}", url, e);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("❌  Task failed: {}", e);
        }
    }
}
