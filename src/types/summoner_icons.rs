use serde::{Deserialize, Serialize};

use crate::preludes::{AssetsTask, CollecTasks, FilterEmptyAssets, ToTasks};

use super::{
    common::{Description, Rarity},
    common_trait::FromUrl,
    utils::{AssetsType, AssetsTypeTrait},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl FromUrl for SummonerIcons {}
impl FromUrl for SummonerIconSets {}

impl AssetsTypeTrait for SummonerIcons {
    fn assets_type() -> AssetsType {
        AssetsType::SummonerIcons
    }
}

impl AssetsTypeTrait for SummonerIconSets {
    fn assets_type() -> AssetsType {
        AssetsType::SummonerIconSets
    }
}

impl ToTasks for SummonerIcon {
    fn to_tasks(&self, config: std::sync::Arc<super::utils::Config>) -> Vec<AssetsTask> {
        let mut tasks = vec![];
        if let Some(path) = self.image_path.clone() {
            if let Some(path) = path.filter_empty_assets() {
                let task = AssetsTask::from_path_config(&path, &config);
                tasks.push(task);
            }
        }
        tasks
    }
}

impl CollecTasks for SummonerIcons {
    fn collect_tasks(&self, config: std::sync::Arc<super::utils::Config>) -> Vec<AssetsTask> {
        let mut tasks: Vec<AssetsTask> = vec![];
        for item in &self.0 {
            tasks.extend(item.to_tasks(config.clone()));
        }
        tasks
    }
}

impl CollecTasks for SummonerIconSets {
    fn collect_tasks(&self, _config: std::sync::Arc<super::utils::Config>) -> Vec<AssetsTask> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::types::utils::Config;

    use super::*;

    #[tokio::test]
    async fn test_summoner_icons() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
            None,
        );
        let summoner_icons = SummonerIcons::from_url(&config).await.unwrap();
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
            None,
        );
        let summoner_icon_sets = SummonerIconSets::from_url(&config).await.unwrap();
        let summoner_icon_set = &summoner_icon_sets.0[0];
        assert_eq!(summoner_icon_set.id, 100);
        assert_eq!(summoner_icon_set.hidden, false);
        assert_eq!(summoner_icon_set.display_name, "Special Events");
    }
}
