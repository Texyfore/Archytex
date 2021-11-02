wasm-pack build entry --target=web --no-typescript --release
cp -r entry/pkg out
cp index.html out/index.html