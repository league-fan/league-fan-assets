import type { AssetCategory } from "./types/common.js";

export const ASSET_CATEGORIES = [
  "summoner-icons",
  "summoner-icon-sets",
  "summoner-emotes",
  "ward-skins",
  "ward-skin-sets",
  "loot",
  "champions",
  "skinlines",
  "universes",
  "skins",
  "added",
  "version",
] as const satisfies readonly AssetCategory[];

export const PUBLISHED_DATA_CATEGORIES = [
  "summoner-icons",
  "summoner-icon-sets",
  "summoner-emotes",
  "ward-skins",
  "ward-skin-sets",
  "loot",
  "champions",
  "skinlines",
  "universes",
  "skins",
  "added",
] as const;

export const DEFAULT_DATA_LANGUAGES = ["default", "zh_cn"] as const;

export interface CategoryMeta {
  id: AssetCategory;
  domain: "cosmetics" | "champions" | "skins" | "meta";
  description: string;
}

export const CATEGORY_META: Record<AssetCategory, CategoryMeta> = {
  "summoner-icons": {
    id: "summoner-icons",
    domain: "cosmetics",
    description: "Summoner profile icons",
  },
  "summoner-icon-sets": {
    id: "summoner-icon-sets",
    domain: "cosmetics",
    description: "Summoner icon sets",
  },
  "summoner-emotes": {
    id: "summoner-emotes",
    domain: "cosmetics",
    description: "Summoner emotes",
  },
  "ward-skins": {
    id: "ward-skins",
    domain: "cosmetics",
    description: "Ward skins",
  },
  "ward-skin-sets": {
    id: "ward-skin-sets",
    domain: "cosmetics",
    description: "Ward skin sets",
  },
  loot: {
    id: "loot",
    domain: "cosmetics",
    description: "Loot chests (filtered)",
  },
  champions: {
    id: "champions",
    domain: "champions",
    description: "Champion summary list",
  },
  skinlines: {
    id: "skinlines",
    domain: "skins",
    description: "Skin lines / thematics",
  },
  universes: {
    id: "universes",
    domain: "skins",
    description: "Universes / IP sets",
  },
  skins: {
    id: "skins",
    domain: "skins",
    description: "Full skins map with chromas / quest tiers",
  },
  added: {
    id: "added",
    domain: "meta",
    description: "IDs added vs previous patch snapshot",
  },
  version: {
    id: "version",
    domain: "meta",
    description: "Data dragon realm / game version",
  },
};
