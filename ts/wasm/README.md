# ImageData

## Build
```bash
wasm-pack build --target web
```

## Optimize
```bash
wasm-opt -Oz -o ./pkg/file_inspector_bg.wasm ./pkg/file_inspector_bg.wasm
```