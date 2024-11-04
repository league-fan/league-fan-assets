const COMMUNITY_DRAGON_URL: &str = "https://raw.communitydragon.org/";
const DDRAGON_VERSIONS_URL: &str = "https://ddragon.leagueoflegends.com/api/versions.json";

#[derive(Default, Debug, Clone, PartialEq)]
pub enum AssetsType {
    #[default]
    Loot,
    Skins,
    SummonerEmotes,
    SummonerBanners,
    SummonerIcons,
    SummonerIconSets,
    WardSkins,
    WardSkinSets,
}

impl AssetsType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AssetsType::Loot => "loot.json",
            AssetsType::Skins => "skins.json",
            AssetsType::SummonerEmotes => "summoner-emotes.json",
            AssetsType::SummonerBanners => "summoner-banners.json",
            AssetsType::SummonerIcons => "summoner-icons.json",
            AssetsType::SummonerIconSets => "summoner-icon-sets.json",
            AssetsType::WardSkins => "ward-skins.json",
            AssetsType::WardSkinSets => "ward-skin-sets.json",
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LanguageType {
    #[default]
    Default,
    Arabic,
    Czech,
    German,
    Greek,
    EnglishAustralia,
    EnglishGreatBritain,
    EnglishPhilippines,
    EnglishSingapore,
    SpanishArgentina,
    SpanishSpain,
    SpanishMexico,
    FrenchFrance,
    Hungarian,
    Italian,
    Japanese,
    Korean,
    Polish,
    PortugueseBrazil,
    Romanian,
    Russian,
    Thai,
    Turkish,
    Vietnamese,
    ChineseChina,
    ChineseMalaysia,
    ChineseTaiwan,
}

impl LanguageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageType::Default => "default",
            LanguageType::Arabic => "ar_ae",
            LanguageType::Czech => "cs_cz",
            LanguageType::German => "de_de",
            LanguageType::Greek => "el_gr",
            LanguageType::EnglishAustralia => "en_au",
            LanguageType::EnglishGreatBritain => "en_gb",
            LanguageType::EnglishPhilippines => "en_ph",
            LanguageType::EnglishSingapore => "en_sg",
            LanguageType::SpanishArgentina => "es_ar",
            LanguageType::SpanishSpain => "es_es",
            LanguageType::SpanishMexico => "es_mx",
            LanguageType::FrenchFrance => "fr_fr",
            LanguageType::Hungarian => "hu_hu",
            LanguageType::Italian => "it_it",
            LanguageType::Japanese => "ja_jp",
            LanguageType::Korean => "ko_kr",
            LanguageType::Polish => "pl_pl",
            LanguageType::PortugueseBrazil => "pt_br",
            LanguageType::Romanian => "ro_ro",
            LanguageType::Russian => "ru_ru",
            LanguageType::Thai => "th_th",
            LanguageType::Turkish => "tr_tr",
            LanguageType::Vietnamese => "vi_vn",
            LanguageType::ChineseChina => "zh_cn",
            LanguageType::ChineseMalaysia => "zh_my",
            LanguageType::ChineseTaiwan => "zh_tw",
        }
    }
}

pub fn get_assets_url(
    assets_type: AssetsType,
    language: LanguageType,
    version: Option<String>,
) -> String {
    match version {
        Some(version) => format!(
            "https://raw.communitydragon.org/{}/plugins/rcp-be-lol-game-data/global/{}/v1/{}",
            version,
            language.as_str(),
            assets_type.as_str()
        ),
        None => format!(
            "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/{}/v1/{}",
            language.as_str(),
            assets_type.as_str()
        ),
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Config {
    pub version: Option<String>,
    pub language: LanguageType,
}

impl Config {
    pub fn new() -> Self {
        Self {
            version: None,
            language: LanguageType::default(),
        }
    }
}
