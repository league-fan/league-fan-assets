use serde::{Deserialize, Serialize};

use super::utils::{get_assets_url, AssetsType, Config};

pub struct SummonerEmotes(pub Vec<SummonerEmote>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerEmote {
    pub id: i64,
    pub name: String,
    pub inventory_icon: String,
    pub description: String,
}

impl SummonerEmotes {
    pub async fn get(config: &Config) -> Result<Self, reqwest::Error> {
        let url = get_assets_url(
            &AssetsType::SummonerEmotes,
            &config.language,
            &config.version,
        );
        let body = reqwest::get(&url)
            .await?
            .json::<Vec<SummonerEmote>>()
            .await?;
        Ok(SummonerEmotes(body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_summoner_emotes() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
        );
        let summoner_emotes = SummonerEmotes::get(&config).await.unwrap();
        let emote = &summoner_emotes.0[1];
        assert_eq!(emote.id, 10);
        assert_eq!(emote.name, "Mastery 10+");
        assert_eq!(emote.description, "Champion Mastery Level 10+");
    }
}
