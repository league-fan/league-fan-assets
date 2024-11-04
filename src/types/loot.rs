use serde::{Deserialize, Serialize};

use super::utils::{get_assets_url, AssetsType, Config};

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
    pub rarity: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub recipe_menu_active: Option<String>,
    pub recipe_menu_title: Option<String>,
    pub recipe_menu_subtitle: Option<String>,
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

impl Loot {
    pub async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let url = get_assets_url(&AssetsType::Loot, &config.language, &config.version);
        let body = reqwest::get(&url).await?.json::<Self>().await?;
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Deserialize the loot.json file into a Loot struct and compare the id of the first loot item
    #[tokio::test]
    async fn test_loot() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
        );
        // {
        //     "LootItems": [
        //       {
        //         "id": "STATSTONE_SHARD_66600132",
        //         "name": "Warwick - Series 1",
        //         "description": "Unlock Series 1 Eternals for Warwick.",
        //         "image": "",
        //         "startDate": "",
        //         "endDate": "",
        //         "mappedStoreId": 0,
        //         "lifetimeMax": 0,
        //         "autoRedeem": true,
        //         "rarity": "Default",
        //         "type": "Statstone_Shard"
        //       }
        //     ]
        // }
        let loot = Loot::get(&config).await.unwrap();
        assert_eq!(loot.loot_items[0].id, "STATSTONE_SHARD_66600132");
        assert_eq!(loot.loot_items[0].name, "Warwick - Series 1");
    }
}
