cargo build --target wasm32-unknown-unknown
mkdir -p web/app
wasm-bindgen --target web --out-dir web/app target/wasm32-unknown-unknown/debug/viewport.wasm