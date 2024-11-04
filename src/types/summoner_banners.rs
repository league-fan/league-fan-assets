use serde::{Deserialize, Serialize};

use super::utils::{get_assets_url, AssetsType, Config};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerBanners {
    #[serde(rename = "BannerFlags")]
    pub banner_flags: Vec<BannerFlag>,
    #[serde(rename = "BannerFrames")]
    pub banner_frames: Vec<BannerFrame>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerFlag {
    pub level: i64,
    pub theme: String,
    pub name: String,
    pub inventory_icon: String,
    pub profile_icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerFrame {
    pub level: i64,
    pub name: String,
    pub inventory_icon: String,
}

impl SummonerBanners {
    pub async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(AssetsType::SummonerBanners, config.language, config.version);
        let body = reqwest::get(&url).await?.json::<SummonerBanners>().await?;
        Ok(body)
    }
}
