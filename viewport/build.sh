wasm-pack build entry --target=web --no-typescript
echo -e "*.wasm\n*.js\npackage.json\n.netlify" > entry/pkg/.gitignore
