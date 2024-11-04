use league_fan_assets::types::preludes::*;

#[tokio::main]
async fn main() {
    let config = Config::new(None, LanguageType::Default);
    let summoner_icons = SummonerIcons::get(&config).await.unwrap();
    println!("{:?}", summoner_icons.0[0]);
}
