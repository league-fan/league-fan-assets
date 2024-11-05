use serde::{Deserialize, Serialize};

use super::{
    common::{FromUrl, Rarity, RegionEnum},
    utils::{AssetsType, AssetsTypeTrait},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WardSkins(pub Vec<WardSkin>);

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WardSkinSets(pub Vec<WardSkinSet>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WardSkinSet {
    pub id: i64,
    pub hidden: bool,
    pub display_name: String,
    pub description: String,
    pub wards: Vec<i64>,
}

impl FromUrl for WardSkins {}
impl FromUrl for WardSkinSets {}

impl AssetsTypeTrait for WardSkins {
    fn assets_type() -> AssetsType {
        AssetsType::WardSkins
    }
}

impl AssetsTypeTrait for WardSkinSets {
    fn assets_type() -> AssetsType {
        AssetsType::WardSkinSets
    }
}

#[cfg(test)]
mod tests {
    use crate::types::utils::Config;

    use super::*;

    #[tokio::test]
    async fn test_ward_skins() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
        );
        let ward_skins = WardSkins::from_url(&config).await.unwrap();
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
        let ward_skin_sets = WardSkinSets::from_url(&config).await.unwrap();
        let ward_skin_set = &ward_skin_sets.0[0];
        assert_eq!(ward_skin_set.id, 10);
        assert_eq!(ward_skin_set.display_name, "Harrowing");
    }
}
