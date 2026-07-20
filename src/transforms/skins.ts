import type { Skin, Skins } from "../types/skins.js";
import type { Skinline } from "../types/skinline.js";
import type { Universe } from "../types/universe.js";
import type { Lang, Patch } from "../types/common.js";
import { assetUrl, rewritePathsDeep } from "./paths.js";

export interface SkinTransformOpts {
  lang?: Lang;
  patch?: Patch;
  /** Prefix base skins with "Original " (default true for default/en). */
  originalPrefix?: boolean;
}

const PATH_KEYS = [
  "splashPath",
  "uncenteredSplashPath",
  "tilePath",
  "loadScreenPath",
  "loadScreenVintagePath",
  "chromaPath",
  "splashVideoPath",
  "collectionSplashVideoPath",
  "collectionCardHoverVideoPath",
  "rarityGemPath",
] as const;

function absolutizeSkinPaths(
  skin: Skin,
  opts: { patch: Patch; lang?: Lang },
): Skin {
  const next = { ...skin } as Skin;
  for (const key of PATH_KEYS) {
    const val = next[key];
    if (typeof val === "string" && val) {
      (next as Record<string, unknown>)[key] = assetUrl(val, {
        patch: opts.patch,
        lang: opts.lang,
        category: "skins",
      });
    }
  }
  if (next.chromas?.length) {
    next.chromas = next.chromas.map((c) => ({
      ...c,
      chromaPath: assetUrl(c.chromaPath, {
        patch: opts.patch,
        lang: opts.lang,
        category: "skins",
      }),
    }));
  }
  if (next.emblems?.length) {
    next.emblems = next.emblems.map((e) => ({
      ...e,
      emblemPath: {
        large: assetUrl(e.emblemPath?.large, {
          patch: opts.patch,
          lang: opts.lang,
          category: "skins",
        }),
        small: assetUrl(e.emblemPath?.small, {
          patch: opts.patch,
          lang: opts.lang,
          category: "skins",
        }),
      },
    }));
  }
  // Augment / border paths
  if (next.skinAugments) {
    next.skinAugments = rewritePathsDeep(next.skinAugments, {
      patch: opts.patch,
      lang: opts.lang,
      category: "skins",
    });
  }
  return next;
}

/**
 * Expand quest skin tiers into individual skin entries and normalize paths.
 * Port of feat-nextjs scraper getLatestSkins.
 */
export function transformSkins(
  raw: Skins | Record<string, Skin>,
  opts: SkinTransformOpts = {},
): Skins {
  const patch = opts.patch ?? "pbe";
  const originalPrefix = opts.originalPrefix ?? true;
  const data: Skins = { ...raw };

  for (const id of Object.keys(data)) {
    const skin = data[id];
    if (!skin) continue;

    if (skin.isBase && originalPrefix && !skin.name.startsWith("Original ")) {
      skin.name = "Original " + skin.name;
    }

    if (skin.questSkinInfo?.tiers?.length) {
      const base = { ...skin };
      delete base.questSkinInfo;
      for (const tier of skin.questSkinInfo.tiers) {
        const merged = { ...base, ...tier } as Skin;
        data[String(merged.id)] = absolutizeSkinPaths(merged, {
          patch,
          lang: opts.lang,
        });
      }
    }

    data[id] = absolutizeSkinPaths(skin, { patch, lang: opts.lang });
  }

  return data;
}

export function transformSkinlines(
  raw: unknown[],
  _opts: SkinTransformOpts = {},
): Skinline[] {
  return (raw as Skinline[])
    .filter((d) => d.id !== 0)
    .map((d) => ({
      id: d.id,
      name: d.name ?? "",
      description: d.description ?? "",
    }))
    .sort((a, b) => (a.name > b.name ? 1 : -1));
}

export function transformUniverses(
  raw: unknown[],
  opts: SkinTransformOpts = {},
): Universe[] {
  const patch = opts.patch ?? "pbe";
  return (raw as Partial<Universe>[])
    .filter((d) => d.id !== 0 && d.id != null)
    .map((d) => ({
      id: Number(d.id),
      name: d.name ?? "",
      description: d.description ?? "",
      imagePath: assetUrl(d.imagePath ?? "", {
        patch,
        lang: opts.lang,
        category: "universes",
      }),
      skinSets: d.skinSets ?? [],
    }))
    .sort((a, b) => (a.name > b.name ? 1 : -1));
}
