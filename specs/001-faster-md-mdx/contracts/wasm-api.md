# WASM API Contract (wasm-bindgen)

## Exported Functions

### parseToAst
```typescript
export function parseToAst(
  input: string,
  options?: ParseOptions
): AstResult
```
**Purpose**: Parse Markdown/MDX to AST  
**Input**: Markdown text, optional configuration  
**Output**: JSON-serialized AST or error  
**Errors**: Throws with diagnostic information

### renderHtml
```typescript
export function renderHtml(
  input: string | AstResult,
  options?: RenderOptions
): string
```
**Purpose**: Render to HTML  
**Input**: Markdown text or parsed AST  
**Output**: HTML string  
**Errors**: Throws with render error

### parseIncremental
```typescript
export function parseIncremental(
  sessionId: string,
  patch: TextPatch
): AstDelta
```
**Purpose**: Incremental parsing  
**Input**: Session ID and text changes  
**Output**: AST changes  
**Errors**: Throws if session not found

### createSession
```typescript
export function createSession(
  id: string,
  content: string,
  options?: ParseOptions
): void
```
**Purpose**: Initialize incremental session  
**Input**: Unique ID, initial content  
**Output**: None (side effect)  
**Errors**: Throws if ID already exists

### destroySession
```typescript
export function destroySession(id: string): void
```
**Purpose**: Clean up session  
**Input**: Session ID  
**Output**: None  
**Errors**: Silent if not found

### getVersion
```typescript
export function getVersion(): string
```
**Purpose**: Get library version  
**Output**: Semantic version string

## Types (TypeScript definitions)

### ParseOptions
```typescript
interface ParseOptions {
  gfm?: boolean              // Default: false
  frontmatter?: boolean       // Default: true
  directives?: boolean        // Default: false
  math?: boolean             // Default: false
  allowDangerousHtml?: boolean // Default: false
  position?: boolean         // Default: true
  source?: string           // Optional source identifier
}
```

### RenderOptions
```typescript
interface RenderOptions {
  sanitize?: boolean         // Default: true
  sanitizePolicy?: SanitizePolicy
  slugger?: 'github' | 'simple' | 'none' // Default: 'github'
  highlight?: boolean        // Default: false
  highlightTheme?: string    // Default: 'github'
}
```

### AstResult
```typescript
interface AstResult {
  ast: AstNode
  diagnostics: Diagnostic[]
  frontmatter?: any
  stats?: {
    parseTime: number
    nodeCount: number
  }
}
```

### AstNode
```typescript
type AstNode = 
  | RootNode
  | BlockNode
  | InlineNode
  | MdxNode

interface BaseNode {
  type: string
  position?: Position
  data?: Record<string, any>
}

interface ParentNode extends BaseNode {
  children: AstNode[]
}

interface RootNode extends ParentNode {
  type: 'root'
}

interface ParagraphNode extends ParentNode {
  type: 'paragraph'
}

interface HeadingNode extends ParentNode {
  type: 'heading'
  depth: 1 | 2 | 3 | 4 | 5 | 6
}

interface TextNode extends BaseNode {
  type: 'text'
  value: string
}

interface CodeNode extends BaseNode {
  type: 'code'
  lang?: string
  meta?: string
  value: string
}

interface LinkNode extends ParentNode {
  type: 'link'
  url: string
  title?: string
}

interface ImageNode extends BaseNode {
  type: 'image'
  url: string
  title?: string
  alt: string
}

// ... other node types follow similar pattern
```

### Position
```typescript
interface Position {
  start: Point
  end: Point
}

interface Point {
  line: number      // 1-indexed
  column: number    // 1-indexed
  offset: number    // 0-indexed
}
```

### Diagnostic
```typescript
interface Diagnostic {
  code: string      // e.g., 'MDX1001'
  severity: 'error' | 'warning' | 'info'
  message: string
  position?: Position
  source?: string
}
```

### TextPatch
```typescript
interface TextPatch {
  start: number     // byte offset
  end: number       // byte offset
  text: string      // replacement text
}
```

### AstDelta
```typescript
interface AstDelta {
  changes: NodeChange[]
  diagnostics: Diagnostic[]
  stats?: {
    reuseRatio: number
    parseTime: number
  }
}

interface NodeChange {
  type: 'insert' | 'delete' | 'update'
  path: number[]    // tree path to node
  node?: AstNode    // for insert/update
}
```

## Contract Tests (JavaScript/TypeScript)

