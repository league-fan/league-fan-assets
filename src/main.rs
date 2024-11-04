use std::{fs::File, io::Write};

use league_fan_assets::types::preludes::*;

#[tokio::main]
async fn main() {
    let config = Config::new(None, LanguageType::ChineseChina);

    let loot = Loot::get(&config).await.unwrap();
    let chest = loot
        .loot_items
        .iter()
        .filter(|x| x.type_field == TypeField::Chest)
        .collect::<Vec<_>>();
    let tasks = chest.iter().map(|x| async {
        if x.image.trim_start_matches(' ').is_empty() {
            return;
        }
        let url = get_cdragon_url(x.image.as_str(), &config);
        eprintln!("Origin: {}", x.image);
        eprintln!("Downloading: {}", url);
        let body = reqwest::get(&url).await.unwrap().bytes().await.unwrap();
        let path = format!("assets/loot/{}", x.image);
        let mut file = File::create(path).unwrap();
        file.write_all(&body).unwrap();
    });

    for task in tasks {
        task.await;
    }
}
