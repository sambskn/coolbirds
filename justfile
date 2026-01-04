dev:
    cargo run

wasm-build:
    trunk build --release true --minify true

wasm-run:
    wasmer run . --net

wasm-check:
    cargo c --target wasm32-unknown-unknown