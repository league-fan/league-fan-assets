use serde::{Deserialize, Serialize};

use super::{
    common::{Description, Rarity},
    utils::{get_assets_url, AssetsType, Config},
};

pub struct SummonerIcons(pub Vec<SummonerIcon>);

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

pub struct SummonerIconSets(pub Vec<SummonerIconSet>);

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
    pub async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(AssetsType::SummonerIcons, config.language, config.version);
        let body = reqwest::get(&url)
            .await?
            .json::<Vec<SummonerIcon>>()
            .await?;
        Ok(SummonerIcons(body))
    }
}

impl SummonerIconSets {
    pub async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(
            AssetsType::SummonerIconSets,
            config.language,
            config.version,
        );
        let body = reqwest::get(&url)
            .await?
            .json::<Vec<SummonerIconSet>>()
            .await?;
        Ok(SummonerIconSets(body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_summoner_icons() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
        );
        let summoner_icons = SummonerIcons::get(&config).await.unwrap();
        let icon = &summoner_icons.0[0];
        assert_eq!(icon.id, 0);
        assert_eq!(icon.title, "Blue Minion Bruiser Icon");
        assert_eq!(icon.year_released, 2009);
    }

    #[tokio::test]
    async fn test_summoner_icon_sets() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
        );
        let summoner_icon_sets = SummonerIconSets::get(&config).await.unwrap();
        let summoner_icon_set = &summoner_icon_sets.0[0];
        assert_eq!(summoner_icon_set.id, 100);
        assert_eq!(summoner_icon_set.hidden, false);
        assert_eq!(summoner_icon_set.display_name, "Special Events");
    }
}
