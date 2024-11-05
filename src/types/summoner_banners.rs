use serde::{Deserialize, Serialize};

use super::{
    common::FromUrl,
    utils::{AssetsType, AssetsTypeTrait},
};

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

impl FromUrl for SummonerBanners {}

impl AssetsTypeTrait for SummonerBanners {
    fn assets_type() -> AssetsType {
        AssetsType::SummonerBanners
    }
}

#[cfg(test)]
mod tests {
    use crate::types::utils::Config;

    use super::*;

    #[tokio::test]
    async fn test_summoner_banners() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
            None,
        );
        let summoner_banners = SummonerBanners::from_url(&config).await.unwrap();
        let banner_flag = &summoner_banners.banner_flags[0];
        let banner_frame = &summoner_banners.banner_frames[0];
        assert_eq!(banner_flag.level, 1);
        assert_eq!(banner_flag.theme, "Bandle_City");
        assert_eq!(banner_frame.level, 1);
        assert_eq!(banner_frame.name, "");
    }
}
