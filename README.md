# JetMD

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

- **50KB Document**: <3ms p50, <15ms p99
- **Memory**: ≤1.5× input size (typical)
- **Incremental**: ≥90% node reuse

## License

MIT