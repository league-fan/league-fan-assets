import type { AssetCategory, Lang } from "../types/common.js";

/**
 * Browser-friendly CDN (Cloudflare Worker → R2).
 * Serves prebuilt JSON with CORS; paths use flat GH-Release asset names.
 *
 * Example: `${DEFAULT_CDN_BASE}/zh_cn__skins.json`
 */
export const DEFAULT_CDN_BASE =
  "https://league-fan-data.yxra3603.workers.dev/latest";

/**
 * GitHub Releases download base (latest data release).
 * Assets are flat: `{lang}__{category}.json` e.g. `zh_cn__summoner-icons.json`.
 * Note: GitHub release assets are **not** browser-CORS-friendly — prefer DEFAULT_CDN_BASE in browsers.
 */
export const GITHUB_RELEASE_BASE =
  "https://github.com/league-fan/league-fan-assets/releases/latest/download";

/** Default release source base — CDN (CORS-friendly). */
export const DEFAULT_RELEASE_BASE = DEFAULT_CDN_BASE;

function normalizeGameVersion(version: string): string {
  return version.replace(/^data-v/, "");
}

export function releaseBaseUrl(opts?: {
  baseUrl?: string;
  version?: string;
}): string {
  if (opts?.baseUrl) {
    return opts.baseUrl.replace(/\/$/, "");
  }
  if (opts?.version) {
    const gameVer = normalizeGameVersion(opts.version);
    return `https://league-fan-data.yxra3603.workers.dev/v/${gameVer}`;
  }
  return DEFAULT_CDN_BASE;
}

/** Flatten path for GitHub Release / CDN asset names. */
export function releaseAssetName(relPath: string): string {
  return relPath.replace(/\//g, "__");
}

export function releaseDataUrl(opts: {
  category: AssetCategory;
  lang?: Lang;
  version?: string;
  baseUrl?: string;
}): string {
  const base = releaseBaseUrl({
    baseUrl: opts.baseUrl,
    // only use version for base resolution when baseUrl is not set
    version: opts.baseUrl ? undefined : opts.version,
  });
  if (opts.category === "version") {
    return `${base}/${releaseAssetName("version.json")}`;
  }
  if (opts.category === "added") {
    return `${base}/${releaseAssetName("default/added.json")}`;
  }
  const lang = opts.lang ?? "default";
  return `${base}/${releaseAssetName(`${lang}/${opts.category}.json`)}`;
}

export function releaseManifestUrl(opts?: {
  baseUrl?: string;
  version?: string;
}): string {
  return `${releaseBaseUrl(opts)}/${releaseAssetName("manifest.json")}`;
}

/** GitHub release URL for a given game version (Node / non-browser). */
export function githubReleaseDataUrl(opts: {
  category: AssetCategory;
  lang?: Lang;
  version?: string;
}): string {
  const base = opts.version
    ? `https://github.com/league-fan/league-fan-assets/releases/download/data-v${normalizeGameVersion(opts.version)}`
    : GITHUB_RELEASE_BASE;
  return releaseDataUrl({
    category: opts.category,
    lang: opts.lang,
    baseUrl: base,
  });
}
