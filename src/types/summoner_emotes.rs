use serde::{Deserialize, Serialize};

use super::{
    common::FromUrl,
    utils::{AssetsType, AssetsTypeTrait},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SummonerEmotes(pub Vec<SummonerEmote>);

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerEmote {
    pub id: i64,
    pub name: String,
    pub inventory_icon: String,
    pub description: String,
}

impl FromUrl for SummonerEmotes {}

impl AssetsTypeTrait for SummonerEmotes {
    fn assets_type() -> AssetsType {
        AssetsType::SummonerEmotes
    }
}

#[cfg(test)]
mod tests {
    use crate::types::utils::Config;

    use super::*;

    #[tokio::test]
    async fn test_summoner_emotes() {
        let config = Config::new(
            Some("14.21.1".to_string()),
            crate::types::utils::LanguageType::Default,
            None,
        );
        let summoner_emotes = SummonerEmotes::from_url(&config).await.unwrap();
        let emote = &summoner_emotes.0[1];
        assert_eq!(emote.id, 10);
        assert_eq!(emote.name, "Mastery 10+");
        assert_eq!(emote.description, "Champion Mastery Level 10+");
    }
}
