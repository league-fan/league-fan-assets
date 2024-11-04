use std::default;

use serde::{Deserialize, Serialize};

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
