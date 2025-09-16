// Basic usage examples for faster-md
import { renderHtml, parse, renderHtmlSync, initWasm } from 'faster-md';

// Example 1: Simple HTML rendering
async function basicRender() {
  const markdown = `
# Hello World

This is a **bold** text and *italic* text.

- Item 1
- Item 2
- Item 3

[Link to Google](https://google.com)
`;

  const html = await renderHtml(markdown);
  console.log('Basic HTML:', html);
}

// Example 2: Parse to AST
async function parseToAst() {
  const markdown = '# Title\n\nParagraph with **bold**';
  
  const ast = await parse(markdown);
  console.log('AST:', JSON.stringify(ast, null, 2));
}

// Example 3: GFM Extensions
async function gfmFeatures() {
  const markdown = `
## GitHub Flavored Markdown

| Feature | Supported |
|---------|-----------|
| Tables  | ✅        |
| Strikethrough | ✅  |

- [x] Completed task
- [ ] Pending task

~~This text is struck through~~

https://github.com (autolink)
`;

  const html = await renderHtml(markdown, { gfm: true });
  console.log('GFM HTML:', html);
}

// Example 4: Synchronous rendering
async function syncRendering() {
  // Initialize WASM first
  await initWasm();
  
  // Now we can use sync methods
  const html = renderHtmlSync('# Synchronous Rendering');
  console.log('Sync HTML:', html);
}

// Example 5: Custom options
async function customOptions() {
  const markdown = '<script>alert("XSS")</script>\n# Safe Title';
  
  // With sanitization (default)
  const safeHtml = await renderHtml(markdown, {
    sanitize: true,
    gfm: false
  });
  console.log('Sanitized:', safeHtml);
  
  // Without sanitization (dangerous!)
  const unsafeHtml = await renderHtml(markdown, {
    sanitize: false,
    allowDangerousHtml: true
  });
  console.log('Unsanitized:', unsafeHtml);
}

// Example 6: Performance testing
async function performanceTest() {
  const markdown = '# Title\n\n'.repeat(1000) + 'Paragraph text. '.repeat(1000);
  
  console.time('Render 50KB document');
  const html = await renderHtml(markdown);
  console.timeEnd('Render 50KB document');
  
  console.log(`Input size: ${markdown.length} bytes`);
  console.log(`Output size: ${html.length} bytes`);
}

// Run examples
async function main() {
  console.log('=== Basic Rendering ===');
  await basicRender();
  
  console.log('\n=== Parse to AST ===');
  await parseToAst();
  
  console.log('\n=== GFM Features ===');
  await gfmFeatures();
  
  console.log('\n=== Synchronous Rendering ===');
  await syncRendering();
  
  console.log('\n=== Custom Options ===');
  await customOptions();
  
  console.log('\n=== Performance Test ===');
  await performanceTest();
}

main().catch(console.error);