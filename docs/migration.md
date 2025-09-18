# Migration Guide

This guide explains how to adopt JetMD 0.1.x in projects that currently depend on other Markdown stacks or on the earlier prototype crates.

## From Node Pipelines (remark/markdown-it)
1. Install JetMD packages: `pnpm add faster-md mdx-compiler`.
2. Replace parser imports:
   ```ts
   // Before
   import { unified } from 'unified'
   import remarkParse from 'remark-parse'
   import remarkGfm from 'remark-gfm'

   // After
   import { parse, renderHtml } from 'faster-md'
   ```
3. Map options:
   - `remark-gfm` → pass `{ gfm: true }` to `parse`/`renderHtml`.
   - Custom sanitizers → supply `sanitize: false` and post-process with your sanitizer, or adjust `sanitizeOptions`.
4. For MDX pipelines, call `compileMdx` and feed the returned `code` into your bundler (Vite, Next.js). JSX components now flow through the shared tokenizer and JSX parser.

## From Prototype Rust Crates
- Rename crate imports to the new workspace layout: `faster_md_core` → `fmd_core`, `faster_md_html` → `fmd_html`.
- Update parser calls:
  ```rust
  use fmd_core::{Document, parse, ProcessorOptions};
  use fmd_html::render_html;

  let doc = Document::new(source);
  let ast = parse(&doc, ProcessorOptions { gfm: true, ..Default::default() })?;
  let html = render_html(&ast.ast, Default::default());
  ```
- Incremental parsing now requires the `SessionManager` from `fmd_wasm`; reuse session IDs between edits to unlock partial recomputation.

## CLI Changes
- Install the Rust CLI with `cargo install --path crates/fmd-cli`.
- Commands have been renamed to verbs: `fmd parse input.md --json`, `fmd render input.md --out output.html`.
- Flags:
  - `--gfm` toggles GitHub extensions.
  - `--mdx` enables JSX parsing.
  - `--no-sanitize` bypasses HTML sanitisation (dangerous).

## WebAssembly Deployment
- The WASM bundle now lives in `packages/wasm/jetmd.wasm`. Serve it from a static origin and configure:
  ```ts
  import { getWasmInstance } from 'faster-md'

  await getWasmInstance({ wasmUrl: '/static/jetmd.wasm' })
  ```
- If you relied on inline base64 WASM, move to streamed loading; JetMD requires `WebAssembly.instantiateStreaming` or a manual `fetch` fallback.

## Breaking Change Checklist
- ✅ Functions renamed to `renderHtml`, `sanitizeHtml`, and `createProcessor` for consistency.
- ✅ Sanitisation is enabled by default; set `allowDangerousHtml: true` when you trust the source.
- ✅ All exported types adopt camelCase property names (`sessionId`, `allowDangerousHtml`). Adjust TypeScript definitions accordingly.
- ⚠️ Incremental parsing serialises edit sets as JSON. If you stored binary patches, convert them to the `{ start, end, text }` tuple the WASM expects.
- ⚠️ Benchmarks target ≥50 MB/s; re-run `tests/benchmarks/wasm_perf.js` after upgrading to verify local regressions.

Record any additional incompatibilities in your Changeset and update `STATUS.md` so Phase 3.5 stays aligned.
