if [ "$#" -eq 0 ]; then
    echo "No arguments supplied"
else 
for var in "$@"; do
        cd games/$var
        RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo +stable build --bin $var --target wasm32-unknown-unknown --profile wasm-release
        cp target/wasm32-unknown-unknown/wasm-release/$var.wasm ../wasm/$var.wasm
        wasm-bindgen --no-typescript --target web \
            --out-dir ../wasm/ \
            --out-name "$var" \
            ../wasm/$var.wasm
    done
fi
