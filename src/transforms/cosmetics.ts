import type {
  LootItem,
  SummonerEmote,
  SummonerIcon,
  SummonerIconSet,
  WardSkin,
  WardSkinSet,
} from "../types/cosmetics.js";
import type { Lang, Patch } from "../types/common.js";
import { assetUrl, rewritePathsDeep } from "./paths.js";
import { filterLootChests } from "./loot.js";

export interface TransformOpts {
  lang?: Lang;
  patch?: Patch;
}

function flattenDescription(
  item: { descriptions?: { description?: string }[]; description?: string },
): string {
  if (item.descriptions?.[0]?.description) {
    return item.descriptions[0].description;
  }
  return item.description ?? "";
}

export function transformSummonerIcons(
  raw: unknown[],
  opts: TransformOpts = {},
): SummonerIcon[] {
  const patch = opts.patch ?? "latest";
  const lang = opts.lang ?? "default";
  const list = (raw as Partial<SummonerIcon>[]).map((item) => {
    const imagePath = assetUrl(item.imagePath ?? "", {
      patch,
      lang,
      category: "summoner-icons",
    });
    return {
      ...item,
      id: Number(item.id),
      title: item.title ?? "",
      imagePath,
      description: flattenDescription(item as SummonerIcon),
    } as SummonerIcon;
  });
  return list
    .filter((i) => i.imagePath)
    .sort((a, b) => b.id - a.id);
}

export function transformSummonerIconSets(
  raw: unknown,
  opts: TransformOpts = {},
): SummonerIconSet[] {
  const data = Array.isArray(raw) ? raw : [];
  return rewritePathsDeep(data, {
    patch: opts.patch ?? "latest",
    lang: opts.lang,
    category: "summoner-icon-sets",
  }) as SummonerIconSet[];
}

export function transformSummonerEmotes(
  raw: unknown[],
  opts: TransformOpts = {},
): SummonerEmote[] {
  const patch = opts.patch ?? "latest";
  return (raw as Partial<SummonerEmote>[]).map((item) => ({
    ...item,
    id: Number(item.id),
    name: item.name ?? "",
    description: item.description ?? "",
    inventoryIcon: assetUrl(item.inventoryIcon ?? "", {
      patch,
      lang: opts.lang,
      category: "summoner-emotes",
    }),
  })) as SummonerEmote[];
}

export function transformWardSkins(
  raw: unknown[],
  opts: TransformOpts = {},
): WardSkin[] {
  const patch = opts.patch ?? "latest";
  return (raw as Partial<WardSkin>[]).map((item) => ({
    ...item,
    id: Number(item.id),
    name: item.name ?? "",
    description: item.description ?? "",
    wardImagePath: assetUrl(item.wardImagePath ?? "", {
      patch,
      lang: opts.lang,
      category: "ward-skins",
    }),
    wardShadowImagePath: assetUrl(item.wardShadowImagePath ?? "", {
      patch,
      lang: opts.lang,
      category: "ward-skins",
    }),
  })) as WardSkin[];
}

export function transformWardSkinSets(
  raw: unknown,
  opts: TransformOpts = {},
): WardSkinSet[] {
  const data = Array.isArray(raw) ? raw : [];
  return rewritePathsDeep(data, {
    patch: opts.patch ?? "latest",
    lang: opts.lang,
    category: "ward-skin-sets",
  }) as WardSkinSet[];
}

export function transformLoot(
  raw: unknown,
  opts: TransformOpts = {},
): LootItem[] {
  return filterLootChests(raw, opts);
}
