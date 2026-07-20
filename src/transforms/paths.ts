import type { AssetCategory, Lang, Patch } from "../types/common.js";
import {
  dataRoot,
  TENCENT_PROFILE_ICON_CDN,
} from "../sources/cdragon.js";

const GAME_DATA_PREFIX = "/lol-game-data/assets";

export interface AssetUrlOptions {
  patch?: Patch;
  /** Text language; image binaries still resolve under global/default. */
  lang?: Lang;
  category?: AssetCategory;
}

/**
 * Resolve a CDragon game-data path (or passthrough absolute URL) to HTTPS.
 * Mirrors historical league-fan-assets + feat-nextjs behavior.
 */
export function assetUrl(
  path: string | null | undefined,
  opts: AssetUrlOptions = {},
): string {
  if (path == null || path === "") return "";
  if (/^https?:\/\//i.test(path)) return path;

  const patch = opts.patch ?? "latest";

  // zh_cn profile icons → Tencent CDN (legacy package behavior)
  if (
    opts.category === "summoner-icons" &&
    (opts.lang === "zh_cn" || opts.lang === "zh_CN")
  ) {
    const m = path.match(
      /\/lol-game-data\/assets\/v1\/profile-icons\/([0-9]+\.(?:jpg|png))/i,
    );
    if (m?.[1]) {
      return `${TENCENT_PROFILE_ICON_CDN}${m[1].toLowerCase()}`;
    }
  }

  if (path.startsWith(GAME_DATA_PREFIX)) {
    const rest = path.slice(GAME_DATA_PREFIX.length).replace(/^\/+/, "");
    // Image assets live under global/default regardless of text locale
    return `${dataRoot({ patch, lang: "default" })}/${rest.toLowerCase()}`;
  }

  // Relative-ish paths without prefix
  if (path.startsWith("/")) {
    return `${dataRoot({ patch, lang: "default" })}${path.toLowerCase()}`;
  }

  return path;
}

/** Deep-walk object/array and rewrite string fields that look like game-data paths. */
export function rewritePathsDeep<T>(
  value: T,
  opts: AssetUrlOptions = {},
): T {
  if (value == null) return value;
  if (typeof value === "string") {
    if (value.includes(GAME_DATA_PREFIX) || value.startsWith("/lol-game-data")) {
      return assetUrl(value, opts) as T;
    }
    return value;
  }
  if (Array.isArray(value)) {
    return value.map((v) => rewritePathsDeep(v, opts)) as T;
  }
  if (typeof value === "object") {
    const out: Record<string, unknown> = {};
    for (const [k, v] of Object.entries(value as Record<string, unknown>)) {
      out[k] = rewritePathsDeep(v, opts);
    }
    return out as T;
  }
  return value;
}
