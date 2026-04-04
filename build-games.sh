#!/usr/bin/env bash
set -euo pipefail

SCRIPT_FILE="${BASH_SOURCE[0]}"
SCRIPT_FOLDER="$(dirname -- "$SCRIPT_FILE")"
PROJECT_ROOT="$(cd -- "$SCRIPT_FOLDER" && pwd)"

WASM_OUT_DIR="$PROJECT_ROOT/site/public/arcade"

if [ "$#" -eq 0 ]; then
    echo "No arguments supplied"
    exit 1
fi

for var in "$@"; do
    (
        RUSTFLAGS='--cfg getrandom_backend="wasm_js"' \
            cargo build --bin "$var" \
            --target wasm32-unknown-unknown \
            --profile wasm-release

        cp "target/wasm32-unknown-unknown/wasm-release/$var.wasm" \
            "$WASM_OUT_DIR/$var.wasm"

        wasm-bindgen \
            --no-typescript \
            --target web \
            --out-dir "$WASM_OUT_DIR" \
            --out-name "$var" \
            "$WASM_OUT_DIR/$var.wasm"
    )
done
