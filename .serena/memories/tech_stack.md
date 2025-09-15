# Technology Stack

## Core Technologies
- **Language**: Rust (latest stable) for core implementation
- **WASM**: wasm-bindgen for WebAssembly bindings
- **JavaScript/TypeScript**: For Node.js packages and API wrappers
- **Runtime**: Node.js ≥20 with ESM support

## Planned Dependencies
### Rust Crates
- `wasm-bindgen`: WASM bindings
- `serde`: Serialization/deserialization
- `nom` or `winnow`: Parser combinators
- `xi-rope`: Rope data structure for incremental parsing
- Future: `neon` for native Node.js bindings

### JavaScript/Node
- `acorn`: JavaScript parser for MDX expressions
- `vitest`: Testing framework
- `pnpm`: Package manager with workspace support
- `wasm-pack`: Build tool for WASM modules

## Build Tools
- `cargo`: Rust build system
- `wasm-pack`: WASM packaging
- `wasm-opt`: WASM optimization (20-30% size reduction)
- `pnpm workspaces`: Monorepo management

## Testing Infrastructure
- `cargo test`: Rust unit tests
- `vitest`: JavaScript/TypeScript tests
- CommonMark conformance suite (target ≥99.5%)
- GFM specification tests
- Property-based testing for edge cases
- Fuzzing for security validation