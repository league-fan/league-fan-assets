use serde::{Deserialize, Serialize};

use super::{
    common::{Rarity, RegionEnum},
    utils::{get_assets_url, AssetsType, Config},
};
pub type WardSkins = Vec<WardSkin>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WardSkin {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub ward_image_path: String,
    pub ward_shadow_image_path: String,
    pub is_legacy: bool,
    pub regional_descriptions: Vec<RegionalDescription>,
    pub rarities: Vec<Rarity>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegionalDescription {
    pub region: RegionEnum,
    pub description: String,
}

pub type WardSkinSets = Vec<WardSkinSet>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WardSkinSet {
    pub id: i64,
    pub hidden: bool,
    pub display_name: String,
    pub description: String,
    pub wards: Vec<i64>,
}

impl WardSkin {
    async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(AssetsType::WardSkins, config.language, config.version);
        let body = reqwest::get(&url).await?.json::<Self>().await?;
        Ok(body)
    }
}

impl WardSkinSet {
    async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(AssetsType::WardSkinSets, config.language, config.version);
        let body = reqwest::get(&url).await?.json::<Self>().await?;
        Ok(body)
    }
}
