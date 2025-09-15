# Performance Targets and Constraints

## Throughput Requirements
- **Native (Rust)**: ≥200 MB/s
- **WASM**: ≥50 MB/s
- **Benchmark**: Based on 50KB document processing

## Latency Targets
- **50KB Document Processing**:
  - P50: <3ms
  - P99: <15ms
  - Maximum: <25ms

## Memory Constraints
- **Typical Usage**: ≤1.5× input size
- **Worst Case**: ≤3× input size
- **AST Size**: Optimized for minimal allocations
- **Incremental Parsing**: ≥90% node reuse on edits

## WASM-Specific Targets
- **Bundle Size**: <200KB gzipped
- **Initialization**: <50ms cold start
- **Memory Growth**: Linear, predictable
- **Optimization**: wasm-opt for 20-30% size reduction

## Optimization Strategies
1. **SIMD Acceleration**: For line boundary detection
2. **Rope Data Structure**: O(log n) edits and lookups
3. **Zero-Copy Operations**: Where possible
4. **Lazy Evaluation**: For expensive computations
5. **Cache-Friendly**: Optimize data locality

## Benchmark Suite
- CommonMark specification documents
- Real-world documents (README files, documentation)
- Synthetic stress tests (deeply nested, many links)
- Incremental edit scenarios
- Memory pressure tests

## Regression Detection
- Automated benchmarks in CI
- Alert on >5% performance degradation
- Profile before/after for major changes
- Track metrics over time