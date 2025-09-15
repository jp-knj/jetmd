# Quickstart: faster-md & mdx

## Installation

### Node.js Package
```bash
npm install faster-md
# or
pnpm add faster-md
# or
yarn add faster-md
```

### MDX Support
```bash
npm install mdx-compiler mdx-runtime
```

### CLI Tool
```bash
npm install -g faster-md-cli
# or use npx
npx fmd input.md
```

## Basic Usage

### 1. Parse Markdown to HTML
```javascript
import { renderHtml } from 'faster-md'

const markdown = `
# Hello World

This is **bold** and this is *italic*.

- List item 1
- List item 2
`

const html = renderHtml(markdown)
console.log(html)
// Output: <h1>Hello World</h1><p>This is <strong>bold</strong>...
```

### 2. Parse to AST
```javascript
import { parse } from 'faster-md'

const markdown = '# Header\n\nParagraph with [link](https://example.com)'
const result = parse(markdown)

console.log(result.ast)
// {
//   type: 'root',
//   children: [
//     { type: 'heading', depth: 1, children: [...] },
//     { type: 'paragraph', children: [...] }
//   ]
// }
```

### 3. Enable GFM Extensions
```javascript
import { renderHtml } from 'faster-md'

const markdown = `
| Feature | Status |
|---------|--------|
| Tables | ✓ |
| ~~Strikethrough~~ | ✓ |
| Task lists | ✓ |

- [x] Completed task
- [ ] Pending task
`

const html = renderHtml(markdown, { gfm: true })
```

### 4. Extract Frontmatter
```javascript
import { parse } from 'faster-md'

const markdown = `---
title: My Document
date: 2025-01-15
tags: [markdown, tutorial]
---

# Content starts here
`

const result = parse(markdown, { frontmatter: true })
console.log(result.frontmatter)
// { title: 'My Document', date: '2025-01-15', tags: ['markdown', 'tutorial'] }
```

### 5. MDX with Components
```javascript
import { compileMdx } from 'mdx-compiler'

const mdx = `
import Button from './Button'
export const meta = { author: 'John' }

# Interactive Document

<Button onClick={() => alert('Clicked!')}>
  Click me
</Button>

The answer is {40 + 2}.
`

const { code } = await compileMdx(mdx)
// Returns ES module code that can be imported
```

### 6. Command Line Usage
```bash
# Convert markdown to HTML
fmd README.md > output.html

# Enable GFM extensions
fmd document.md --gfm

# Output AST as JSON
fmd input.md --ast > ast.json

# Watch mode for development
fmd input.md --watch -o output.html

# Process MDX
fmd component.mdx --mdx > component.js
```

### 7. Vite Integration
```javascript
// vite.config.js
import { defineConfig } from 'vite'
import markdown from 'faster-md-vite'

export default defineConfig({
  plugins: [
    markdown({
      gfm: true,
      frontmatter: true
    })
  ]
})

// In your app
import content, { frontmatter } from './post.md'
console.log(frontmatter.title)
document.getElementById('content').innerHTML = content
```

### 8. Custom Processor with Plugins
```javascript
import { createProcessor } from 'faster-md'
import { gfm, frontmatter, highlight } from 'faster-md/plugins'

const processor = createProcessor()
  .use(gfm())
  .use(frontmatter({ format: ['yaml', 'toml'] }))
  .use(highlight({ theme: 'github-dark' }))

const result = processor.parse(`
---
title: Code Example
---

\`\`\`javascript
console.log('Syntax highlighted!')
\`\`\`

~~Strikethrough~~ works with GFM plugin.
`)

const html = processor.render(result)
```

### 9. Streaming Large Files
```javascript
import { stream } from 'faster-md'
import { createReadStream, createWriteStream } from 'fs'
import { pipeline } from 'stream/promises'

await pipeline(
  createReadStream('large-document.md'),
  stream({ gfm: true }),
  createWriteStream('output.html')
)
```

### 10. Security: Sanitization
```javascript
import { renderHtml } from 'faster-md'

// Dangerous HTML is sanitized by default
const unsafe = `
<script>alert('XSS')</script>
<img src=x onerror="alert('XSS')">
[link](javascript:alert('XSS'))
`

const safe = renderHtml(unsafe)
// Scripts and dangerous attributes are removed

