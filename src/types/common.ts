/** CommunityDragon language folder name (e.g. default, zh_cn). */
export type Lang = string;

/** CDragon patch segment: latest | pbe | 15.24 | 15.24.1 */
export type Patch = "latest" | "pbe" | (string & {});

export type AssetCategory =
  | "summoner-icons"
  | "summoner-icon-sets"
  | "summoner-emotes"
  | "ward-skins"
  | "ward-skin-sets"
  | "loot"
  | "champions"
  | "skinlines"
  | "universes"
  | "skins"
  | "added"
  | "version";

export type CosmeticsCategory =
  | "summoner-icons"
  | "summoner-icon-sets"
  | "summoner-emotes"
  | "ward-skins"
  | "ward-skin-sets"
  | "loot";

export type SkinsDomainCategory =
  | "champions"
  | "skinlines"
  | "universes"
  | "skins";

export interface RegionalDescription {
  region: string;
  description: string;
}

export interface RegionalRarity {
  region: string;
  rarity: number;
}
