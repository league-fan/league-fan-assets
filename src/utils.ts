import { AssetsType, LanguageType, COMMUNITY_DRAGON_URL, type CDragonUrlConfig } from "./types";


export const assets_json_url = (assets_type: AssetsType, config: CDragonUrlConfig = { version: 'latest', language: LanguageType.Default }) => {
    const { version, language } = config;
    return `${COMMUNITY_DRAGON_URL}/${version}/plugins/rcp-be-lol-game-data/global/${language}/v1/${assets_type}`;
};

export const assets_item_url = (item_url_with_prefix: string, config: CDragonUrlConfig = { version: 'latest', language: LanguageType.Default }) => {
    // https://github.com/communitydragon/docs/blob/master/assets.md#mapping-paths-from-json-files
    const { version, language } = config;
    const url_trimmed = item_url_with_prefix.replace(/^\/lol-game-data\/assets\//, '');
    let version_trimmed = version;
    if (version != 'latest') {
        version_trimmed = version.split('.').slice(0, 1).join('.');
    }
    return `${COMMUNITY_DRAGON_URL}/${version}/plugins/rcp-be-lol-game-data/global/${language}/${url_trimmed}`;
}