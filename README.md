# ImageData

## Build
```bash
wasm-pack build --target web
```

## Optimize
```bash
wasm-opt -Oz -o ./pkg/image_data.wasm ./pkg/image_data.wasm
```