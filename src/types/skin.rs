use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    common::{Description, Rarity, RarityEnum},
    utils::{get_assets_url, AssetsType, Config},
};

pub struct Skins(pub HashMap<String, Skin>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub id: i64,
    pub is_base: bool,
    pub name: String,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub skin_type: SkinType,
    pub rarity: RarityEnum,
    pub is_legacy: bool,
    pub splash_video_path: Option<String>,
    pub collection_splash_video_path: Option<String>,
    pub collection_card_hover_video_path: Option<String>,
    pub features_text: Option<String>,
    pub chroma_path: Option<String>,
    pub emblems: Option<Value>,
    pub region_rarity_id: i64,
    pub rarity_gem_path: Option<Value>,
    pub skin_lines: Option<Vec<SkinLine>>,
    pub description: Option<String>,
    pub chromas: Option<Vec<Chroma>>,
    pub skin_augments: Option<SkinAugments>,
    pub quest_skin_info: Option<QuestSkinInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinLine {
    id: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chroma {
    pub id: i64,
    pub name: String,
    pub chroma_path: String,
    pub colors: Vec<String>,
    pub descriptions: Vec<Description>,
    pub rarities: Vec<Rarity>,
    pub skin_augments: Option<SkinAugments>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinAugments {
    pub borders: Borders,
    pub augments: Option<Vec<Augment>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Borders {
    pub layer0: Option<Vec<Layer>>,
    pub layer1: Option<Vec<Layer>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub content_id: String,
    pub layer: i64,
    pub priority: i64,
    pub border_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Augment {
    pub content_id: String,
    pub overlays: Vec<Overlay>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Overlay {
    #[serde(rename = "centeredLCOverlayPath")]
    pub centered_lcoverlay_path: String,
    #[serde(rename = "uncenteredLCOverlayPath")]
    pub uncentered_lcoverlay_path: String,
    #[serde(rename = "socialCardLCOverlayPath")]
    pub social_card_lcoverlay_path: String,
    #[serde(rename = "tileLCOverlayPath")]
    pub tile_lcoverlay_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestSkinInfo {
    pub name: String,
    pub product_type: String,
    pub collection_description: String,
    pub description_info: Vec<DescriptionInfo>,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub collection_card_path: String,
    pub tiers: Vec<Tier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tier {
    pub id: i64,
    pub name: String,
    pub stage: i64,
    pub description: String,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub load_screen_path: String,
    pub short_name: String,
    pub splash_video_path: Option<String>,
    pub collection_splash_video_path: Option<String>,
    pub collection_card_hover_video_path: Option<String>,
    pub skin_augments: Option<SkinAugments>,
    pub load_screen_vintage_path: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionInfo {
    title: String,
    description: String,
    icon_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SkinType {
    #[default]
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Ultimate")]
    Ultimate,
}

impl Skins {
    pub async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let url = get_assets_url(&AssetsType::Skins, &config.language, &config.version);
        let body = reqwest::get(&url)
            .await?
            .json::<HashMap<String, Skin>>()
            .await?;
        Ok(Skins(body))
    }
}

pub struct Skinlines(pub Vec<Skinline>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skinline {
    id: i64,
    name: String,
    description: String,
}

impl Skinlines {
    pub async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let url = get_assets_url(&AssetsType::Skinlines, &config.language, &config.version);
        let body = reqwest::get(&url).await?.json::<Vec<Skinline>>().await?;
        Ok(Skinlines(body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_skins() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
        );

        let skins = Skins::get(&config).await.unwrap();
        let skin = skins.0.get("1000").unwrap();
        assert_eq!(skin.id, 1000);
        assert_eq!(skin.name, "Annie");
    }

    #[tokio::test]
    async fn test_skinlines() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
        );

        let skinlines = Skinlines::get(&config).await.unwrap();
        let skinline = &skinlines.0[1];
        assert_eq!(skinline.id, 1);
        assert_eq!(skinline.name, "World Champions: 2011");
    }
}
