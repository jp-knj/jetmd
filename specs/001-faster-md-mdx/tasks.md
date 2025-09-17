# Tasks: faster-md & mdx

**Input**: Design documents from `/specs/001-faster-md-mdx/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/

## Branch Strategy

### Main Feature Branch
- **001-faster-md-mdx** - Main feature branch (current)

### Phase Branches
Each phase will have its own branch for parallel development:
```
001-faster-md-mdx (main feature branch)
├── 001-phase-3.1-setup
├── 001-phase-3.2-tests  
├── 001-phase-3.3-core
├── 001-phase-3.4-integration
├── 001-phase-3.5-polish
└── 001-phase-3.6-release
```

### Branch Workflow
1. Create phase branch from 001-faster-md-mdx
2. Complete all tasks in that phase
3. Create PR back to 001-faster-md-mdx
4. Review and merge
5. Next phase branches from updated 001-faster-md-mdx

### Parallel Development
- Multiple phases can be developed simultaneously if no dependencies
- Tests (3.2) and Release prep (3.6) can start early
- Core (3.3) must wait for tests to be written

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → ✓ Loaded: Rust/WASM architecture, monorepo structure
   → Extract: fmd-core, fmd-wasm, faster-md, mdx-compiler libraries
2. Load optional design documents:
   → data-model.md: Document, ProcessorOptions, FMDAST entities
   → contracts/: rust-api.md, wasm-api.md, node-api.md
   → research.md: Parser architecture, WASM strategy decisions
3. Generate tasks by category:
   → Setup: monorepo init, Rust/Node dependencies
   → Tests: conformance, contract tests, benchmarks
   → Core: parser, AST, WASM bindings, Node wrapper
   → Integration: MDX compiler, Vite plugins
   → Polish: docs, performance validation
4. Apply task rules:
   → Different crates/packages = mark [P] for parallel
   → Same file = sequential (no [P])
   → Tests before implementation (TDD)
5. Number tasks sequentially (T001-T050)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   → All APIs have contract tests ✓
   → All entities have implementations ✓
   → All user stories covered ✓
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Rust crates**: `crates/[crate-name]/src/`
- **Node packages**: `packages/[package-name]/src/`
- **Tests**: `tests/` at repository root, `crates/*/tests/`, `packages/*/test/`
- **Docs**: `docs/` at repository root

## Phase 3.1: Setup
**Branch**: `001-phase-3.1-setup`

- [ ] T001 Create and checkout branch 001-phase-3.1-setup from 001-faster-md-mdx
- [ ] T002 [P] Set up changesets for coordinated releases and version management (@changesets/cli@2.27.1)
- [ ] T003 Create monorepo structure with crates/ and packages/ directories
- [ ] T004 Initialize Cargo workspace in crates/ with workspace members
- [ ] T005 Initialize pnpm workspace with pnpm-workspace.yaml and shared dependencies (pnpm@8.14.0, exact versions: @biomejs/biome@1.5.0, @types/node@20.10.0, typescript@5.3.0, vitest@1.0.0, acorn@8.11.0)
- [ ] T006 [P] Set up Rust toolchain (stable) and wasm-pack
- [ ] T007 [P] Configure CI/CD with GitHub Actions for Rust and Node

### Git Hooks & Code Quality
- [ ] T008 Install and configure husky for git hooks management (husky@8.0.0)
- [ ] T009 Set up pre-commit hook with lint-staged for formatting and linting (lint-staged@15.2.0)
- [ ] T010 Configure commit-msg hook for conventional commits validation (SKIP - not using conventional commits)
- [ ] T011 [P] Create .lintstagedrc.json for Rust (cargo fmt) and JS (Biome instead of prettier/eslint)
- [ ] T012 Create PR: Merge 001-phase-3.1-setup → 001-faster-md-mdx

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**Branch**: `001-phase-3.2-tests`
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

- [ ] T013 Create and checkout branch 001-phase-3.2-tests from 001-faster-md-mdx

### Conformance Tests
- [ ] T014 [P] Set up CommonMark test suite in tests/conformance/commonmark.rs
- [ ] T015 [P] Set up GFM test suite in tests/conformance/gfm.rs
- [ ] T016 [P] Create MDX test fixtures in tests/fixtures/mdx/

### Rust API Contract Tests
- [ ] T017 [P] Contract test parse() in crates/fmd-core/tests/parse_contract.rs
- [ ] T018 [P] Contract test to_html() in crates/fmd-html/tests/render_contract.rs
- [ ] T019 [P] Contract test incremental parsing in crates/fmd-core/tests/incremental_contract.rs

### WASM API Contract Tests
- [ ] T020 [P] Contract test parseToAst() in crates/fmd-wasm/tests/parse_wasm.rs
- [ ] T021 [P] Contract test renderHtml() in crates/fmd-wasm/tests/render_wasm.rs
- [ ] T022 [P] Contract test session management in crates/fmd-wasm/tests/session_wasm.rs

### Node API Contract Tests
- [ ] T023 [P] Contract test parse() in packages/faster-md/test/parse.test.js
- [ ] T024 [P] Contract test renderHtml() in packages/faster-md/test/render.test.js
- [ ] T025 [P] Contract test processor with plugins in packages/faster-md/test/processor.test.js
- [ ] T026 [P] Contract test stream processing in packages/faster-md/test/stream.test.js

### MDX Contract Tests
- [ ] T027 [P] Contract test compileMdx() in packages/mdx-compiler/test/compile.test.js
- [ ] T028 [P] Contract test JSX parsing in packages/mdx-compiler/test/jsx.test.js
- [ ] T029 [P] Contract test ESM generation in packages/mdx-compiler/test/esm.test.js

### Integration Tests
- [ ] T030 [P] Integration test: Parse quickstart examples in tests/integration/quickstart.rs
- [ ] T031 [P] Integration test: Security/sanitization in tests/integration/security.rs
- [ ] T032 [P] Integration test: Performance benchmarks in tests/benchmarks/throughput.rs
- [ ] T033 Create PR: Merge 001-phase-3.2-tests → 001-faster-md-mdx

## Phase 3.3: Core Implementation (ONLY after tests are failing)
**Branch**: `001-phase-3.3-core`

- [x] T034 Create and checkout branch 001-phase-3.3-core from 001-faster-md-mdx

### Rust Core Parser (fmd-core)
- [x] T035 Create AST node types in crates/fmd-core/src/ast.rs
- [x] T036 Implement block scanner with SIMD in crates/fmd-core/src/scanner.rs
- [x] T037 Implement inline parser in crates/fmd-core/src/inline.rs
- [x] T038 Implement parse() function in crates/fmd-core/src/lib.rs
- [x] T039 Add position tracking in crates/fmd-core/src/position.rs
- [x] T040 [P] Create rope structure for incremental in crates/fmd-core/src/rope.rs

### HTML Renderer (fmd-html)
- [x] T041 [P] Implement HTML visitor in crates/fmd-html/src/visitor.rs
- [x] T042 [P] Add sanitization with ammonia in crates/fmd-html/src/sanitize.rs
- [x] T043 [P] Implement to_html() in crates/fmd-html/src/lib.rs

### GFM Extensions (fmd-gfm)
- [x] T044 [P] Add table parser in crates/fmd-gfm/src/table.rs
- [x] T045 [P] Add strikethrough parser in crates/fmd-gfm/src/strikethrough.rs
- [x] T046 [P] Add autolink parser in crates/fmd-gfm/src/autolink.rs

### WASM Bindings (fmd-wasm)
- [x] T047 Set up wasm-bindgen exports in crates/fmd-wasm/src/lib.rs
- [x] T048 Implement parseToAst() binding in crates/fmd-wasm/src/parse.rs
- [x] T049 Implement renderHtml() binding in crates/fmd-wasm/src/render.rs
- [x] T050 Add session management in crates/fmd-wasm/src/session.rs

### Node Wrapper (faster-md)
- [x] T051 Create WASM loader in packages/faster-md/src/loader.js
- [x] T052 Implement parse() wrapper in packages/faster-md/src/parse.js
- [x] T053 Implement renderHtml() wrapper in packages/faster-md/src/render.js
- [x] T054 Create processor class in packages/faster-md/src/processor.js
- [x] T055 Add stream support in packages/faster-md/src/stream.js

### MDX Compiler (mdx-compiler)
- [x] T056 Implement MDX tokenizer in packages/mdx-compiler/src/tokenizer.js
- [x] T057 Add JSX parser integration in packages/mdx-compiler/src/jsx.js
- [x] T058 Implement ESM code generator in packages/mdx-compiler/src/codegen.js
- [x] T059 Create compileMdx() in packages/mdx-compiler/src/index.js
- [x] T060 Create PR: Merge 001-phase-3.3-core → 001-faster-md-mdx

## Phase 3.4: Integration
**Branch**: `001-phase-3.4-integration`

- [x] T061 Create and checkout branch 001-phase-3.4-integration from 001-faster-md-mdx

### CLI Implementation
- [x] T062 [P] Implement Rust CLI in crates/fmd-cli/src/main.rs
- [x] T063 [P] Create Node CLI wrapper in packages/faster-md-cli/src/cli.js

### Vite Plugins
- [x] T064 [P] Create Markdown Vite plugin in packages/faster-md-vite/src/index.js
- [x] T065 [P] Create MDX Vite plugin in packages/mdx-vite/src/index.js

### Plugin System
- [x] T066 Implement plugin registry in packages/faster-md/src/plugins/registry.js
- [x] T067 [P] Create GFM plugin in packages/faster-md/src/plugins/gfm.js
- [x] T068 [P] Create frontmatter plugin in packages/faster-md/src/plugins/frontmatter.js
- [x] T069 [P] Create highlight plugin in packages/faster-md/src/plugins/highlight.js
- [x] T070 Create PR: Merge 001-phase-3.4-integration → 001-faster-md-mdx

## Phase 3.5: Polish
**Branch**: `001-phase-3.5-polish`

- [x] T071 Create and checkout branch 001-phase-3.5-polish from 001-faster-md-mdx

### Performance Validation
- [ ] T072 Benchmark: Verify ≥50 MB/s WASM throughput in tests/benchmarks/wasm_perf.js
  - [ ] 2025-09-17: 37.9 MB/s (fail) via direct WASM render call. Investigate cache layer and wasm SIMD.
- [ ] T073 Benchmark: Verify <3ms p50 for 50KB docs in tests/benchmarks/latency.rs
  - [x] 2025-09-17: 0.62 ms p50 (pass) across parse/render/incremental benches.
- [ ] T074 Memory profiling: Verify ≤1.5× input usage in tests/benchmarks/memory.rs
  - [ ] 2025-09-17: 86× input (fail) – current arena allocator retains full AST; needs compact mode.

### Documentation
- [ ] T075 [P] Write API documentation in docs/api.md
- [ ] T076 [P] Create migration guide in docs/migration.md
- [ ] T077 [P] Update README.md with badges and examples
  - [x] 2025-09-17 Author contributor guide in `AGENTS.md`

### Final Integration
- [ ] T078 Run full conformance suite and fix failures
- [ ] T079 Security audit with cargo-audit and npm audit
  - [x] 2025-09-17: `cargo audit` (no advisories reported).
  - [ ] 2025-09-17: `pnpm audit` blocked (ENOTFOUND registry.npmjs.org).
- [ ] T080 Create example projects in examples/
- [ ] T081 Manual testing with quickstart.md scenarios
- [ ] T082 Create PR: Merge 001-phase-3.5-polish → 001-faster-md-mdx

## Phase 3.6: Release Preparation
**Branch**: `001-phase-3.6-release`

- [ ] T083 Create and checkout branch 001-phase-3.6-release from 001-faster-md-mdx

### Package Configuration
- [x] T084 [P] Configure package.json files with name, version, description, keywords, repository
  - [x] 2025-09-17: Added authors, homepage, bugs, repo metadata across packages/* and root package.json.
- [x] T085 [P] Set up Cargo.toml files with [package] metadata for crates.io publishing
  - [x] 2025-09-17: Updated workspace metadata and propagated documentation/keywords/categories to crates.
- [x] T086 [P] Create .npmignore files for packages/ to exclude test and source files
  - [x] 2025-09-17: Added standard ignore template to packages/* (dist-only publish).
- [x] T087 [P] Configure Cargo.toml exclude patterns for smaller crate sizes
  - [x] 2025-09-17: Added shared exclude list to crates to strip tests/fixtures/docs from crates.io tarballs.

### pnpm-Specific Configuration
- [ ] T088 [P] Configure pnpm catalogs for shared dependencies across packages
- [ ] T089 [P] Set up pnpm overrides for dependency resolution in root package.json
- [ ] T090 [P] Configure pnpm's side-effects-cache for faster installs
- [ ] T091 [P] Create pnpm scripts for monorepo commands (build:all, test:all, etc.)

### Build & Distribution
- [ ] T092 Create release build script with wasm-opt optimization in scripts/build-release.sh
- [ ] T093 Generate TypeScript definitions from WASM bindings in packages/faster-md/types/
- [ ] T094 Create UMD and ESM bundles for browser usage in packages/faster-md/dist/
- [ ] T095 Set up pnpm publish hooks with prepublishOnly scripts

### Publishing Configuration
- [ ] T096 Configure npm registry authentication in .npmrc (with placeholder tokens)
- [ ] T097 Set up crates.io API token configuration (documented in RELEASE.md)
- [ ] T098 Create changeset configuration in .changeset/config.json
- [ ] T099 Set up version bump strategy (patch/minor/major) guidelines

### Release Automation
- [ ] T100 Create GitHub Actions workflow for release in .github/workflows/release.yml
- [ ] T101 Set up automatic changelog generation from changesets
- [ ] T102 Configure GitHub release creation with artifacts (WASM files, bundles)
- [ ] T103 Add release verification tests in scripts/verify-release.sh

### Documentation & Process
- [ ] T104 [P] Create RELEASE.md with step-by-step release process
- [ ] T105 [P] Document version compatibility matrix in docs/compatibility.md
- [ ] T106 [P] Set up release notes template in .github/release-template.md
- [ ] T107 Create post-release checklist (npm verify, crates.io verify, CDN update)
- [ ] T108 Create PR: Merge 001-phase-3.6-release → 001-faster-md-mdx

## Dependencies

### Branch Dependencies
- Phase 3.1 (Setup) must complete and merge first
- Phase 3.2 (Tests) can start after 3.1 merges
- Phase 3.3 (Core) requires 3.2 tests to be written (but not merged)
- Phase 3.4 (Integration) requires 3.3 to merge
- Phase 3.5 (Polish) requires 3.4 to merge
- Phase 3.6 (Release) can start after 3.3, parallel with 3.4/3.5

### Task Dependencies Within Phases
- T001 (branch creation) is always first in each phase
- Setup (T002-T011) must complete before phase merge
- Tests (T014-T032) must all fail before moving to implementation
- Core parser (T035-T040) before renderer (T041-T043)
- WASM bindings (T047-T050) require core and renderer
- Node wrapper (T051-T055) requires WASM bindings
- MDX compiler (T056-T059) requires Node wrapper
- Final PR task is always last in each phase

## Parallel Execution Examples

### Initial Setup (Phase 3.1, after T003-T005)
```bash
# Launch T001, T005-T006 together:
Task: "Set up changesets for coordinated releases"
Task: "Set up Rust toolchain and wasm-pack"
Task: "Configure CI/CD with GitHub Actions"
```

### Git Hooks Setup (Phase 3.1, after T006)
```bash
# Run T007-T009 sequentially (husky setup first):
Task: "Install and configure husky"
Task: "Set up pre-commit hook"
Task: "Configure commit-msg hook"
```

### Contract Tests (Phase 3.2, after T013)
```bash
# Launch T011-T029 together (all test files):
Task: "Set up CommonMark test suite"
Task: "Set up GFM test suite"
Task: "Contract test parse() in Rust"
Task: "Contract test parseToAst() in WASM"
Task: "Contract test parse() in Node"
Task: "Contract test compileMdx()"
# ... (all test tasks can run in parallel)
```

### Independent Crates (Phase 3.3, after tests written)
```bash
# Launch T039-T041 together (GFM extensions):
Task: "Add table parser"
Task: "Add strikethrough parser"
Task: "Add autolink parser"
```

### Documentation (Phase 3.5, parallel tasks)
```bash
# Launch T066-T068 together:
Task: "Write API documentation"
Task: "Create migration guide"
Task: "Update README.md"
```

### Release Preparation (Phase 3.6, parallel tasks)
```bash
# Launch T073-T076 together (package configuration):
Task: "Configure package.json files with metadata"
Task: "Set up Cargo.toml files for crates.io"
Task: "Create .npmignore files"
Task: "Configure Cargo.toml exclude patterns"

