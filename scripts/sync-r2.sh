#!/usr/bin/env bash
# Sync data/out JSON snapshots to R2 (bucket: league-fan, prefix: meta/).
#
# Layout (flat names match GitHub Release / releaseDataUrl):
#   meta/latest/zh_cn__skins.json
#   meta/v/{gameVersion}/zh_cn__skins.json
#
# Usage:
#   ./scripts/sync-r2.sh                 # uses VERSION file + data/out
#   VERSION=16.14.1 ./scripts/sync-r2.sh
#   R2_BUCKET=league-fan ./scripts/sync-r2.sh
#
# Requires: wrangler authenticated (local OAuth or CLOUDFLARE_API_TOKEN in CI)

set -euo pipefail

workDir="$(cd "$(dirname "$0")/.." && pwd)"
cd "$workDir"

BUCKET="${R2_BUCKET:-league-fan}"
OUT_DIR="${DATA_OUT:-data/out}"
version="${VERSION:-}"
if [ -z "$version" ] && [ -f VERSION ]; then
  version="$(tr -d '[:space:]' < VERSION)"
fi
if [ -z "$version" ]; then
  echo "[sync-r2] ERROR: VERSION not set and VERSION file missing" >&2
  exit 1
fi

if [ ! -d "$OUT_DIR" ]; then
  echo "[sync-r2] ERROR: missing $OUT_DIR — run npm run build:data first" >&2
  exit 1
fi

log() { echo "[sync-r2] $1"; }

upload_one() {
  local src="$1"
  local key="$2"
  local cache_control
  case "$key" in
    meta/v/*) cache_control="public, max-age=31536000, immutable" ;;
    *) cache_control="public, max-age=300" ;;
  esac
  npx wrangler r2 object put "${BUCKET}/${key}" \
    --file "$src" \
    --content-type "application/json; charset=utf-8" \
    --cache-control "$cache_control" \
    --remote \
    >/dev/null
  log "  put $key"
}

count=0
while IFS= read -r -d '' f; do
  rel="${f#"$OUT_DIR"/}"
  flat="${rel//\//__}"
  upload_one "$f" "meta/latest/${flat}"
  upload_one "$f" "meta/v/${version}/${flat}"
  count=$((count + 1))
done < <(find "$OUT_DIR" -type f -name '*.json' -print0)

if [ "$count" -eq 0 ]; then
  echo "[sync-r2] ERROR: no JSON under $OUT_DIR" >&2
  exit 1
fi

log "Synced $count files → r2://$BUCKET/meta/latest and meta/v/$version"
log "CDN: https://league-fan-data.yxra3603.workers.dev/latest/manifest.json"
