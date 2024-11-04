use league_fan_assets::types::preludes::*;

#[tokio::main]
async fn main() {
    let config = Config::new(None, LanguageType::Default);
    let all_urls = get_all_assets_urls(&config);
    all_urls.iter().for_each(|url| {
        println!("{}", url);
    });
}
