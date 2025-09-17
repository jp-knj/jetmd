# JetMD API Reference

JetMD ships parallel runtimes for Node.js, Rust, WebAssembly, and CLI usage. This guide lists the public entry points that remain stable during Phase 3.x.

## Node Packages (`packages/faster-md`)

```ts
import {
  parse,
  parseSync,
  renderHtml,
  renderHtmlSync,
  sanitizeHtml,
  createParseSession,
  parseIncremental,
  clearSession,
  Processor,
  createProcessor,
  quickProcess,
  quickRender,
  createStream,
  processStream,
} from 'faster-md'
```

### Parsing
- `parse(markdown, options)` → `Promise<AstNode>` (safe default). Supports `{ gfm, frontmatter, mdx, position, sessionId }`.
- `parseSync(markdown, options)` → `AstNode`. Requires a warmed WASM instance (`parse` or `getWasmInstance` must run first).
- `createParseSession(sessionId?)` / `parseIncremental(sessionId, markdown, edits)` enable incremental parsing. Call `clearSession(sessionId)` when done.

### Rendering
- `renderHtml(input, options)` renders markdown or an AST to HTML. Options include `{ sanitize = true, allowDangerousHtml, gfm, xhtml, sanitizeOptions, sessionId }`.
- `renderHtmlSync` mirrors the async API once WASM is initialised.
- `sanitizeHtml(html, sanitizeOptions)` cleans raw HTML; JetMD defaults to a secure wasm-backed sanitizer.
- `quickRender(markdown)` is a convenience wrapper that enables sanitisation and GFM.
- `renderWithCustom(input, renderers)` walks the AST and lets you override node handlers.

### Processor & Streaming
- `createProcessor(options)` returns a reusable pipeline that memoises module state. Options `{ gfm, frontmatter, sanitize, mdx, data }`.
- `Processor` exposes `.parse`, `.render`, `.process`, `.withPlugins`.
- Streaming helpers (`createStream`, `createLineStream`, `createDuplexStream`, `processStream`) convert Node.js streams to incremental renders for large files.

## MDX Compiler (`packages/mdx-compiler`)

```ts
import { compileMdx, tokenizeMdx, parseJsx } from 'mdx-compiler'
```
- `compileMdx(source, options)` returns `{ code, map, diagnostics }` for Vite/ESM builds.
- `tokenizeMdx(source)` emits lexical tokens for editor tooling.
- `parseJsx(source, options)` re-uses the shared JSX grammar for custom transforms.

## Rust Crates
- `crates/fmd-core`: `Document::new`, `parse(document, ProcessorOptions)` produce the AST; feature flags enable GFM and MDX.
- `crates/fmd-html`: `render_html(ast, RenderOptions)` returns sanitised HTML; enabling the `dangerous-html` feature bypasses sanitisation.
- `crates/fmd-cli`: `fmd` binary exposes `fmd parse <file>` and `fmd render <file> --out out.html` with `--gfm/--mdx` flags.

## WASM Bindings (`crates/fmd-wasm`)
- `parseToAst(markdown, options)` and `renderHtml(markdown, options)` mirror the Node wrappers.
- `SessionManager::create_session`, `parse_incremental`, and `clear_session` drive incremental workflows.
- Bundled JS loader (`packages/faster-md/src/loader.js`) caches the singleton `__fmd_wasm_instance` for both browser and Node.js.

## Tooling Notes
- Initialise WASM once per process: `await parse('')` or `await getWasmInstance()`.
- When embedding in browsers, serve `packages/wasm/*.wasm` via a static origin and pass a custom `wasmUrl` to the loader.
- All APIs follow semantic versioning starting at `0.1.x`; breaking changes are flagged in `docs/migration.md` and the Changeset log.
