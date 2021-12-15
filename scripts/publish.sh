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

node "$workDir/index.js"

publish