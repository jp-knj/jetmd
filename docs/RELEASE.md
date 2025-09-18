# Release Guide

This document describes how to cut a JetMD release from the `001-phase-3.6-release` branch and publish packages to npm and crates.io.

## 1. Prerequisites

### Access Tokens
- **npm:** export a short–lived token before running release commands:
  ```bash
  export NPM_TOKEN="<scoped-publishing-token>"
  ```
- **crates.io:** store your API token in `~/.cargo/credentials` (or set `CARGO_REGISTRY_TOKEN`). Example credentials file:
  ```toml
  [registry]
  token = "<crates.io-api-token>"
  ```
  Avoid committing this file; keep it local or provide the token in CI secrets.

### Tooling
- Ensure the latest toolchain is installed (`pnpm >= 8.14`, `node >= 20`, Rust stable with `wasm-pack`, `wasm-bindgen`, `wasm-opt` on PATH).
- Install release dependencies once:
  ```bash
  pnpm install
  cargo install wasm-bindgen-cli wasm-opt --locked
  ```

## 2. Pre-flight Checklist
1. Pull the latest `001-phase-3.6-release` branch.
2. Run verification:
   ```bash
   pnpm release:verify
   ```
3. Update benchmarks if any performance-related change shipped; capture results in `tests/perf-snapshots/` (optional but recommended).
4. Review open changesets (`pnpm changeset status`) and confirm they reflect the release scope.

## 3. Versioning Strategy
| Change Type                              | Bump | Notes |
|------------------------------------------|------|-------|
| Bug fixes, dependency updates            | patch | No API change |
| Backwards-compatible feature additions   | minor | New options, faster defaults |
| Breaking API/CLI changes, config rename  | major | Coordinate with doc updates |

Guidelines:
- Group related packages in a single changeset when they ship together (e.g., `faster-md` + `mdx-compiler`).
- Prefer multiple small changesets over one large "catch-all".

## 4. Release Steps
1. Create or update changesets for any remaining work:
   ```bash
   pnpm changeset
   ```
2. Compute new versions and install lockfile updates:
   ```bash
   pnpm changeset version
   pnpm install --no-frozen-lockfile
   ```
3. Build release artifacts (WASM, bundles, type bundles):
   ```bash
   pnpm release:build
   ```
4. Verify once more after version bumps:
   ```bash
   pnpm release:verify
   ```
5. Publish packages and crates:
   ```bash
   pnpm release    # runs pnpm build && changeset publish
   cargo publish --package fmd-core
   cargo publish --package fmd-html
   cargo publish --package fmd-gfm
   cargo publish --package fmd-slug
   cargo publish --package fmd-wasm
   cargo publish --package fmd-cli
   cargo publish --package mdx-core
   ```
   Publish crates sequentially; wait for each to finish to avoid dependency resolution issues.
6. Tag and push:
   ```bash
   git push origin HEAD
   git push origin --tags
   ```

## 5. Post-release Actions
- Draft a GitHub release using the generated changelog entries (`.changeset` output) and attach the artifacts from `dist-release/` (bundles, WASM binaries).
- Verify that npm packages resolve correctly:
  ```bash
  npx npmview faster-md version
  ```
- Confirm crates are indexed:
  ```bash
  cargo search fmd-core --limit 1
  ```
- Update `STATUS.md` and `docs/compatibility.md` (when available) to reflect the new version.

## 6. Troubleshooting
- If `changeset publish` fails due to network issues, rerun with existing version bumps (no need to re-run `changeset version`).
- For npm two-factor accounts, use automation tokens with publish permission.
- If crates publish fails mid-way, bump patch versions to avoid "version already uploaded" conflicts.

Keep this guide in sync with script changes (`scripts/build-release.sh`, `scripts/verify-release.sh`) when the pipeline evolves.
