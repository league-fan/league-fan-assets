export { createClient } from "./client.js";
export type {
  ClientOptions,
  LeagueFanAssetsClient,
  DataSource,
  LoadOptions,
} from "./client.js";

export { loadCategory } from "./loaders/category.js";
export { fetchJson, FetchError } from "./loaders/http.js";

export {
  ASSET_CATEGORIES,
  PUBLISHED_DATA_CATEGORIES,
  DEFAULT_DATA_LANGUAGES,
  CATEGORY_META,
} from "./catalog.js";
export type { CategoryMeta } from "./catalog.js";

export {
  CDRAGON,
  TENCENT_PROFILE_ICON_CDN,
  CATEGORY_CDRAGON_FILE,
  dataRoot,
  cdragonDataUrl,
  defaultPatchForCategory,
} from "./sources/cdragon.js";

export {
  getGameVersion,
  getVersionInfo,
  DDRAGON_REALM_TENCENT,
  DDRAGON_REALM_NA,
} from "./sources/ddragon.js";

export {
  DEFAULT_CDN_BASE,
  DEFAULT_RELEASE_BASE,
  GITHUB_RELEASE_BASE,
  releaseBaseUrl,
  releaseDataUrl,
  releaseManifestUrl,
  releaseAssetName,
  githubReleaseDataUrl,
} from "./sources/release.js";

export {
  assetUrl,
  rewritePathsDeep,
  transformSummonerIcons,
  transformSummonerIconSets,
  transformSummonerEmotes,
  transformWardSkins,
  transformWardSkinSets,
  transformLoot,
  filterLootChests,
  transformChampions,
  transformSkins,
  transformSkinlines,
  transformUniverses,
  computeAdded,
} from "./transforms/index.js";

export { splitSkinId, splitId } from "./helpers/split-skin-id.js";
export type { SplitSkinId } from "./helpers/split-skin-id.js";
export { substitute, ALIAS_SUBSTITUTIONS } from "./helpers/substitute.js";
export {
  getRarityUrl,
  modelviewerUrl,
  raritiesMap,
} from "./helpers/rarity.js";

export type * from "./types/index.js";
export { RarityEnum, SkinType } from "./types/skins.js";