### Basic Parsing
```javascript
import { parseToAst, renderHtml } from 'faster-md-wasm'

test('parse empty document', () => {
  const result = parseToAst('')
  expect(result.ast.type).toBe('root')
  expect(result.ast.children).toHaveLength(0)
  expect(result.diagnostics).toHaveLength(0)
})

test('parse paragraph', () => {
  const result = parseToAst('Hello world')
  expect(result.ast.children).toHaveLength(1)
  expect(result.ast.children[0].type).toBe('paragraph')
})

test('parse with GFM', () => {
  const result = parseToAst('~~strike~~', { gfm: true })
  // Should contain delete node
  const para = result.ast.children[0]
  expect(para.children[0].type).toBe('delete')
})

test('parse without GFM', () => {
  const result = parseToAst('~~strike~~', { gfm: false })
  // Should be plain text
  const para = result.ast.children[0]
  expect(para.children[0].type).toBe('text')
  expect(para.children[0].value).toContain('~~')
})
```

### HTML Rendering
```javascript
test('render HTML with sanitization', () => {
  const html = renderHtml('<script>alert(1)</script>')
  expect(html).not.toContain('<script>')
})

test('render HTML without sanitization', () => {
  const html = renderHtml('<div>test</div>', { 
    sanitize: false 
  })
  expect(html).toContain('<div>test</div>')
})

test('render with custom slugger', () => {
  const html = renderHtml('# Hello World', {
    slugger: 'simple'
  })
  expect(html).toContain('id="hello-world"')
})
```

### Position Tracking
```javascript
test('track positions', () => {
  const result = parseToAst('# Header\n\nParagraph', {
    position: true
  })
  
  const heading = result.ast.children[0]
  expect(heading.position).toBeDefined()
  expect(heading.position.start.line).toBe(1)
  expect(heading.position.start.column).toBe(1)
  expect(heading.position.end.line).toBe(1)
  
  const para = result.ast.children[1]
  expect(para.position.start.line).toBe(3)
})
```

### Error Handling
```javascript
test('parse error with position', () => {
  const result = parseToAst('<Component')
  expect(result.diagnostics).toHaveLength(1)
  expect(result.diagnostics[0].code).toMatch(/^MDX/)
  expect(result.diagnostics[0].position).toBeDefined()
})

test('invalid options', () => {
  expect(() => {
    parseToAst('test', { unknownOption: true })
  }).not.toThrow() // Unknown options ignored
})
```

### Incremental Parsing
```javascript
test('incremental session', () => {
  createSession('test-1', '# Hello\n\nWorld')
  
  const delta = parseIncremental('test-1', {
    start: 8,
    end: 8,
    text: '\n\nNew paragraph'
  })
  
  expect(delta.changes.length).toBeGreaterThan(0)
  expect(delta.stats.reuseRatio).toBeGreaterThan(0.5)
  
  destroySession('test-1')
})

test('session not found', () => {
  expect(() => {
    parseIncremental('nonexistent', { start: 0, end: 0, text: '' })
  }).toThrow(/session not found/i)
})
```

### Frontmatter
```javascript
test('parse YAML frontmatter', () => {
  const input = `---
title: Test
date: 2025-01-01
---

# Content`

  const result = parseToAst(input, { frontmatter: true })
  expect(result.frontmatter).toEqual({
    title: 'Test',
    date: '2025-01-01'
  })
})
```

### MDX Support
```javascript
test('parse JSX component', () => {
  const result = parseToAst('<Button>Click me</Button>')
  const mdx = result.ast.children[0]
  expect(mdx.type).toBe('mdxJsxFlowElement')
  expect(mdx.name).toBe('Button')
})

test('parse JavaScript expression', () => {
  const result = parseToAst('The answer is {40 + 2}')
  const para = result.ast.children[0]
  const expr = para.children[1]
  expect(expr.type).toBe('mdxTextExpression')
})
```

## WASM Initialization

### Browser
```javascript
import init, { parseToAst } from 'faster-md-wasm'

await init() // Async initialization
const result = parseToAst('# Hello')
```

### Node.js
```javascript
import { parseToAst } from 'faster-md-wasm'

// Synchronous initialization handled automatically
const result = parseToAst('# Hello')
```

### Bundler (Vite/Webpack)
```javascript
import init, { parseToAst } from 'faster-md-wasm'

// Bundler handles WASM loading
await init()
const result = parseToAst('# Hello')
```