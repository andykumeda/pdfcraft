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
if /usr/bin/grep -q PDFCRAFT_PROFILE_DIR "$APP/Contents/MacOS/PDFCraft"; then exit 1; fi
/usr/bin/grep -q -- --disable-updates "$APP/Contents/MacOS/PDFCraft"
[[ "$("$APP/Contents/MacOS/PDFCraft" --version)" = "pdfcraft 0.18.2 (${2:0:8})" ]]
MODELS="$APP/Contents/Resources/models"
[[ -s "$MODELS/ATTRIBUTION.txt" && -s "$MODELS/text-detection.onnx.LICENCE.txt" && -s "$MODELS/text-recognition.onnx.LICENCE.txt" ]]
[[ "$(shasum -a 256 "$MODELS/text-detection.onnx" | cut -d ' ' -f1)" = a917b23dbd9524b465df7e922641b3ff2981623df4ded5a0234004ef2fee7cfe ]]
[[ "$(shasum -a 256 "$MODELS/text-recognition.onnx" | cut -d ' ' -f1)" = 86c145c2edb96c8caed5b1ebb8f44d706408922211451c309c625157dd6061c5 ]]
codesign --verify --strict --deep "$APP"
echo 'Standard PDFCraft archive checks passed.'