// To allow raw HTML (use with caution!)
const raw = renderHtml(unsafe, {
  allowDangerousHtml: true,
  sanitize: false  // Must explicitly disable both
})
```

## Performance Tips

### 1. Reuse Processors
```javascript
// Create once, use many times
const processor = createProcessor({ gfm: true })

// ✅ Good
for (const doc of documents) {
  const html = processor.renderHtml(doc)
}

// ❌ Bad - creates new processor each time
for (const doc of documents) {
  const html = renderHtml(doc, { gfm: true })
}
```

### 2. Use Incremental Parsing for Editors
```javascript
import { createSession, parseIncremental } from 'faster-md'

// Initialize session
createSession('editor-1', initialContent)

// On each edit
function handleEdit(start, end, newText) {
  const delta = parseIncremental('editor-1', {
    start,
    end,
    text: newText
  })
  // Update UI with delta.changes
}
```

### 3. Stream for Large Files
```javascript
// For files > 1MB, use streaming
import { stream } from 'faster-md'

readStream
  .pipe(stream())
  .pipe(writeStream)
```

## Common Patterns

### Blog Post with Metadata
```javascript
import { parse } from 'faster-md'
import { readFile } from 'fs/promises'

async function loadPost(path) {
  const content = await readFile(path, 'utf-8')
  const { ast, frontmatter } = parse(content, {
    frontmatter: true,
    gfm: true
  })
  
  return {
    title: frontmatter.title,
    date: new Date(frontmatter.date),
    tags: frontmatter.tags || [],
    content: renderHtml(ast)
  }
}
```

### Documentation Site
```javascript
// Process all markdown files
import { globby } from 'globby'
import { parseFile, renderHtml } from 'faster-md'

const files = await globby('docs/**/*.md')
const pages = await Promise.all(
  files.map(async (file) => {
    const result = await parseFile(file, {
      gfm: true,
      frontmatter: true
    })
    
    return {
      path: file,
      title: result.frontmatter?.title || 'Untitled',
      html: renderHtml(result.ast),
      toc: extractHeadings(result.ast)
    }
  })
)
```

### React MDX Component
```jsx
import { compileMdx } from 'mdx-compiler'
import { useState, useEffect } from 'react'

function MDXDocument({ source }) {
  const [Component, setComponent] = useState(null)
  
  useEffect(() => {
    compileMdx(source).then(({ code }) => {
      // Dynamically import the compiled module
      const module = new Function('React', code)
      setComponent(() => module(React).default)
    })
  }, [source])
  
  if (!Component) return <div>Loading...</div>
  return <Component />
}
```

## Verification Tests

After installation, verify everything works:

```javascript
// test-setup.js
import { parse, renderHtml } from 'faster-md'
import { compileMdx } from 'mdx-compiler'

// Test 1: Basic parsing
const ast = parse('# Test')
console.assert(ast.ast.children[0].type === 'heading', 'Parse failed')

// Test 2: HTML rendering
const html = renderHtml('**bold**')
console.assert(html.includes('<strong>'), 'Render failed')

// Test 3: GFM support
const gfmHtml = renderHtml('~~strike~~', { gfm: true })
console.assert(gfmHtml.includes('<del>'), 'GFM failed')

// Test 4: MDX compilation
const { code } = await compileMdx('<Button />')
console.assert(code.includes('Button'), 'MDX failed')

console.log('✅ All tests passed!')
```

## Troubleshooting

### WASM not loading in browser
```javascript
// Ensure proper initialization
import init, { parse } from 'faster-md-wasm'

await init() // Must await before using
const result = parse('# Test')
```

### Performance issues
```javascript
// Check these settings
const options = {
  position: false,  // Disable if not needed
  incremental: true // Enable for editors
}
```

### MDX compilation errors
```javascript
// Always check diagnostics
const { code, diagnostics } = await compileMdx(source)
if (diagnostics.length > 0) {
  console.warn('MDX warnings:', diagnostics)
}
```

## Next Steps

- Read the [API documentation](contracts/) for detailed interface specifications
- Review [data model](data-model.md) for AST structure
- Check [research notes](research.md) for design decisions
- Run benchmark suite: `npm run bench`
- Join community: [GitHub Discussions](#)