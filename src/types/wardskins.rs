use serde::{Deserialize, Serialize};

use super::{
    common::{Rarity, RegionEnum},
    utils::{get_assets_url, AssetsType, Config},
};
pub struct WardSkins(Vec<WardSkin>);

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

pub struct WardSkinSets(Vec<WardSkinSet>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WardSkinSet {
    pub id: i64,
    pub hidden: bool,
    pub display_name: String,
    pub description: String,
    pub wards: Vec<i64>,
}

impl WardSkins {
    async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(AssetsType::WardSkins, config.language, config.version);
        let body = reqwest::get(&url).await?.json::<Vec<WardSkin>>().await?;
        Ok(WardSkins(body))
    }
}

impl WardSkinSets {
    async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(AssetsType::WardSkinSets, config.language, config.version);
        let body = reqwest::get(&url).await?.json::<Vec<WardSkinSet>>().await?;
        Ok(WardSkinSets(body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ward_skins() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
        );
        let ward_skins = WardSkins::get(&config).await.unwrap();
        let ward_skin = &ward_skins.0[0];
        assert_eq!(ward_skin.id, 0);
        assert_eq!(ward_skin.name, "Default Ward");
    }

    #[tokio::test]
    async fn test_ward_skin_sets() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
        );
        let ward_skin_sets = WardSkinSets::get(&config).await.unwrap();
        let ward_skin_set = &ward_skin_sets.0[0];
        assert_eq!(ward_skin_set.id, 10);
        assert_eq!(ward_skin_set.display_name, "Harrowing");
    }
}
