import type { AssetCategory, Lang } from "../types/common.js";

/**
 * Default GitHub Releases download base (latest data release).
 * Assets are flat: `{lang}__{category}.json` e.g. `zh_cn__summoner-icons.json`.
 */
export const DEFAULT_RELEASE_BASE =
  "https://github.com/league-fan/league-fan-assets/releases/latest/download";

export function releaseBaseUrl(opts?: {
  baseUrl?: string;
  version?: string;
}): string {
  if (opts?.baseUrl) {
    return opts.baseUrl.replace(/\/$/, "");
  }
  if (opts?.version) {
    const ver = opts.version.startsWith("data-v")
      ? opts.version
      : `data-v${opts.version}`;
    return `https://github.com/league-fan/league-fan-assets/releases/download/${ver}`;
  }
  return DEFAULT_RELEASE_BASE;
}

/** Flatten path for GitHub Release asset names. */
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
    version: opts.version,
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