# Launch T093-T095 together (documentation):
Task: "Create RELEASE.md with release process"
Task: "Document version compatibility matrix"
Task: "Set up release notes template"
```

## Notes
- [P] tasks = different files, no shared state
- Monorepo allows parallel crate/package development
- Tests MUST fail before implementation (TDD)
- Commit after each task with descriptive message (no AI tool references)
- Run conformance tests frequently during development
- Use `cargo watch` and `pnpm dev` for rapid iteration
- **IMPORTANT**: Use exact versions specified in tasks for reproducible builds
- **Tooling**: Biome is used for both linting and formatting (replaces Prettier/ESLint)

## PR Descriptions for Each Phase

### Phase 3.1 Setup PR
```markdown
## Setup: Monorepo Infrastructure

### What
- Initialized monorepo with pnpm workspaces and Cargo workspace
- Configured changesets for version management
- Set up git hooks with husky and lint-staged
- Added CI/CD pipelines

### Files Changed
- package.json, pnpm-workspace.yaml
- Cargo.toml (workspace)
- .husky/, .lintstagedrc.json
- .github/workflows/

### Testing
- [ ] pnpm install works
- [ ] cargo build works
- [ ] Git hooks trigger on commit
- [ ] CI runs on push
```

### Phase 3.2 Tests PR
```markdown
## Tests: Contract and Conformance Tests

