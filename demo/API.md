# API Documentation

## Core Functions

### `renderHtml(markdown, options)`

Markdownを**HTML**に変換します。

#### Parameters

- `markdown` (string): 入力Markdown
- `options` (object): オプション
  - `gfm` (boolean): GFM有効化
  - `sanitize` (boolean): サニタイズ

#### Returns

Promise<string> - HTML文字列

#### Example

```typescript
const html = await renderHtml(
  '# Title\n\nParagraph',
  { gfm: true }
);
```

### `parse(markdown, options)`

MarkdownをASTに変換します。

> **Note**: このAPIは内部実装に依存します

#### Code Example

```javascript
const ast = await parse('# Hello');
console.log(ast);
// Output:
// {
//   type: 'root',
//   children: [{
//     type: 'heading',
//     depth: 1,
//     children: [{
//       type: 'text',
//       value: 'Hello'
//     }]
//   }]
// }
```

## Advanced Usage

### Custom Renderers

```javascript
import { renderWithCustom } from 'faster-md';

const html = await renderWithCustom(markdown, {
  heading: (node) => {
    const level = node.depth;
    const text = getTextContent(node);
    return `<h${level} id="${slugify(text)}">${text}</h${level}>`;
  }
});
```

---

[Back to README](./README.md) | [GitHub](https://github.com)