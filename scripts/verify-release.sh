#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

pushd "$ROOT_DIR" >/dev/null

pnpm install --frozen-lockfile
pnpm lint:all
pnpm test:all
cargo test --workspace

# Optional: run wasm smoke test if node available
# Smoke-test WASM throughput (regression only; ignore failures in CI)
if command -v node >/dev/null 2>&1; then
  node tests/benchmarks/wasm_perf.js || true
fi

# Ensure release artifacts exist
test -d dist-release || {
  echo "dist-release directory missing; run pnpm release:build" >&2
  exit 1
}

popd >/dev/null
