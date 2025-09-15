# Task Completion Checklist

## Before Marking a Task Complete

### 1. Code Quality Checks
- [ ] Code follows project style conventions
- [ ] No commented-out code or debug statements
- [ ] Functions have appropriate documentation
- [ ] Error handling is robust and informative

### 2. Testing Requirements
- [ ] Unit tests written and passing
- [ ] Integration tests updated if needed
- [ ] Run `cargo test` for Rust code
- [ ] Run `pnpm test` for JavaScript code
- [ ] Conformance tests pass (if touching parser)

### 3. Linting and Formatting
- [ ] Run `cargo fmt` on Rust code
- [ ] Run `cargo clippy` and address warnings
- [ ] Run `pnpm lint` on JavaScript/TypeScript
- [ ] Run `pnpm format` on JS/TS code

### 4. Performance Validation
- [ ] No performance regressions in benchmarks
- [ ] Memory usage within constraints (≤1.5× input typical)
- [ ] Throughput targets met (≥200 MB/s native, ≥50 MB/s WASM)

### 5. Documentation Updates
- [ ] API documentation updated if interfaces changed
- [ ] README updated if usage changed
- [ ] CHANGELOG entry added for notable changes
- [ ] Code comments explain complex logic

### 6. Security Considerations
- [ ] No sensitive data in logs or errors
- [ ] Input validation in place
- [ ] Sanitization enabled by default
- [ ] No unsafe code without justification

### 7. Build Verification
- [ ] `cargo build` succeeds
- [ ] `wasm-pack build` succeeds (for WASM components)
- [ ] `pnpm build` succeeds (once configured)
- [ ] No new warnings introduced

### 8. Final Checks
- [ ] Feature works as specified in spec.md
- [ ] All TODO items addressed or documented
- [ ] Git status clean (no untracked files)
- [ ] Ready for code review

## Special Considerations

### For Parser Changes
- CommonMark conformance ≥99.5%
- GFM tests pass when enabled
- Incremental parsing maintains ≥90% node reuse

### For API Changes
- Backward compatibility maintained or documented
- Migration guide provided for breaking changes
- Version bump planned appropriately

### For Performance-Critical Code
- Benchmarks show improvement or no regression
- Profile-guided optimization considered
- SIMD optimizations explored where applicable