const COMMUNITY_DRAGON_URL = "https://raw.communitydragon.org";
const DDRAGON_VERSIONS_URL = "https://ddragon.leagueoflegends.com/api/versions.json";

enum LanguageType {
    Default = "default",
    Arabic = "ar_ae",
    Czech = "cs_cz",
    German = "de_de",
    Greek = "el_gr",
    EnglishAustralia = "en_au",
    EnglishGreatBritain = "en_gb",
    EnglishPhilippines = "en_ph",
    EnglishSingapore = "en_sg",
    SpanishArgentina = "es_ar",
    SpanishSpain = "es_es",
    SpanishMexico = "es_mx",
    FrenchFrance = "fr_fr",
    Hungarian = "hu_hu",
    Italian = "it_it",
    Japanese = "ja_jp",
    Korean = "ko_kr",
    Polish = "pl_pl",
    PortugueseBrazil = "pt_br",
    Romanian = "ro_ro",
    Russian = "ru_ru",
    Thai = "th_th",
    Turkish = "tr_tr",
    Vietnamese = "vi_vn",
    ChineseChina = "zh_cn",
    ChineseMalaysia = "zh_my",
    ChineseTaiwan = "zh_tw",
}

enum AssetsType {
    Loot = "loot.json",
    Skins = "skins.json",
    Skinlines = "skinlines.json",
    SummonerEmotes = "summoner-emotes.json",
    SummonerBanners = "summoner-banners.json",
    SummonerIcons = "summoner-icons.json",
    SummonerIconSets = "summoner-icon-sets.json",
    WardSkins = "ward-skins.json",
    WardSkinSets = "ward-skin-sets.json",
}

export type CDragonUrlConfig = {
    version: string,
    language: LanguageType,
}

export const safeJsonParse = <T>(guard: (o: any) => o is T) =>
    (text: string): ParseResult<T> => {
        const parsed = JSON.parse(text)
        return guard(parsed) ? { parsed, hasError: false } : { hasError: true }
    }

export type ParseResult<T> =
    | { parsed: T; hasError: false; error?: undefined }
    | { parsed?: undefined; hasError: true; error?: unknown }

export {
    COMMUNITY_DRAGON_URL,
    DDRAGON_VERSIONS_URL,
    LanguageType,
    AssetsType,
};