# WASM Setup Guide

## Prerequisites

To build the WASM modules for this project, you need to install `wasm-pack`:

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Or via cargo
cargo install wasm-pack
```

## Building WASM

To build the WASM module:

```bash
# From the project root
cd crates/fmd-wasm
wasm-pack build --target web --out-dir ../../packages/faster-md/wasm

# For optimized production build
wasm-pack build --target web --out-dir ../../packages/faster-md/wasm --release
```

## wasm-opt

For additional optimization, install `wasm-opt` from the Binaryen toolkit:

```bash
# macOS
brew install binaryen

# Linux/WSL
npm install -g wasm-opt
```

Then optimize the WASM file:

```bash
wasm-opt -Oz packages/faster-md/wasm/fmd_wasm_bg.wasm -o packages/faster-md/wasm/fmd_wasm_bg.wasm
```