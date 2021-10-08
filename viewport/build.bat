wasm-pack build entry --target=web --no-typescript
(
    echo *.wasm
    echo *.js
    echo package.json
) > "entry/pkg/.gitignore"