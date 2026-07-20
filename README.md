# @magicwenli/league-fan-assets

Typed TypeScript library for loading and normalizing League of Legends client assets for [league-fan](https://github.com/league-fan/league-fan.github.io).

> **v2 breaking change:** this package no longer ships multi‑MB JSON on npm.  
> The npm package is a **lightweight loader + types**. Prebuilt JSON snapshots are published as **GitHub Releases** (`data-v{gameVersion}`).

## Install

```bash
npm install @magicwenli/league-fan-assets
```

## Quick start

```ts
import { createClient, assetUrl, splitSkinId } from "@magicwenli/league-fan-assets";

// Live CommunityDragon
const live = createClient({
  lang: "zh_cn",
  source: { kind: "cdragon", patch: "latest" },
});

const icons = await live.summonerIcons();
const champions = await live.champions();
const skins = await live.skins();

// Prebuilt snapshot from GitHub Releases
const cached = createClient({
  lang: "default",
  source: { kind: "release" }, // latest data-v* release
});
```

## Asset categories

| Category | Description |
|----------|-------------|
| `summoner-icons` / `summoner-icon-sets` | Profile icons |
| `summoner-emotes` | Emotes |
| `ward-skins` / `ward-skin-sets` | Ward skins |
| `loot` | Chest loot (filtered) |
| `champions` | Champion summary |
| `skinlines` | Skin lines |
| `universes` | Universes |
| `skins` | Full skins map (quest tiers expanded) |
| `added` | Diff vs previous snapshot (release only) |
| `version` | Data Dragon realm version |

## Data backends

### A) CommunityDragon (live)

```ts
createClient({ source: { kind: "cdragon", patch: "pbe" } })
```

Fetches raw CDragon JSON and applies shared transforms (path rewrite, loot filter, quest skin expansion, …).

### B) Cloudflare CDN / GitHub Releases (prebuilt)

Daily CI builds normalized JSON, publishes `data-v{version}` **GitHub Releases**, and mirrors the same files to **Cloudflare R2** served by Worker `league-fan-data` (CORS-enabled).

**Browser (recommended):**

```ts
createClient({
  lang: "zh_cn",
  source: { kind: "release" }, // defaults to Cloudflare CDN
  // or pin: source: { kind: "release", version: "16.14.1" }
  // or override: source: { kind: "release", baseUrl: "https://..." }
});
```

CDN base: `https://league-fan-data.yxra3603.workers.dev/latest`

> **CORS note:** GitHub Release download URLs are **not** browser-CORS-friendly.
> Prefer the default CDN base (or any CORS proxy). Use `GITHUB_RELEASE_BASE` / `githubReleaseDataUrl()` only from Node.

Asset names are **flattened** (same on GH Releases and CDN):

| Logical path | Asset name |
|--------------|------------|
| `default/summoner-icons.json` | `default__summoner-icons.json` |
| `zh_cn/skins.json` | `zh_cn__skins.json` |
| `manifest.json` | `manifest.json` |

```
https://league-fan-data.yxra3603.workers.dev/latest/default__champions.json
https://github.com/league-fan/league-fan-assets/releases/latest/download/default__champions.json
```

Also attached on GH: `league-fan-assets-data-{version}.tar.gz` with nested folders.

```ts
import { releaseDataUrl, githubReleaseDataUrl } from "@magicwenli/league-fan-assets";

releaseDataUrl({ category: "skins", lang: "zh_cn", version: "16.14.1" });
// → CDN /v/16.14.1/zh_cn__skins.json
```

### Ops: data CDN

```bash
npm run build:data
npm run sync:r2          # upload meta/latest + meta/v/{VERSION} to R2
npm run deploy:cdn       # deploy workers/data-cdn
```

CI secrets for R2 sync: `CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_ACCOUNT_ID`.

## Helpers

- `assetUrl(path, { patch, lang, category })` — rewrite `/lol-game-data/assets/...`
- `splitSkinId(id)` — `{ champId, skinIndex }`
- `getRarityUrl(rarity)` / `modelviewerUrl(skinId)`
- `getGameVersion()` — Tencent DDragon realm version

## Scripts

```bash
npm test
npm run build          # library → dist/
npm run build:data     # snapshots → data/out/ (network)
npm run build:data -- --force
```

## Versioning

| Channel | Scheme | When |
|---------|--------|------|
| npm lib | Semver `2.x` | API changes (`v2.0.0` tag) |
| data | `data-v15.24.1` | Game version changes (cron) |

## Migration from v1 (unpkg JSON)

**Before:**

```ts
fetch("https://unpkg.com/@magicwenli/league-fan-assets/zh_cn/summoner-icons.json")
```

**After:**

```ts
const client = createClient({
  lang: "zh_cn",
  source: { kind: "release" }, // or cdragon
});
const icons = await client.summonerIcons();
```

Old v1 data-only npm versions remain on the registry for historical unpkg URLs but are no longer updated.

## License

MIT. Game assets © Riot Games. This project is not endorsed by Riot.
