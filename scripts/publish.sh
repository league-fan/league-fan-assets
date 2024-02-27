#!/usr/bin/env bash
npm=$(command -v npm)
workDir=$(pwd)

publish() {
  if [ -d "$workDir/save" ]; then
    cp ".npmrc" "$workDir/save/.npmrc"
    cd "$workDir/save" || return
    log "ready to publish"
    $npm publish --access public
  else
    log "$workDir/save not exists"
  fi
}

log() {
  echo "[publish::] $1"
}

log "pwd is $workDir"

rm -rf "$workDir/SKIP"
node "$workDir/index.js"

if [ -f "$workDir/SKIP" ]; then
  log "Version not change, skip publish"
  exit 0
fi

publish

if [ $? -eq 0 ]; then
  log "publish success"
  git add $workDir/VERSION
  git commit -m "chore: publish version $(cat VERSION)"
  git push
else
  log "publish failed"
  exit 1
fi
