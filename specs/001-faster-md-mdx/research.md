# Research: faster-md & mdx

**Date**: 2025-09-15  
**Feature**: High-performance Markdown/MDX processor  
**Branch**: 001-faster-md-mdx

## Executive Summary
Research findings for WASM-first Markdown processor with MDX support. All technical decisions validated against performance requirements (≥200 MB/s native, ≥50 MB/s WASM) and architectural constraints (no ENV vars, options-based config).

## Parser Architecture

### Decision: Dual-pass architecture with SIMD acceleration
**Rationale**: 
- Block scanning in first pass allows SIMD optimization for line boundaries
- Inline parsing per block minimizes memory allocation
- Proven approach in pulldown-cmark and comrak

**Alternatives considered**:
- Single-pass recursive descent: Rejected - poor incremental parsing support
- PEG parser: Rejected - insufficient performance for target throughput
- Tree-sitter: Rejected - overhead too high for simple Markdown

## WASM Strategy

### Decision: wasm-bindgen with wasm-opt post-processing
**Rationale**:
- Most mature Rust→WASM toolchain
- Supports both sync and async initialization
- Good JavaScript interop for options objects
- wasm-opt can reduce size by 20-30%

**Alternatives considered**:
- AssemblyScript: Rejected - would require full rewrite, less performance
- Emscripten: Rejected - C++ toolchain, larger runtime overhead
- Direct WAT: Rejected - unmaintainable for complex project

## Incremental Parsing

### Decision: Rope-based storage with range invalidation
**Rationale**:
- Ropes provide O(log n) edits and lookups
- Range invalidation allows precise re-parsing
- Xi-editor's rope library (xi-rope) is production-tested

**Alternatives considered**:
- Gap buffer: Rejected - poor for multi-cursor/concurrent edits
- Piece table: Rejected - complex for marginal benefit
- Full reparse: Rejected - fails incremental performance requirement

## MDX Integration

### Decision: Lightweight lookahead with delegated JS parsing
**Rationale**:
- Avoids duplicating JavaScript parser in Rust
- Acorn is battle-tested for ESTree generation
- Clean separation of concerns

**Alternatives considered**:
- Full JS parser in Rust: Rejected - maintenance burden, larger WASM size
- Regex-based extraction: Rejected - cannot handle nested JSX correctly
- MDX.js integration: Rejected - different architecture, performance constraints

## Security Model

### Decision: Default-deny with opt-in dangerous features
**Rationale**:
- Matches user expectation for Markdown processors
- Sanitization via allowlist safer than denylist
- Explicit opt-in for raw HTML prevents accidents

**Alternatives considered**:
- Trust all input: Rejected - security vulnerability
- CSP-only protection: Rejected - insufficient for stored XSS
- External sanitizer: Rejected - performance overhead, integration complexity

## Plugin Architecture

### Decision: Registration-based with priority ordering
**Rationale**:
- Simple mental model for users
- Predictable execution order
- Pure functions enable parallelization

**Alternatives considered**:
- AST visitor pattern: Rejected - harder to optimize
- Event emitter: Rejected - implicit dependencies
- Middleware chain: Rejected - complex for syntax extensions

## Testing Strategy

### Decision: CommonMark/GFM suites + property-based testing
**Rationale**:
- Official test suites ensure spec compliance
- Property testing catches edge cases
- Fuzzing finds security issues

**Alternatives considered**:
- Manual test cases only: Rejected - insufficient coverage
- Snapshot testing: Rejected - brittle, doesn't validate correctness
- E2E only: Rejected - slow feedback loop

## Monorepo Tooling

### Decision: pnpm workspaces + changesets
**Rationale**:
- pnpm: Fast, disk-efficient with proper hoisting control
- Changesets: Proven for coordinated releases
- Both have excellent monorepo support

**Alternatives considered**:
- Lerna: Rejected - maintenance concerns
- Rush: Rejected - overcomplicated for this project
- Yarn workspaces: Rejected - pnpm more performant

## Build Pipeline

### Decision: Cargo for Rust, wasm-pack for WASM, Vite for packages
**Rationale**:
- Standard tools reduce learning curve
- wasm-pack handles bindings generation
- Vite provides fast dev experience

**Alternatives considered**:
- Custom build scripts: Rejected - maintenance burden
- Webpack: Rejected - slower than Vite
- Turbo: Rejected - overkill for current scope

## Performance Validation

### Decision: Criterion for Rust, Vitest bench for JS
**Rationale**:
- Criterion: Statistical rigor, regression detection
- Vitest bench: Integrated with test suite
- Both output machine-readable results for CI

**Alternatives considered**:
- Manual timing: Rejected - not reproducible
- Hyperfine: Rejected - CLI only, need library benchmarks
- Custom harness: Rejected - reinventing the wheel

## Memory Management

### Decision: Arena allocation for AST nodes
**Rationale**:
- Batch deallocation improves performance
- Predictable memory patterns
- Reduces fragmentation

**Alternatives considered**:
- Reference counting: Rejected - overhead for temporary nodes
- Garbage collection: Rejected - not available in Rust/WASM
- Per-node allocation: Rejected - fragmentation, slower

## Error Handling

### Decision: Result types with diagnostic accumulation
**Rationale**:
- Rust Result type ensures handling
- Accumulation allows multiple errors per parse
- Structured diagnostics enable good UX

**Alternatives considered**:
- Panic on error: Rejected - poor user experience
- Error callbacks: Rejected - complex API
- Silent recovery: Rejected - hides problems

## Source Maps

### Decision: Position tracking in AST with lazy map generation
**Rationale**:
- Always tracking positions has minimal overhead
- Lazy generation only when requested
- Standard source map format for tools

**Alternatives considered**:
- No source maps: Rejected - poor debugging experience
- Eager generation: Rejected - wasteful if not needed
- Custom format: Rejected - tool incompatibility

## Platform Support

### Decision: WASM primary, native secondary via Neon
**Rationale**:
- WASM works everywhere with single build
- Neon for performance-critical native uses
- Clean separation of concerns

**Alternatives considered**:
- Native only: Rejected - no browser support
- WASM only: Rejected - suboptimal server performance
- napi-rs: Rejected - user explicitly excluded

## Resolved Clarifications
All technical aspects from the specification have been researched and decided. No remaining NEEDS CLARIFICATION items.

## Risk Assessment

**Low Risk**:
- CommonMark compliance (well-defined spec)
- WASM toolchain (mature ecosystem)
- Monorepo structure (proven patterns)

**Medium Risk**:
- Performance targets (aggressive but achievable)
- Incremental parsing complexity (mitigated by rope library)
- MDX integration (clear boundaries defined)

**High Risk**:
- None identified

## Next Steps
Proceed to Phase 1: Design contracts, data model, and quickstart guide.