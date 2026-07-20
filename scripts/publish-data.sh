#!/usr/bin/env bash
set -euo pipefail

workDir="$(cd "$(dirname "$0")/.." && pwd)"
cd "$workDir"

log() { echo "[publish-data::] $1"; }

rm -f "$workDir/SKIP"
npm run build:data -- "$@"

if [ -f "$workDir/SKIP" ]; then
  log "Version not change, skip release"
  exit 0
fi

version="$(tr -d '[:space:]' < VERSION)"
tag="data-v${version}"

if gh release view "$tag" >/dev/null 2>&1; then
  log "Release $tag already exists, skip create"
  exit 0
fi

log "Creating GitHub Release $tag"
mkdir -p data/release-assets
rm -rf data/release-assets/*
tar -C data/out -czf "data/release-assets/league-fan-assets-data-${version}.tar.gz" .

# Flatten paths: default/summoner-icons.json → default__summoner-icons.json
while IFS= read -r -d '' f; do
  rel="${f#data/out/}"
  asset_name="${rel//\//__}"
  cp "$f" "data/release-assets/${asset_name}"
done < <(find data/out -type f -name '*.json' -print0)

gh release create "$tag" \
  --title "Data $version" \
  --notes "Prebuilt LoL asset JSON for @magicwenli/league-fan-assets (game version ${version}).

Asset names are flattened with \`__\` for GitHub Releases:
- \`default__summoner-icons.json\`
- \`zh_cn__skins.json\`
- \`manifest.json\`, \`version.json\`

Also attached: \`league-fan-assets-data-${version}.tar.gz\` (nested paths preserved).

Mirrored to Cloudflare R2 CDN:
https://league-fan-data.yxra3603.workers.dev/latest/" \
  data/release-assets/*

log "Release $tag published"

if [ -n "${CLOUDFLARE_API_TOKEN:-}" ] || [ -n "${CLOUDFLARE_ACCOUNT_ID:-}" ] || command -v wrangler >/dev/null 2>&1; then
  log "Syncing to R2…"
  chmod +x scripts/sync-r2.sh
  ./scripts/sync-r2.sh || log "WARN: R2 sync failed (non-fatal for local publish)"
else
  log "Skip R2 sync (no wrangler / CLOUDFLARE_API_TOKEN)"
fi

exit 0
