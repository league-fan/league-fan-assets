import type { Loot } from "./bindings/Loot";
import { AssetsType, DDRAGON_VERSIONS_URL, LanguageType, safeJsonParse, type CDragonUrlConfig } from "./types";
import { assets_json_url } from "./utils";

async function fetchAndParseAsset<T>(assetType: AssetsType, config: CDragonUrlConfig = { version: 'latest', language: LanguageType.Default }): Promise<T> {
    const url = assets_json_url(assetType, config);
    const response = await fetch(url);
    if (!response.ok) {
        throw new Error(`Failed to fetch ${assetType}: ${response.statusText}`);
    }
    const data = await response.json();
    return data as T;
}

async function fetchCurrentVersion() {
    const response = await fetch(DDRAGON_VERSIONS_URL);
    if (!response.ok) {
        throw new Error(`Failed to fetch current version: ${response.statusText}`);
    }
    const data = await response.json();
    return data[0] as string;
}


const scraper = {
    fetchAndParseAsset,
    fetchCurrentVersion,
};

export default scraper;