# JetMD

[![CI](https://github.com/jp-knj/jetmd/actions/workflows/ci.yml/badge.svg)](https://github.com/jp-knj/jetmd/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](#license)
[![Node >=20](https://img.shields.io/badge/node-%3E%3D20-43853d.svg)](package.json)
[![Rust 1.75+](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](rust-toolchain.toml)

High-performance Markdown/MDX processor with WASM-first architecture.

## Features

- ⚡ **Fast**: ≥200 MB/s native, ≥50 MB/s WASM throughput
- 📝 **CommonMark Compliant**: Full CommonMark 0.30 specification support
- 🔧 **GFM Extensions**: Tables, strikethrough, autolinks (opt-in)
- 🎯 **MDX Support**: JSX components in Markdown
- 🔒 **Secure**: Sanitization enabled by default
- 🌐 **WASM-First**: Runs in browser and Node.js
- 📦 **Zero Config**: No environment variables needed

## Quick Start

```bash
# Install
npm install faster-md

# Parse Markdown
import { renderHtml } from 'faster-md'
const html = renderHtml('# Hello World')

# MDX Support
npm install mdx-compiler
import { compileMdx } from 'mdx-compiler'
const { code } = await compileMdx('<Button />')
```

### Incremental Sessions

```ts
import { createParseSession, parseIncremental } from 'faster-md'

const sessionId = await createParseSession()
await parseIncremental(sessionId, '# Title', [])
await parseIncremental(sessionId, '# Title\nMore', [
  { start: { line: 1, column: 1, offset: 8 }, end: { line: 1, column: 1, offset: 8 }, text: '\nMore' },
])
```

## Project Structure

```
crates/          # Rust packages
├── fmd-core/    # Parser, lexer, AST
├── fmd-gfm/     # GFM extensions
├── fmd-html/    # HTML renderer
├── fmd-wasm/    # WASM bindings
└── fmd-cli/     # Rust CLI

packages/        # Node packages  
├── faster-md/   # Node wrapper
├── mdx-compiler/# MDX → ESM compiler
└── mdx-runtime/ # Runtime components
```

## Development

```bash
# Setup
pnpm install

# Build
pnpm build        # Build all packages
cargo build       # Build Rust crates

# Test
pnpm test         # Run all tests
cargo test        # Rust tests

# Lint/Format
pnpm lint         # Run Biome checks
pnpm format       # Format code
```

## Performance

- **WASM Throughput (50KB doc)**: 37.9 MB/s (target ≥50 MB/s) → needs optimisation. See `node tests/benchmarks/wasm_perf.js`.
- **Latency (50KB doc)**: 0.62 ms p50 (target <3 ms) → `cargo bench --bench latency`.
- **Memory Usage (500KB doc)**: 86× input (target ≤1.5×) → failing in `cargo bench --bench memory`.
- **Incremental Editing**: 1.30 ms p50 for append/edit operations; optimisations pending.

## Examples

- `demo/README.md`: end-to-end walkthrough comparing JetMD with legacy pipelines.
- `demo/performance-comparison.js`: script that renders real-world docs and prints throughput deltas.
- `examples/astro-jetmd`: Astro integration that wires `faster-md` into MDX routes.
- `tests/benchmarks/*`: Criterion and Node benchmarks that back the performance targets in `specs/001-faster-md-mdx/tasks.md`.

Additional API details live in [`docs/api.md`](docs/api.md) and migration notes in [`docs/migration.md`](docs/migration.md).

## License

MIT
