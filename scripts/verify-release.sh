#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pushd "$ROOT_DIR" >/dev/null

pnpm install --frozen-lockfile
pnpm lint:all
pnpm test:all
cargo test --workspace

# Optional: run wasm smoke test if node available
if command -v node >/dev/null 2>&1; then
  node tests/benchmarks/wasm_perf.js || true
fi

popd >/dev/null
