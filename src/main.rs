use std::sync::Arc;

use league_fan_assets::{preludes::*, types::summoner_banners};
use log::{error, info};

fn init_logger() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();
}

async fn collect_tasks(config: Arc<Config>) -> Vec<AssetsTask> {
    let mut handles = Vec::new();

    handles.push(SummonerIcons::to_task(config.clone()));
    handles.push(SummonerIconSets::to_task(config.clone()));
    handles.push(SummonerEmotes::to_task(config.clone()));
    handles.push(SummonerBanners::to_task(config.clone()));
    handles.push(Loot::to_task(config.clone()));
    handles.push(Skins::to_task(config.clone()));
    handles.push(Skinlines::to_task(config.clone()));
    handles.push(WardSkins::to_task(config.clone()));
    handles.push(WardSkinSets::to_task(config.clone()));

    let summoner_icons = SummonerIcons::from_url(config.as_ref())
        .await
        .expect("summoner_icons");
    handles.extend(summoner_icons.collect_tasks(config.clone()));

    let summoner_emotes = SummonerEmotes::from_url(config.as_ref())
        .await
        .expect("summoner_emotes");
    handles.extend(summoner_emotes.collect_tasks(config.clone()));

    let summoner_banners = SummonerBanners::from_url(config.as_ref())
        .await
        .expect("summoner_banners");
    handles.extend(summoner_banners.collect_tasks(config.clone()));

    let loot = Loot::from_url(config.as_ref()).await.expect("loot");
    handles.extend(loot.collect_tasks(config.clone()));

    let skins = Skins::from_url(config.as_ref()).await.expect("skins");
    handles.extend(skins.collect_tasks(config.clone()));

    let wardskins = WardSkins::from_url(config.as_ref())
        .await
        .expect("wardskins");
    handles.extend(wardskins.collect_tasks(config.clone()));

    handles
}

#[tokio::main(flavor = "multi_thread", worker_threads = 3)]
async fn main() {
    init_logger();

    let config = Arc::new(Config::new(None, LanguageType::ChineseChina, None));
    let handles = collect_tasks(config.clone()).await;
    let client = R2Client::default();
    let join_handles: Vec<_> = handles
        .into_iter()
        .map(|handle| {
            let client = client.clone();
            tokio::spawn(async move {
                if let Err(e) = client.do_task(&handle).await {
                    error!("❓Error: {}", e);
                } else {
                    info!("✅Downloaded: {}", handle.path);
                }
            })
        })
        .collect();

    for join_handle in join_handles {
        join_handle.await.unwrap();
    }
}
