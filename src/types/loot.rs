use std::sync::Arc;

use super::{
    common::{CollectDownloadTasks, FromUrl, ToDownloadTasks},
    utils::{AssetsType, AssetsTypeTrait, Config},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Loot {
    #[serde(rename = "LootItems")]
    pub loot_items: Vec<LootItem>,
    #[serde(rename = "LootRecipes")]
    pub loot_recipes: Vec<LootRecipe>,
    #[serde(rename = "LootTables")]
    pub loot_tables: Vec<LootTable>,
    #[serde(rename = "LootBundles")]
    pub loot_bundles: Vec<LootBundle>,
    #[serde(rename = "LootTokenBankCards")]
    pub loot_token_bank_cards: Vec<LootTokenBankCard>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: String,
    pub start_date: String,
    pub end_date: String,
    pub mapped_store_id: i64,
    pub lifetime_max: i64,
    pub auto_redeem: bool,
    pub rarity: RarityField,
    #[serde(rename = "type")]
    pub type_field: TypeField,
    pub recipe_menu_active: Option<String>,
    pub recipe_menu_title: Option<String>,
    pub recipe_menu_subtitle: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RarityField {
    #[default]
    Default,
    Epic,
    Legendary,
    Mythic,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum TypeField {
    #[default]
    Boost,
    Chest,
    Companion,
    Currency,
    Material,
    Skin,
    #[serde(rename = "Skin_Rental")]
    SkinRental,
    #[serde(rename = "Statstone")]
    Statstone,
    #[serde(rename = "Statstone_Shard")]
    StatstoneShard,
    #[serde(rename = "SummonerIcon")]
    SummonerIcon,
    #[serde(rename = "TFT_Damage_Skin")]
    TftDamageSkin,
    #[serde(rename = "TFT_Map_Skin")]
    TftMapSkin,
    #[serde(rename = "WardSkin")]
    WardSkin,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootRecipe {
    pub id: String,
    pub description: String,
    pub context_menu_text: String,
    pub requirement_text: String,
    pub image_path: String,
    pub intro_video_path: String,
    pub loop_video_path: String,
    pub outro_video_path: String,
    pub has_visible_loot_odds: bool,
    pub outputs: Vec<Output>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub loot_id: String,
    pub localized_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootTable {
    pub id: String,
    pub description: String,
    pub description_long: String,
    pub image: String,
    pub drop_chance: Vec<DropChance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropChance {
    pub loot_id: String,
    pub localized_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootBundle {
    pub id: String,
    pub description: String,
    pub description_long: String,
    pub image: String,
    pub contents: Vec<Content>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub query: Query,
    pub quantity_expression: String,
    pub localized_description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub loot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootTokenBankCard {
    pub loot_item_name: String,
    pub backsplash_image_path: String,
    pub button_text: String,
    pub non_premium_cap_currency_id: String,
    pub premium_cap_currency_id: String,
    pub title_text: String,
    pub token_icon_path: String,
    pub tooltip_description_text: String,
    pub tooltip_splash_path: String,
    pub tooltip_title_text: String,
    pub unlock_item_id: String,
    pub unlock_item_type: String,
    pub activation_date: String,
    pub deactivation_date: String,
    pub store_link_item: String,
    pub store_link_type: String,
}

impl FromUrl for Loot {}

impl AssetsTypeTrait for Loot {
    fn assets_type() -> AssetsType {
        AssetsType::Loot
    }
}

impl ToDownloadTasks for LootItem {
    fn to_download_tasks(&self, config: Arc<Config>) -> Option<JoinHandle<Result<()>>> {
        let config = Arc::clone(&config);
        let image = self.image.clone();
        let url = image.trim_start_matches('/');

        Self::to_download_tasks_inner(url, config)
    }
}

impl ToDownloadTasks for LootBundle {
    fn to_download_tasks(&self, config: Arc<Config>) -> Option<JoinHandle<Result<()>>> {
        let config = Arc::clone(&config);
        let image = self.image.clone();
        let url = image.trim_start_matches('/');

        Self::to_download_tasks_inner(url, config)
    }
}

impl ToDownloadTasks for LootRecipe {
    fn to_download_tasks(&self, config: Arc<Config>) -> Option<JoinHandle<Result<()>>> {
        let config = Arc::clone(&config);
        let image = self.image_path.clone();
        let url = image.trim_start_matches('/');

        Self::to_download_tasks_inner(url, config)
    }
}

impl ToDownloadTasks for LootTable {
    fn to_download_tasks(&self, config: Arc<Config>) -> Option<JoinHandle<Result<()>>> {
        let config = Arc::clone(&config);
        let image = self.image.clone();
        let url = image.trim_start_matches('/');

        Self::to_download_tasks_inner(url, config)
    }
}

impl CollectDownloadTasks for Loot {
    fn collect_download_tasks(&self, config: Arc<Config>) -> Vec<JoinHandle<Result<()>>> {
        let mut tasks = vec![];

        for item in &self.loot_items {
            if let Some(task) = item.to_download_tasks(Arc::clone(&config)) {
                tasks.push(task);
            }
        }

        for bundle in &self.loot_bundles {
            if let Some(task) = bundle.to_download_tasks(Arc::clone(&config)) {
                tasks.push(task);
            }
        }

        for recipe in &self.loot_recipes {
            if let Some(task) = recipe.to_download_tasks(Arc::clone(&config)) {
                tasks.push(task);
            }
        }

        for table in &self.loot_tables {
            if let Some(task) = table.to_download_tasks(Arc::clone(&config)) {
                tasks.push(task);
            }
        }

        return tasks;
    }
}

#[cfg(test)]
mod tests {
    use crate::types::utils::Config;

    use super::*;

    #[tokio::test]
    async fn test_loot() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
            None,
        );

        let loot = Loot::from_url(&config).await.unwrap();
        assert_eq!(loot.loot_items[0].id, "STATSTONE_SHARD_66600132");
        assert_eq!(loot.loot_items[0].name, "Warwick - Series 1");
    }
}
