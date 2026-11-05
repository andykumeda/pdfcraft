#!/usr/bin/env bash
# Verify a real standard artifact; rejects historical Preview archives.
set -euo pipefail
[[ $# = 2 ]] || { echo 'usage: test-standard-archive.sh ZIP SOURCE_SHA' >&2; exit 2; }
ROOT="$(mktemp -d)"
trap 'rm -rf "$ROOT"' EXIT
ditto -x -k "$1" "$ROOT"
APP="$ROOT/pdfcraft-0.18.2-macos-aarch64/PdfCraft.app"
[[ -d "$APP" ]]
PLIST="$APP/Contents/Info.plist"
[[ "$(plutil -extract CFBundleIdentifier raw -o - "$PLIST")" = ai.storyteller.pdfcraft ]]
[[ "$(plutil -extract CraftAssistantBridgeProtocol raw -o - "$PLIST")" = 1 ]]
[[ "$(plutil -extract CraftUnderlyingTag raw -o - "$PLIST")" = v0.18.2 ]]
[[ "$(plutil -extract CraftUnderlyingCommit raw -o - "$PLIST")" = 7faa4f35cefb0dea04137ec475bdc770945c22cd ]]
[[ "$(plutil -extract CraftUpdateRepository raw -o - "$PLIST")" = andykumeda/pdfcraft ]]
! /usr/bin/grep -q PDFCRAFT_PROFILE_DIR "$APP/Contents/MacOS/PDFCraft"
/usr/bin/grep -q -- --disable-updates "$APP/Contents/MacOS/PDFCraft"
[[ "$("$APP/Contents/MacOS/PDFCraft" --version)" = "pdfcraft 0.18.2 (${2:0:8})" ]]
codesign --verify --strict --deep "$APP"
echo 'Standard PDFCraft archive checks passed.'
