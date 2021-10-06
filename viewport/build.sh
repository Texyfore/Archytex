wasm-pack build entry --target=web --no-typescript
echo -e "*.wasm\n*.js\npackage.json" > entry/pkg/.gitignore