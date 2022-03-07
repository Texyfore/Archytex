cd packages/web-runner
set "RUSTFLAGS=-C target-feature=+atomics,+bulk-memory"
wasm-pack build --dev --target=web --out-dir ../../express/public/pkg . -Z build-std=panic_abort,std
cd ..