### What
- Added CommonMark/GFM conformance test suites
- Created contract tests for all APIs (Rust, WASM, Node)
- Set up integration test framework
- All tests currently FAILING (TDD approach)

### Files Changed
- tests/conformance/
- crates/*/tests/
- packages/*/test/

### Testing
- [ ] All tests run and fail as expected
- [ ] No implementation code added
- [ ] Test coverage setup complete
```

### Phase 3.3 Core PR
```markdown
## Core: Parser and Compiler Implementation

### What
- Implemented Rust parser with SIMD optimizations
- Added WASM bindings via wasm-bindgen
- Created Node.js wrapper packages
- Implemented MDX compiler

### Files Changed
- crates/fmd-core/src/
- crates/fmd-wasm/src/
- packages/faster-md/src/
- packages/mdx-compiler/src/

### Testing
- [ ] All contract tests now pass
- [ ] CommonMark conformance ≥99.5%
- [ ] Performance benchmarks meet targets
```

### Phase 3.4 Integration PR
```markdown
## Integration: CLI and Plugins

### What
- Implemented Rust and Node CLIs
- Created Vite plugins for .md and .mdx
- Built plugin system with GFM, frontmatter, highlight

### Files Changed
- crates/fmd-cli/src/
- packages/faster-md-cli/src/
- packages/*-vite/src/
- packages/faster-md/src/plugins/

### Testing
- [ ] CLI commands work as expected
- [ ] Vite plugins compile correctly
- [ ] Plugins integrate properly
```

### Phase 3.5 Polish PR
```markdown
## Polish: Performance and Documentation

### What
- Verified performance targets met
- Completed API documentation
- Security audit passed
- Created example projects

### Files Changed
- docs/
- examples/
- README.md
- Benchmark results

### Testing
- [ ] WASM throughput ≥50 MB/s
- [ ] 50KB docs <3ms p50
- [ ] All examples work
- [ ] No security vulnerabilities
```

### Phase 3.6 Release PR
```markdown
## Release: Publishing Configuration

### What
- Configured package.json and Cargo.toml for publishing
- Set up release automation with changesets
- Created release documentation
- Prepared for npm and crates.io publishing

### Files Changed
- All package.json files
- All Cargo.toml files
- .changeset/config.json
- .github/workflows/release.yml
- RELEASE.md

### Testing
- [ ] Dry-run publish successful
- [ ] Changesets work correctly
- [ ] Release workflow triggers properly
- [ ] All packages ready to publish
```

## Success Criteria
- [ ] CommonMark conformance ≥99.5%
- [ ] WASM throughput ≥50 MB/s
- [ ] 50KB document <3ms p50 latency
- [ ] All quickstart examples working
- [ ] Zero security vulnerabilities
- [ ] Full API documentation
- [ ] CI/CD pipeline green
- [ ] Packages ready for npm publish
- [ ] Crates ready for crates.io publish
- [ ] Release automation configured
