build_wasm_game () {
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo +stable build --bin $1 --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --target web \
        --out-dir ./out/ \
        --out-name "$1" \
        ./target/wasm32-unknown-unknown/release/$1.wasm
}

build_wasm_game "snake"
