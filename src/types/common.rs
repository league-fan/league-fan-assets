use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::middleware::FetchClient;

use super::utils::{get_assets_url, AssetsTypeTrait, Config};

pub trait CollectTasks {
    fn collect_tasks(&self) -> Vec<impl std::future::Future<Output = Result<()>> + Send>
    where
        Self: Sync;
}

pub trait FromUrl: DeserializeOwned + AssetsTypeTrait {
    fn from_url(config: &Config) -> impl std::future::Future<Output = Result<Self>> + Send
    where
        Self: Sync,
    {
        async {
            let assets_type = Self::assets_type(); // 使用 Self 调用关联函数
            let url = get_assets_url(&assets_type, &config.language, &config.version);
            eprintln!("url: {}", url);
            FetchClient::default()
                .get(&url)
                .await?
                .json::<Self>() // 明确指定反序列化的目标类型
                .await
                .map_err(|e| e.into())
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub region: RegionEnum,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rarity {
    pub region: RegionEnum,
    pub rarity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RarityEnum {
    #[serde(rename = "kEpic")]
    Epic,

    #[serde(rename = "kLegendary")]
    Legendary,

    #[serde(rename = "kMythic")]
    Mythic,

    #[default]
    #[serde(rename = "kNoRarity")]
    NoRarity,

    #[serde(rename = "kRare")]
    Rare,

    #[serde(rename = "kTranscendent")]
    Transcendent,

    #[serde(rename = "kUltimate")]
    Ultimate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegionEnum {
    #[default]
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "ID")]
    Id,

    #[serde(rename = "PH")]
    Ph,

    #[serde(rename = "tencent")]
    RegionTencent,

    #[serde(rename = "riot")]
    Riot,

    #[serde(rename = "SG")]
    Sg,

    #[serde(rename = "TENCENT")]
    Tencent,

    #[serde(rename = "TH")]
    Th,

    #[serde(rename = "TW")]
    Tw,

    #[serde(rename = "VN")]
    Vn,
}
