# Project Structure

## Monorepo Layout
```
jetmd/
├── crates/                 # Rust packages
│   ├── fmd-core/          # Core parser, lexer, AST operations
│   ├── fmd-gfm/           # GitHub Flavored Markdown extensions
│   ├── fmd-html/          # HTML renderer with sanitization
│   ├── fmd-slug/          # Stable slug generation
│   ├── fmd-wasm/          # WASM bindings via wasm-bindgen
│   ├── fmd-cli/           # Rust CLI tool
│   └── mdx-core/          # MDX token detection
│
├── packages/              # Node.js packages
│   ├── faster-md/         # Node.js wrapper for WASM
│   ├── mdx-compiler/      # MDX to ESM transformation
│   ├── mdx-runtime/       # Runtime components for MDX
│   └── faster-md-vite/    # Vite plugin (future)
│
├── tests/
│   ├── conformance/       # CommonMark/GFM spec compliance tests
│   ├── fixtures/          # Test documents
│   └── benchmarks/        # Performance benchmarks
│
├── specs/                 # Feature specifications
│   └── 001-faster-md-mdx/ # Current feature branch specs
│       ├── spec.md        # Feature specification
│       ├── plan.md        # Implementation plan
│       ├── data-model.md  # Data structures and API
│       ├── quickstart.md  # Usage examples
│       ├── research.md    # Technical decisions
│       └── contracts/     # API contracts
│
└── .specify/              # Project tooling
    ├── scripts/           # Build and development scripts
    ├── templates/         # File templates
    └── memory/            # Project constitution
```

## Key Files
- `CLAUDE.md`: AI agent context and instructions
- `Cargo.toml`: Rust workspace configuration (to be created)
- `package.json`: Node.js workspace configuration (to be created)
- `pnpm-workspace.yaml`: Workspace definition (to be created)