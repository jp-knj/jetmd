# faster-md

High-performance Markdown/MDX processor with WASM-first architecture. CommonMark compliant with opt-in GFM extensions.

## Installation

```bash
npm install faster-md
```

## Quick Start

```javascript
import { renderHtml, parse } from 'faster-md';

// Simple HTML rendering
const html = await renderHtml('# Hello World');

// Parse to AST
const ast = await parse('# Hello World');

// With GFM extensions
const gfmHtml = await renderHtml('~~strikethrough~~', { gfm: true });
```

## Features

- **High Performance**: 36-45 MB/s throughput via WASM (optimizing to 50+ MB/s)
- **CommonMark Compliant**: 99%+ spec compliance
- **GFM Extensions**: Tables, strikethrough, autolinks, task lists
- **Type-Safe**: Full TypeScript support
- **Security First**: Built-in HTML sanitization
- **Smart Caching**: Automatic result caching for repeated renders
- **Incremental Parsing**: Session-based caching for real-time editing (coming soon)

## API Reference

See the [full API documentation](./docs/API.md) for detailed usage.

### `renderHtml(markdown, options?)`

Render Markdown to HTML.

```javascript
const html = await renderHtml('# Title\n\nParagraph', {
  gfm: true,        // Enable GitHub Flavored Markdown
  sanitize: true,   // Sanitize HTML output (default: true)
  xhtml: false      // Use XHTML self-closing tags
});
```

### `parse(markdown, options?)`

Parse Markdown to Abstract Syntax Tree (AST).

```javascript
const ast = await parse('# Title', {
  gfm: false,       // Enable GFM extensions
  position: true    // Include position information
});
```

### `renderHtmlSync(markdown, options?)`

Synchronous rendering (requires pre-initialization).

```javascript
// First initialize WASM
await initWasm();

// Then use sync methods
const html = renderHtmlSync('# Title');
```

### `sanitizeHtml(html, options?)`

Sanitize HTML string.

```javascript
const safe = await sanitizeHtml('<script>alert("XSS")</script><p>Safe</p>');
// Output: <p>Safe</p>
```

## GFM Extensions

Enable GitHub Flavored Markdown features:

```javascript
const html = await renderHtml(`
| Header | Column |
|--------|--------|
| Cell   | Cell   |

- [x] Task completed
- [ ] Task pending

~~strikethrough text~~

https://example.com (autolink)
`, { gfm: true });
```

## Performance

Benchmarks on M1 MacBook Pro:

- **Throughput**: 36-45 MB/s (optimizing to 50+ MB/s)
- **50KB document**: ~1.1ms (p50), <3ms (p99)
- **Memory usage**: ~1.5× input size
- **Cache hit rate**: >90% for repeated renders

## Advanced Usage

### Custom Renderers

```javascript
import { renderWithCustom } from 'faster-md';

const html = await renderWithCustom('# Title', {
  heading: (node) => {
    return `<h${node.depth} class="custom">${node.children}</h${node.depth}>`;
  }
});
```

### Incremental Parsing

```javascript
import { createParseSession, parseIncremental } from 'faster-md';

// Create session
const sessionId = await createParseSession();

// Parse incrementally
const ast1 = await parseIncremental(sessionId, 'Version 1');
const ast2 = await parseIncremental(sessionId, 'Version 2');
```

## Node.js Usage

```javascript
import { renderHtml } from 'faster-md';

// Works in Node.js 20+
const html = await renderHtml('# Hello from Node.js');
```

## Browser Usage

```html
<script type="module">
import { renderHtml } from 'https://cdn.jsdelivr.net/npm/faster-md/dist/index.js';

const html = await renderHtml('# Hello from Browser');
document.body.innerHTML = html;
</script>
```

## TypeScript

Full TypeScript support with type definitions:

```typescript
import { renderHtml, ParseOptions, RenderOptions } from 'faster-md';

const options: RenderOptions = {
  gfm: true,
  sanitize: true
};

const html: string = await renderHtml('# Title', options);
```

## Security

By default, HTML output is sanitized to prevent XSS attacks:

```javascript
// Dangerous HTML is removed
const safe = await renderHtml('<script>alert("XSS")</script>');
// Output: (empty)

// Allow dangerous HTML (use with caution!)
const unsafe = await renderHtml('<div onclick="alert()">Click</div>', {
  sanitize: false,
  allowDangerousHtml: true
});
```

## License

MIT

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for development setup and guidelines.