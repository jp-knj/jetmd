# Project Overview: JetMD (faster-md & mdx)

## Purpose
High-performance Markdown/MDX processor with WASM-first architecture. The project aims to provide:
- Ultra-fast Markdown parsing and rendering (≥200 MB/s native, ≥50 MB/s WASM)
- CommonMark 0.30 compliant with opt-in GitHub Flavored Markdown (GFM) extensions
- MDX support for JSX components in Markdown
- Zero environment variables - all configuration via options
- Security by default with sanitization enabled

## Current Status
- **Active Branch**: 001-faster-md-mdx
- **Phase**: Planning and specification complete, ready for implementation
- **Last Updated**: 2025-09-15

## Architecture
- **Core**: Rust-based parser and lexer for maximum performance
- **Bindings**: WASM via wasm-bindgen (primary), future native via Neon
- **Distribution**: Node.js packages with ESM support
- **Parsing**: Dual-pass architecture with SIMD acceleration
- **Storage**: Rope-based for incremental parsing with ≥90% node reuse

## Key Features
1. CommonMark and GFM compliance
2. MDX compilation to ESM modules
3. Incremental parsing support
4. Streaming output capability
5. Plugin system for extensibility
6. Position tracking in AST nodes
7. Frontmatter support (YAML/TOML)
8. Security-first approach