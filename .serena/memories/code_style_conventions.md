# Code Style and Conventions

## Rust Code Style
- **Style**: Follow standard Rust conventions (rustfmt defaults)
- **Naming**: snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE_CASE for constants
- **Error Handling**: Use Result<T, E> for fallible operations
- **Documentation**: Doc comments (///) for public APIs
- **Testing**: Unit tests in same file (#[cfg(test)] mod tests)
- **Performance**: Prefer zero-copy operations, avoid unnecessary allocations
- **Safety**: Minimize unsafe code, document invariants when used

## JavaScript/TypeScript Style
- **Module System**: ESM only (import/export)
- **Node Version**: Target Node.js ≥20
- **TypeScript**: Use strict mode, explicit types for public APIs
- **Async**: Prefer async/await over callbacks
- **Error Handling**: Throw Error objects with descriptive messages
- **Testing**: Vitest for unit/integration tests
- **Documentation**: JSDoc comments for public functions

## API Design Principles
1. **No Environment Variables**: All configuration via options objects
2. **Pure Functions**: Plugins must be pure functions
3. **Position Tracking**: All AST nodes include position information
4. **Security Default**: Sanitization enabled by default
5. **Explicit Options**: No magic, all behavior controlled by options

## Project Conventions
- **Monorepo Structure**: Separate crates/ for Rust, packages/ for Node
- **Feature Branches**: Named as XXX-feature-name (e.g., 001-faster-md-mdx)
- **Specifications First**: Write spec.md before implementation
- **Test-Driven Development**: Write tests before implementation
- **Library-First**: Every feature as a standalone library
- **CLI Interface**: Every library exposes CLI functionality

## Documentation Standards
- **README.md**: Each package/crate has its own README
- **API Docs**: Generated from code comments
- **Examples**: Quickstart guide with practical examples
- **Changelog**: Track changes per package

## Git Conventions
- **Commit Messages**: Descriptive, present tense
- **Branch Protection**: Main branch protected, PR required
- **Reviews**: Code review before merge
- **CI/CD**: Tests must pass before merge