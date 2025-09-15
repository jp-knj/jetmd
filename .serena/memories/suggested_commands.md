# Suggested Commands

## Build Commands
```bash
# Build all packages (once implemented)
pnpm build           # Build JavaScript packages and WASM
cargo build          # Build Rust crates
wasm-pack build      # Build WASM module specifically

# Build individual crates
cargo build -p fmd-core
cargo build -p fmd-wasm
```

## Test Commands
```bash
# Run all tests
pnpm test                # JavaScript/TypeScript tests
cargo test               # Rust tests
pnpm test:conformance    # CommonMark/GFM spec compliance

# Test individual components
cargo test -p fmd-core
pnpm test --filter faster-md
```

## Development Commands
```bash
# Watch mode
pnpm dev             # Watch JavaScript/TypeScript
cargo watch          # Watch Rust files (requires cargo-watch)

# Linting and formatting (to be configured)
cargo fmt            # Format Rust code
cargo clippy         # Rust linter
pnpm lint           # JavaScript/TypeScript linting
pnpm format         # Format JS/TS code
```

## Benchmark Commands
```bash
# Performance testing
cargo bench          # Rust benchmarks
pnpm bench          # JavaScript benchmarks
```

## Git Commands
```bash
# Check current branch
git rev-parse --abbrev-ref HEAD

# Feature branch status
bash .specify/scripts/bash/get-feature-paths.sh
```

## Project Setup Scripts
```bash
# Create new feature
bash .specify/scripts/bash/create-new-feature.sh

# Setup plan for current feature
bash .specify/scripts/bash/setup-plan.sh

# Update agent context
bash .specify/scripts/bash/update-agent-context.sh
```

## System Commands (macOS/Darwin)
```bash
# File operations
ls -la              # List files with details
find . -name "*.rs" # Find Rust files
grep -r "pattern"   # Search in files (use ripgrep 'rg' for better performance)

# Process monitoring
ps aux | grep node  # Check Node processes
lsof -i :3000      # Check port usage
```

## CLI Usage (once implemented)
```bash
# Basic markdown conversion
fmd input.md > output.html

# With options
fmd document.md --gfm --frontmatter

# Watch mode
fmd input.md --watch -o output.html

# MDX compilation
fmd component.mdx --mdx > component.js
```