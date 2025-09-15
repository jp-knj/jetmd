# Node.js API Contract

## Main Module (`faster-md`)

### parse
```typescript
export function parse(
  input: string,
  options?: ParseOptions
): ParseResult
```
**Purpose**: Parse Markdown/MDX to AST  
**Input**: Markdown text, optional configuration  
**Output**: AST with diagnostics  
**Errors**: Throws ParseError with details

### renderHtml
```typescript
export function renderHtml(
  input: string | ParseResult,
  options?: RenderOptions
): string
```
**Purpose**: Render to HTML  
**Input**: Markdown or AST  
**Output**: HTML string  
**Errors**: Throws RenderError

### parseFile
```typescript
export async function parseFile(
  path: string,
  options?: ParseOptions
): Promise<ParseResult>
```
**Purpose**: Parse file from disk  
**Input**: File path  
**Output**: Parsed AST  
**Errors**: File I/O or parse errors

### renderFile
```typescript
export async function renderFile(
  path: string,
  options?: RenderOptions
): Promise<string>
```
**Purpose**: Render file to HTML  
**Input**: File path  
**Output**: HTML string  
**Errors**: File I/O or render errors

### stream
```typescript
export function stream(
  options?: StreamOptions
): Transform
```
**Purpose**: Create transform stream  
**Input**: Stream options  
**Output**: Node.js transform stream  
**Usage**: `fs.createReadStream(...).pipe(stream())`

### createProcessor
```typescript
export function createProcessor(
  options?: ProcessorOptions
): Processor
```
**Purpose**: Create reusable processor  
**Input**: Default options  
**Output**: Processor instance

## Processor Class

```typescript
class Processor {
  constructor(options?: ProcessorOptions)
  
  parse(input: string, options?: ParseOptions): ParseResult
  render(ast: ParseResult, options?: RenderOptions): string
  renderHtml(input: string | ParseResult, options?: RenderOptions): string
  
  use(plugin: Plugin): this
  freeze(): this
  
  data(key: string, value?: any): any
}
```

## Plugin System

### Plugin Interface
```typescript
interface Plugin {
  name: string
  phase?: 'parse' | 'transform' | 'render'
  
  // For transform plugins
  transform?(ast: AstNode, file: VFile): void | AstNode | Promise<void | AstNode>
  
  // For syntax plugins  
  syntax?(this: Processor): void
  
  // For render plugins
  render?(node: AstNode, context: RenderContext): string | void
}
```

### Built-in Plugins
```typescript
// GFM support
import { gfm } from 'faster-md/plugins'

// Frontmatter extraction
import { frontmatter } from 'faster-md/plugins'

// Syntax highlighting
import { highlight } from 'faster-md/plugins'

// Slug generation
import { slugger } from 'faster-md/plugins'

// Usage
const processor = createProcessor()
  .use(gfm())
  .use(frontmatter({ format: ['yaml', 'toml'] }))
  .use(highlight({ theme: 'github-dark' }))
  .use(slugger({ prefix: 'heading-' }))
```

## MDX Module (`mdx-compiler`)

### compileMdx
```typescript
export async function compileMdx(
  source: string,
  options?: MdxOptions
): Promise<CompileResult>
```
**Purpose**: Compile MDX to JavaScript  
**Input**: MDX source  
**Output**: ESM module code  
**Errors**: Syntax or compilation errors

### compileMdxFile
```typescript
export async function compileMdxFile(
  path: string,
  options?: MdxOptions
): Promise<CompileResult>
```
**Purpose**: Compile MDX file  
**Input**: File path  
**Output**: ESM module code

### MdxOptions
```typescript
interface MdxOptions {
  jsx?: 'automatic' | 'classic'      // Default: 'automatic'
  jsxRuntime?: string                 // Default: 'react'
  jsxImportSource?: string           // Default: 'react'
  providerImportSource?: string      // MDX provider
  
  remarkPlugins?: Plugin[]           // Markdown plugins
  rehypePlugins?: Plugin[]          // HTML plugins
  
  development?: boolean             // Dev mode
  sourceMap?: boolean              // Generate source map
  
  components?: Record<string, any> // Component mapping
  scope?: Record<string, any>     // Available in expressions
}
```

### CompileResult
```typescript
interface CompileResult {
  code: string                    // ESM JavaScript code
  map?: SourceMap                 // Source map if requested
  frontmatter?: any              // Extracted frontmatter
  diagnostics: Diagnostic[]      // Warnings/errors
  
  // Metadata
  imports: string[]             // Detected imports
  exports: string[]            // Detected exports
  components: string[]         // Used components
}
```

## CLI Module (`faster-md-cli`)

