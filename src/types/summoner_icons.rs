use serde::{Deserialize, Serialize};

use super::{
    common::{Description, Rarity},
    utils::{get_assets_url, AssetsType, Config},
};

pub struct SummonerIcons(Vec<SummonerIcon>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerIcon {
    pub id: i64,
    pub title: String,
    pub year_released: i64,
    pub is_legacy: bool,
    pub image_path: Option<String>,
    pub descriptions: Vec<Description>,
    pub rarities: Vec<Rarity>,
    pub disabled_regions: Vec<String>,
    pub esports_team: Option<String>,
    pub esports_region: Option<String>,
    pub esports_event: Option<String>,
}

pub type SummonerIconSets = Vec<SummonerIconSet>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerIconSet {
    pub id: i64,
    pub hidden: bool,
    pub display_name: String,
    pub description: String,
    pub icons: Vec<i64>,
}

impl SummonerIcons {
    async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(AssetsType::SummonerIcons, config.language, config.version);
        let body = reqwest::get(&url)
            .await?
            .json::<Vec<SummonerIcon>>()
            .await?;
        Ok(SummonerIcons(body))
    }
}

impl SummonerIconSet {
    async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(
            AssetsType::SummonerIconSets,
            config.language,
            config.version,
        );
        let body = reqwest::get(&url).await?.json::<Self>().await?;
        Ok(body)
    }
}
