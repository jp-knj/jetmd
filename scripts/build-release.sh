#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist-release"
WASM_OUT="$DIST_DIR/wasm"

rm -rf "$DIST_DIR"
mkdir -p "$WASM_OUT"

pushd "$ROOT_DIR" >/dev/null

# 1. Build Rust workspace
cargo build --workspace --release

# 2. Build WASM package
cargo build --package fmd-wasm --release --target wasm32-unknown-unknown
wasm-bindgen --target bundler --out-dir "$WASM_OUT" \
  --typescript target/wasm32-unknown-unknown/release/fmd_wasm.wasm

# Optimize WASM output if wasm-opt exists
if command -v wasm-opt >/dev/null 2>&1; then
  wasm-opt -O3 "$WASM_OUT/fmd_wasm_bg.wasm" -o "$WASM_OUT/fmd_wasm_bg.wasm"
fi

# 3. Copy wasm artifacts into packages/wasm
mkdir -p "$ROOT_DIR/packages/wasm"
cp "$WASM_OUT"/fmd_wasm* "$ROOT_DIR/packages/wasm/"

# 4. Build TypeScript packages
pnpm -r --filter ./packages --if-present build

# 5. Generate type definitions for wasm package
npx --yes dts-bundle-generator -o "$ROOT_DIR/packages/wasm/fmd_wasm.d.ts" \
  "$ROOT_DIR/packages/faster-md/src/index.ts"

# 6. Bundle faster-md for browser usage
pnpm --filter faster-md run build
rollup -c rollup.config.release.mjs

# 7. Create summary
printf "Release artifacts staged in %s\n" "$DIST_DIR"

popd >/dev/null
