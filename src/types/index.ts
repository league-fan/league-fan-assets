export type {
  Lang,
  Patch,
  AssetCategory,
  CosmeticsCategory,
  SkinsDomainCategory,
  RegionalDescription,
  RegionalRarity,
} from "./common.js";

export type {
  SummonerIcon,
  SummonerIconSet,
  SummonerEmote,
  WardSkin,
  WardSkinSet,
  LootItem,
} from "./cosmetics.js";

export type { Champion, Role } from "./champion.js";
export type { Skinline } from "./skinline.js";
export type { Universe } from "./universe.js";

export type {
  Skins,
  Skin,
  Emblem,
  Chroma,
  SkinAugments,
  Augment,
  Overlay,
  Borders,
  Layer,
  QuestSkinInfo,
  DescriptionInfo,
  Tier,
  SkinLineRef,
} from "./skins.js";
export { RarityEnum, SkinType } from "./skins.js";

export type { Added } from "./added.js";
export type { VersionInfo, DataManifest } from "./version.js";

import type {
  SummonerIcon,
  SummonerIconSet,
  SummonerEmote,
  WardSkin,
  WardSkinSet,
  LootItem,
} from "./cosmetics.js";
import type { Champion } from "./champion.js";
import type { Skinline } from "./skinline.js";
import type { Universe } from "./universe.js";
import type { Skins } from "./skins.js";
import type { Added } from "./added.js";
import type { VersionInfo } from "./version.js";
import type { AssetCategory } from "./common.js";

export type CategoryDataMap = {
  "summoner-icons": SummonerIcon[];
  "summoner-icon-sets": SummonerIconSet[];
  "summoner-emotes": SummonerEmote[];
  "ward-skins": WardSkin[];
  "ward-skin-sets": WardSkinSet[];
  loot: LootItem[];
  champions: Champion[];
  skinlines: Skinline[];
  universes: Universe[];
  skins: Skins;
  added: Added;
  version: VersionInfo;
};

export type CategoryData<C extends AssetCategory> = CategoryDataMap[C];