### CLI Commands
```bash
# Parse and render
fmd input.md                    # Output HTML to stdout
fmd input.md -o output.html    # Output to file
fmd input.md --ast             # Output AST as JSON
fmd input.md --format=html     # Explicit format

# Options
fmd input.md --gfm            # Enable GFM
fmd input.md --no-sanitize    # Disable sanitization
fmd input.md --frontmatter    # Extract frontmatter
fmd input.md --watch          # Watch mode

# MDX compilation
fmd input.mdx --mdx           # Compile MDX to JS
fmd input.mdx --jsx=classic   # Use classic JSX

# Batch processing
fmd "**/*.md" -o dist/        # Process multiple files
fmd --stdin                   # Read from stdin

# Information
fmd --version                 # Show version
fmd --help                    # Show help
```

### Programmatic CLI
```typescript
import { cli } from 'faster-md-cli'

await cli(['input.md', '--gfm', '-o', 'output.html'])
```

## Vite Plugin (`faster-md-vite`)

### Configuration
```typescript
// vite.config.ts
import { defineConfig } from 'vite'
import markdown from 'faster-md-vite'
import mdx from 'mdx-vite'

export default defineConfig({
  plugins: [
    markdown({
      include: ['**/*.md'],
      exclude: ['node_modules/**'],
      
      gfm: true,
      frontmatter: true,
      
      transforms: [
        // Custom transforms
      ],
      
      // Wrapper component
      wrapper: 'MarkdownLayout',
      
      // Export frontmatter
      exportFrontmatter: true,
      
      // Logging
      logging: 'ndjson' | 'none'
    }),
    
    mdx({
      include: ['**/*.mdx'],
      jsxRuntime: 'automatic',
      providerImportSource: '@mdx-js/react'
    })
  ]
})
```

### Module Types
```typescript
// For *.md files
declare module '*.md' {
  export const html: string
  export const ast: AstNode
  export const frontmatter: any
  export default html
}

// For *.mdx files  
declare module '*.mdx' {
  export const frontmatter: any
  const MDXContent: (props: any) => JSX.Element
  export default MDXContent
}
```

## Contract Tests

### Node API Tests
```javascript
import { parse, renderHtml, createProcessor } from 'faster-md'

test('parse markdown', () => {
  const result = parse('# Hello')
  expect(result.ast.children[0].type).toBe('heading')
})

test('render HTML', () => {
  const html = renderHtml('**bold**')
  expect(html).toContain('<strong>bold</strong>')
})

test('processor with plugins', () => {
  const processor = createProcessor()
    .use(gfm())
    .use(frontmatter())
  
  const result = processor.parse('---\ntitle: Test\n---\n\n~~strike~~')
  expect(result.frontmatter.title).toBe('Test')
  // GFM strikethrough should work
})

test('stream processing', (done) => {
  const chunks = []
  const s = stream({ gfm: true })
  
  s.on('data', chunk => chunks.push(chunk))
  s.on('end', () => {
    expect(chunks.join('')).toContain('<h1>')
    done()
  })
  
  s.write('# Header\n')
  s.write('Content')
  s.end()
})
```

### MDX Tests
```javascript
import { compileMdx } from 'mdx-compiler'

test('compile MDX', async () => {
  const result = await compileMdx(`
import Button from './Button'

# Hello

<Button>Click me</Button>
  `)
  
  expect(result.code).toContain('import Button')
  expect(result.code).toContain('jsx')
  expect(result.imports).toContain('./Button')
})

test('MDX with expressions', async () => {
  const result = await compileMdx('The answer is {40 + 2}')
  expect(result.code).toContain('40 + 2')
})

test('MDX errors', async () => {
  const result = await compileMdx('<Button>')
  expect(result.diagnostics).toHaveLength(1)
  expect(result.diagnostics[0].code).toMatch(/MDX/)
})
```

### CLI Tests
```javascript
import { exec } from 'child_process'
import { promisify } from 'util'

const execAsync = promisify(exec)

test('CLI basic usage', async () => {
  const { stdout } = await execAsync('fmd test.md')
  expect(stdout).toContain('<h1>')
})

test('CLI with options', async () => {
  const { stdout } = await execAsync('fmd test.md --gfm --ast')
  const ast = JSON.parse(stdout)
  expect(ast.type).toBe('root')
})

test('CLI version', async () => {
  const { stdout } = await execAsync('fmd --version')
  expect(stdout).toMatch(/\d+\.\d+\.\d+/)
})
```

### Vite Plugin Tests
```javascript
import { build } from 'vite'
import markdown from 'faster-md-vite'

test('Vite plugin transforms markdown', async () => {
  const result = await build({
    plugins: [markdown()],
    build: { write: false }
  })
  
  // Check transformed output
  const output = result.output[0]
  expect(output.code).toContain('export const html')
})
```