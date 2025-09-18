# Repository Guidelines

## Project Layout & Ownership
Rust crates live in `crates/` (`fmd-core`, `fmd-gfm`, `fmd-html`, `fmd-slug`, `fmd-wasm`, `fmd-cli`, `mdx-core`) and share workspace settings via `Cargo.toml`. JavaScript and TypeScript packages sit in `packages/` (wrappers, CLIs, Vite plugins, wasm bundle). Long-form docs and demos are in `docs/`, `demo/`, and `examples/`. Cross-cutting tests stay in `tests/` (`benchmarks/`, `conformance/`, `fixtures/`). Consult `specs/001-faster-md-mdx/` for phase plans and `STATUS.md` for current priorities.

## Branch Flow & Phase Cadence
All feature work fans out from `001-faster-md-mdx`. Create phase branches `001-phase-3.x-*`, complete the numbered checklist, then PR back to the feature branch. Phase 3.2 tests must merge before Phase 3.3 implementation; Phase 3.6 release prep can run alongside 3.5 once 3.3 is green. Track success gates: CommonMark ≥99.5%, WASM ≥50 MB/s, 50 KB doc <3 ms, zero known vulnerabilities, API docs shipped.

## Build & Development Commands
`pnpm install` bootstraps the workspace. Use `pnpm build` for JS/TS artefacts and `cargo build --workspace --release` for Rust (add the WASM target when needed). Run `pnpm test` for Vitest suites, `cargo test --workspace` for Rust checks, and `cargo bench --bench latency` or `node demo/performance-comparison.js` when validating performance tasks (T072–T074). Quality gates: `pnpm lint`, `pnpm lint:fix`, `pnpm format`, `cargo fmt --all`, and `cargo clippy --workspace --all-targets`.

## Coding Style & Tooling
Biome enforces 2-space indents, single quotes, trailing commas, 100-character lines, and LF endings; colocate tests as `*.test.ts`. Prefer named exports, reserve defaults for singular entry points, and align file naming with existing patterns. Rust code must run through `rustfmt` and resolve Clippy warnings. Keep wasm-bindgen surfaces in `crates/fmd-wasm` stable, and document new exports in the matching `packages/wasm` wrapper.

## Testing Protocol
Phase 3.2 mandates writing failing conformance and contract suites (`tests/conformance/*.rs`, `packages/*/test/*.ts`) before implementation. For regressions, run `cargo test -- --ignored test_commonmark_conformance`, `node test-gfm.js`, and benchmark checks under `tests/benchmarks/`. Update fixtures in `tests/fixtures` when the upstream CommonMark or GFM specs change and record benchmark deltas in PR summaries.

## Commits, PRs & Release Prep
Use short imperative commit subjects prefixed with `fix:`, `chore:`, etc., referencing task IDs where helpful. Create a Changeset (`pnpm changeset`) whenever shipping packages change. PR templates expect a “What/Testing” summary plus evidence (logs, screenshots, benchmarks) for risky changes. Before closing a phase, confirm the success gates, run `cargo-audit` and `pnpm audit`, refresh `docs/api.md` and `docs/migration.md`, and document release steps in `RELEASE.md`. Dry-run publish flows with `pnpm build && changeset publish --dry-run` and sync outcomes to `STATUS.md`.
