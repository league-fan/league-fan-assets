import type { RegionalDescription, RegionalRarity } from "./common.js";

export interface SummonerIcon {
  id: number;
  contentId?: string;
  title: string;
  yearReleased?: number;
  isLegacy?: boolean;
  /** Absolute HTTPS image URL after transform. */
  imagePath: string;
  /** Flattened primary description when available. */
  description: string;
  descriptions?: RegionalDescription[];
  rarities?: RegionalRarity[];
  disabledRegions?: string[];
}

export interface SummonerIconSet {
  id: number;
  name?: string;
  description?: string;
  iconIds?: number[];
  [key: string]: unknown;
}

export interface SummonerEmote {
  id: number;
  contentId?: string;
  name: string;
  description: string;
  /** Absolute HTTPS icon URL after transform. */
  inventoryIcon: string;
  taggedChampionsIds?: number[];
  [key: string]: unknown;
}

export interface WardSkin {
  id: number;
  name: string;
  description: string;
  wardImagePath: string;
  wardShadowImagePath: string;
  contentId?: string;
  isLegacy?: boolean;
  regionalDescriptions?: RegionalDescription[];
  rarities?: RegionalRarity[];
  [key: string]: unknown;
}

export interface WardSkinSet {
  id: number;
  name?: string;
  description?: string;
  [key: string]: unknown;
}

export interface LootItem {
  id: string;
  name: string;
  description: string;
  image: string;
  startDate?: string;
  endDate?: string;
  mappedStoreId?: number;
  lifetimeMax?: number;
  autoRedeem?: boolean;
  rarity?: string;
  type?: string;
  [key: string]: unknown;
}
