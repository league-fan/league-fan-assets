import type { AssetCategory, Lang, Patch } from "../types/common.js";
import type { CategoryDataMap } from "../types/index.js";
import type { Skins } from "../types/skins.js";
import {
  cdragonDataUrl,
  defaultPatchForCategory,
} from "../sources/cdragon.js";
import { releaseDataUrl } from "../sources/release.js";
import { getVersionInfo } from "../sources/ddragon.js";
import { fetchJson } from "./http.js";
import {
  transformChampions,
  transformLoot,
  transformSkinlines,
  transformSkins,
  transformSummonerEmotes,
  transformSummonerIconSets,
  transformSummonerIcons,
  transformUniverses,
  transformWardSkinSets,
  transformWardSkins,
} from "../transforms/index.js";

export type DataSource =
  | { kind: "cdragon"; patch?: Patch }
  | { kind: "release"; baseUrl?: string; version?: string };

export interface LoadOptions {
  lang?: Lang;
  source?: DataSource;
  fetch?: typeof globalThis.fetch;
  /** Override patch for transforms when source is release. */
  patch?: Patch;
}

function resolvePatch(
  category: AssetCategory,
  source: DataSource,
  explicit?: Patch,
): Patch {
  if (explicit) return explicit;
  if (source.kind === "cdragon") {
    return source.patch ?? defaultPatchForCategory(category);
  }
  // Release snapshots are version-pinned; image CDN uses latest by default
  return defaultPatchForCategory(category);
}

export async function loadCategory<C extends AssetCategory>(
  category: C,
  opts: LoadOptions = {},
): Promise<CategoryDataMap[C]> {
  const lang = opts.lang ?? "default";
  const source: DataSource = opts.source ?? { kind: "cdragon" };
  const patch = resolvePatch(category, source, opts.patch);
  const fetchOpt = { fetch: opts.fetch };

  if (category === "version") {
    if (source.kind === "release") {
      const url = releaseDataUrl({
        category: "version",
        baseUrl: source.baseUrl,
        version: source.version,
      });
      return (await fetchJson(url, fetchOpt)) as CategoryDataMap[C];
    }
    return (await getVersionInfo(fetchOpt)) as CategoryDataMap[C];
  }

  if (category === "added") {
    if (source.kind === "release") {
      const url = releaseDataUrl({
        category: "added",
        baseUrl: source.baseUrl,
        version: source.version,
      });
      return (await fetchJson(url, fetchOpt)) as CategoryDataMap[C];
    }
    throw new Error(
      'Category "added" requires source: { kind: "release" } or precomputed data from build-data',
    );
  }

  let raw: unknown;
  if (source.kind === "release") {
    const url = releaseDataUrl({
      category,
      lang,
      baseUrl: source.baseUrl,
      version: source.version,
    });
    raw = await fetchJson(url, fetchOpt);
    // Release JSON is already transformed; return as-is
    return raw as CategoryDataMap[C];
  }

  const url = cdragonDataUrl({ category, lang, patch });
  raw = await fetchJson(url, fetchOpt);

  const tOpts = { lang, patch };
  switch (category) {
    case "summoner-icons":
      return transformSummonerIcons(raw as unknown[], tOpts) as CategoryDataMap[C];
    case "summoner-icon-sets":
      return transformSummonerIconSets(raw, tOpts) as CategoryDataMap[C];
    case "summoner-emotes":
      return transformSummonerEmotes(raw as unknown[], tOpts) as CategoryDataMap[C];
    case "ward-skins":
      return transformWardSkins(raw as unknown[], tOpts) as CategoryDataMap[C];
    case "ward-skin-sets":
      return transformWardSkinSets(raw, tOpts) as CategoryDataMap[C];
    case "loot":
      return transformLoot(raw, tOpts) as CategoryDataMap[C];
    case "champions":
      return transformChampions(raw as unknown[], tOpts) as CategoryDataMap[C];
    case "skinlines":
      return transformSkinlines(raw as unknown[], tOpts) as CategoryDataMap[C];
    case "universes":
      return transformUniverses(raw as unknown[], tOpts) as CategoryDataMap[C];
    case "skins":
      return transformSkins(raw as Skins, tOpts) as CategoryDataMap[C];
    default:
      throw new Error(`Unsupported category: ${category}`);
  }
}
