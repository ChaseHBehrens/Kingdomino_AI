#!/usr/bin/env bash
# Builds the browser demo into one self-contained file, dist/kingdomino.html,
# with the engine embedded so it runs offline straight from disk
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release --target wasm32-unknown-unknown
mkdir -p dist
python3 - <<'PY'
import base64
wasm = open("target/wasm32-unknown-unknown/release/kingdomino_web.wasm", "rb").read()
page = open("index.html").read().replace("__WASM_BASE64__", base64.b64encode(wasm).decode())
open("dist/kingdomino.html", "w").write(page)
PY
echo "Built $(pwd)/dist/kingdomino.html"
