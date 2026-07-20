import type { AssetCategory, Lang, Patch } from "../types/common.js";

export const CDRAGON = "https://raw.communitydragon.org";

export const TENCENT_PROFILE_ICON_CDN =
  "https://dlied1.qq.com/lolapp/lol/summoner/profileicon/";

/** CDragon filename for a category (under global/{lang}/v1/). */
export const CATEGORY_CDRAGON_FILE: Partial<Record<AssetCategory, string>> = {
  "summoner-icons": "summoner-icons.json",
  "summoner-icon-sets": "summoner-icon-sets.json",
  "summoner-emotes": "summoner-emotes.json",
  "ward-skins": "ward-skins.json",
  "ward-skin-sets": "ward-skin-sets.json",
  loot: "loot.json",
  champions: "champion-summary.json",
  skinlines: "skinlines.json",
  universes: "universes.json",
  skins: "skins.json",
};

export function dataRoot(opts: { patch?: Patch; lang?: Lang } = {}): string {
  const patch = opts.patch ?? "latest";
  const lang = opts.lang ?? "default";
  return `${CDRAGON}/${patch}/plugins/rcp-be-lol-game-data/global/${lang}`;
}

export function cdragonDataUrl(opts: {
  category: AssetCategory;
  lang?: Lang;
  patch?: Patch;
}): string {
  const file = CATEGORY_CDRAGON_FILE[opts.category];
  if (!file) {
    throw new Error(
      `Category "${opts.category}" is not loaded from CDragon v1 JSON`,
    );
  }
  return `${dataRoot({ patch: opts.patch, lang: opts.lang })}/v1/${file}`;
}

/** Default patch preference by domain when using live CDragon. */
export function defaultPatchForCategory(category: AssetCategory): Patch {
  switch (category) {
    case "champions":
    case "skinlines":
    case "universes":
    case "skins":
    case "added":
      return "pbe";
    default:
      return "latest";
  }
}
