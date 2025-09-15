# Claude Code Context - faster-md & mdx

## Project Overview
High-performance Markdown/MDX processor. WASM-first architecture with Rust core. CommonMark compliant with opt-in GFM. Zero environment variables, all config via options.

## Current Focus
Building faster-md & mdx feature (branch: 001-faster-md-mdx)

## Tech Stack
- **Core**: Rust (latest stable)
- **Bindings**: wasm-bindgen (WASM), future Neon (native)
- **Packages**: TypeScript/JavaScript, Node ≥20, ESM
- **Build**: Cargo, wasm-pack, pnpm workspaces
- **Testing**: cargo test, vitest, CommonMark/GFM suites

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

tests/
├── conformance/ # Spec compliance
└── fixtures/    # Test documents
```

## Key Commands
```bash
# Build
pnpm build           # Build all packages
cargo build          # Build Rust crates
wasm-pack build      # Build WASM

# Test
pnpm test           # Run all tests
cargo test          # Rust tests
pnpm test:conformance # Spec compliance

# Dev
pnpm dev            # Watch mode
cargo watch         # Rust watch
```

## Performance Targets
- Native: ≥200 MB/s throughput
- WASM: ≥50 MB/s throughput  
- 50KB doc: <3ms p50, <15ms p99
- Memory: ≤1.5× input (typical)
- Incremental: ≥90% node reuse

## Recent Changes
- [2025-09-15] Created implementation plan and specs
- [2025-09-15] Defined data model and API contracts
- [2025-09-15] Set up monorepo structure

## Development Guidelines
1. TDD: Write tests first (RED-GREEN-Refactor)
2. No ENV vars - all config via options
3. Security by default (sanitization enabled)
4. Pure functions for plugins
5. Position tracking in AST nodes

## API Patterns
```javascript
// Basic usage
import { renderHtml } from 'faster-md'
const html = renderHtml('# Hello', { gfm: true })

// MDX compilation
import { compileMdx } from 'mdx-compiler'
const { code } = await compileMdx('<Button />')

// Processor with plugins
const processor = createProcessor()
  .use(gfm())
  .use(frontmatter())
```

## Testing Strategy
- CommonMark conformance ≥99.5%
- Property-based testing for edge cases
- Fuzzing for security validation
- Benchmark regression detection

---
*Last updated: 2025-09-15 | Feature: faster-md & mdx*