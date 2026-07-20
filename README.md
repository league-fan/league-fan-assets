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

### B) GitHub Releases (prebuilt)

Daily CI builds normalized JSON and publishes `data-v{version}` releases.

Asset names are **flattened** (GitHub Releases are flat):

| Logical path | Release asset |
|--------------|---------------|
| `default/summoner-icons.json` | `default__summoner-icons.json` |
| `zh_cn/skins.json` | `zh_cn__skins.json` |
| `manifest.json` | `manifest.json` |

```
https://github.com/league-fan/league-fan-assets/releases/latest/download/default__champions.json
```

Also attached: `league-fan-assets-data-{version}.tar.gz` with nested folders.

```ts
import { releaseDataUrl } from "@magicwenli/league-fan-assets";

releaseDataUrl({ category: "skins", lang: "zh_cn", version: "15.24.1" });
```

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
