use serde::{Deserialize, Serialize};

use super::utils::{get_assets_url, AssetsType, Config};

pub struct SummonerEmotes(Vec<SummonerEmote>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerEmote {
    pub id: i64,
    pub name: String,
    pub inventory_icon: String,
    pub description: String,
}

impl SummonerEmotes {
    async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let config = config.clone();
        let url = get_assets_url(AssetsType::SummonerEmotes, config.language, config.version);
        let body = reqwest::get(&url)
            .await?
            .json::<Vec<SummonerEmote>>()
            .await?;
        Ok(SummonerEmotes(body))
    }
}
