# PR: Phase 3.1 - Monorepo Infrastructure Setup

## 🎯 Overview
This PR establishes the foundational infrastructure for the JetMD project - a high-performance Markdown/MDX processor with WASM-first architecture.

## ✅ What's Included

### 📦 Monorepo Structure
- **Rust Workspace** (7 crates):
  - `fmd-core`: Core parser, lexer, AST operations
  - `fmd-gfm`: GitHub Flavored Markdown extensions
  - `fmd-html`: HTML renderer with sanitization
  - `fmd-slug`: Stable slug generation
  - `fmd-wasm`: WASM bindings via wasm-bindgen
  - `fmd-cli`: Rust CLI tool
  - `mdx-core`: MDX token detection

- **Node.js Packages** (6 packages):
  - `faster-md`: Node.js wrapper for WASM
  - `mdx-compiler`: MDX to ESM transformation
  - `mdx-runtime`: Runtime components for MDX
  - `faster-md-cli`: Node CLI wrapper
  - `faster-md-vite`: Vite plugin for Markdown
  - `mdx-vite`: Vite plugin for MDX

### 🛠️ Build Tools & Configuration
- **Package Management**: pnpm@8.14.0 workspaces
- **Version Management**: Changesets for coordinated releases
- **Code Quality**: Biome (replaces Prettier/ESLint) for formatting and linting
- **Git Hooks**: Husky + lint-staged for pre-commit checks
- **Rust Toolchain**: Stable with rustfmt and clippy
- **WASM**: Documentation for wasm-pack setup

### 🚀 CI/CD Pipeline
- GitHub Actions workflow for:
  - Rust tests and builds
  - WASM compilation
  - Node.js tests (Node 20.x, 21.x)
  - Biome linting
  - Conformance tests (CommonMark/GFM)

### 📝 Documentation
- Project README with quick start guide
- WASM setup guide
- Monorepo structure documentation

## 📋 Tasks Completed
- [x] T001: Created branch 001-phase-3.1-setup
- [x] T002: Set up changesets (@changesets/cli@2.27.1)
- [x] T003: Created monorepo structure
- [x] T004: Initialized Cargo workspace
- [x] T005: Initialized pnpm workspace with exact versions
- [x] T006: Set up Rust toolchain
- [x] T007: Configured CI/CD
- [x] T008: Installed husky (8.0.0)
- [x] T009: Set up pre-commit with lint-staged (15.2.0)
- [x] T010: Skipped (not using conventional commits)
- [x] T011: Created .lintstagedrc.json with Biome
- [x] T012: Created this PR

## 🔍 Key Decisions
1. **Biome over Prettier/ESLint**: Single tool for both formatting and linting
2. **Exact versions**: All dependencies use exact versions for reproducibility
3. **No conventional commits**: Simplified commit process without strict formatting
4. **WASM-first**: Primary distribution via WebAssembly

## 🧪 Testing
To verify the setup:
```bash
# Install dependencies
pnpm install

# Run linting
pnpm lint

# Build Rust crates
cargo build

# Run tests
cargo test
pnpm test
```

## 📊 Performance Targets
- Native: ≥200 MB/s throughput
- WASM: ≥50 MB/s throughput
- 50KB docs: <3ms p50, <15ms p99
- Memory: ≤1.5× input size

## 🔜 Next Steps
- Phase 3.2: Write failing tests (TDD approach)
- Phase 3.3: Core implementation
- Phase 3.4: Integration (CLI, Vite plugins)
- Phase 3.5: Polish (docs, performance)
- Phase 3.6: Release preparation

## 📁 Files Changed
- 45 new files for initial setup
- Monorepo configuration files
- CI/CD workflows
- Package manifests
- Documentation

## ⚠️ Notes
- All tests will fail until implementation (TDD approach)
- WASM build requires wasm-pack installation
- Node.js ≥20.0.0 required

---

Ready for review and merge into `001-faster-md-mdx` branch.