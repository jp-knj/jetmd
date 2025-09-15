# Data Model: faster-md & mdx

**Date**: 2025-09-15  
**Feature**: High-performance Markdown/MDX processor  
**Branch**: 001-faster-md-mdx

## Core Entities

### Document
**Purpose**: Input text to be processed  
**Fields**:
- content: string (raw Markdown/MDX text)
- source: string (optional, file path or identifier)
- options: ProcessorOptions (configuration)

**Validation**:
- content must not exceed configured max size (default 10MB)
- UTF-8 encoding required

### ProcessorOptions
**Purpose**: Configuration for parsing and rendering  
**Fields**:
- gfm: boolean (enable GitHub Flavored Markdown)
- frontmatter: boolean (parse YAML/TOML frontmatter)
- directives: boolean (enable directive syntax)
- math: boolean (enable math syntax passthrough)
- allowDangerousHtml: boolean (default false)
- sanitize: SanitizeOptions (default enabled)
- slugger: SluggerFunction (optional custom slugger)
- position: boolean (track source positions)
- incremental: boolean (enable incremental parsing)
- format: 'html' | 'ast' | 'events' (output format)

**Validation**:
- If allowDangerousHtml is true, warn about security implications
- Options must be serializable (no functions except slugger)

### FMDAST (Node)
**Purpose**: Abstract syntax tree representation  
**Base Fields** (all nodes):
- type: string (node type identifier)
- position: Position (optional, source location)
- data: Record<string, any> (optional, metadata)

**Node Types**:

#### Block Nodes
- root: { children: Node[] }
- paragraph: { children: Node[] }
- heading: { depth: 1-6, children: Node[] }
- blockquote: { children: Node[] }
- list: { ordered: boolean, start?: number, tight: boolean, children: Node[] }
- listItem: { checked?: boolean, children: Node[] }
- code: { lang?: string, meta?: string, value: string }
- html: { value: string }
- thematicBreak: {}
- table: { align: ('left'|'right'|'center'|null)[], children: Node[] }
- tableRow: { children: Node[] }
- tableCell: { children: Node[] }
- definition: { identifier: string, url: string, title?: string }
- footnoteDefinition: { identifier: string, children: Node[] }
- frontmatter: { format: 'yaml'|'toml', value: string }

#### Inline Nodes
- text: { value: string }
- emphasis: { children: Node[] }
- strong: { children: Node[] }
- delete: { children: Node[] }
- link: { url: string, title?: string, children: Node[] }
- image: { url: string, title?: string, alt: string }
- inlineCode: { value: string }
- break: {}
- footnoteReference: { identifier: string }

#### Directive Nodes
- containerDirective: { name: string, attributes: Record<string, string>, children: Node[] }
- leafDirective: { name: string, attributes: Record<string, string> }
- textDirective: { name: string, attributes: Record<string, string> }

#### MDX Nodes
- mdxjsEsm: { value: string, data: { estree: Program } }
- mdxJsxFlowElement: { name: string, attributes: MdxJsxAttribute[], children: Node[] }
- mdxJsxTextElement: { name: string, attributes: MdxJsxAttribute[], children: Node[] }
- mdxFlowExpression: { value: string, data: { estree: Expression } }
- mdxTextExpression: { value: string, data: { estree: Expression } }

**Validation**:
- Tree must be acyclic
- Parent-child relationships must be valid per type
- Position.start must be before position.end

### Position
**Purpose**: Source location tracking  
**Fields**:
- start: Point (beginning location)
- end: Point (ending location)
- source?: string (optional source identifier)

### Point
**Purpose**: Specific location in source  
**Fields**:
- line: number (1-indexed)
- column: number (1-indexed)
- offset: number (0-indexed byte offset)

**Validation**:
- line >= 1
- column >= 1
- offset >= 0

### MdxJsxAttribute
**Purpose**: JSX element attributes in MDX  
**Fields**:
- type: 'mdxJsxAttribute'
- name: string
- value: string | MdxJsxAttributeValueExpression

### MdxJsxAttributeValueExpression
**Purpose**: JavaScript expression as attribute value  
**Fields**:
- type: 'mdxJsxAttributeValueExpression'
- value: string
- data: { estree: Expression }

### Plugin
**Purpose**: Extension point for customization  
**Fields**:
- name: string (unique identifier)
- phase: 'syntax' | 'transform' | 'render'
- priority: number (execution order)
- handler: Function (phase-specific signature)

**Validation**:
- name must be unique within phase
- priority determines execution order (lower first)
- handler must be pure function (transform phase)

### Diagnostic
**Purpose**: Error and warning reporting  
**Fields**:
- code: string (e.g., 'MDX1001')
- severity: 'error' | 'warning' | 'info'
- message: string (human-readable)
- position?: Position (location in source)
- source?: string (which file/input)

**Validation**:
- code must follow pattern: /^[A-Z]+\d{4}$/
- message must be non-empty

### ParseResult
**Purpose**: Output of parsing operation  
**Fields**:
- ast?: FMDAST (if successful)
- diagnostics: Diagnostic[]
- frontmatter?: any (parsed frontmatter data)
- time?: number (parse time in ms)

### RenderResult
**Purpose**: Output of rendering operation  
**Fields**:
- output: string (HTML, serialized AST, or events)
- diagnostics: Diagnostic[]
- sourceMap?: SourceMap (if requested)
- time?: number (render time in ms)

### IncrementalSession
**Purpose**: Stateful parsing context  
**Fields**:
- id: string (unique session identifier)
- rope: RopeStructure (internal text storage)
- ast: FMDAST (current parse tree)
- invalidRanges: Range[] (pending reparse regions)
- nodeCache: Map<string, Node> (reusable nodes)

**Validation**:
- id must be unique across active sessions
- invalidRanges must not overlap
- nodeCache entries must have valid checksums

### SourceMap
**Purpose**: Mapping between input and output positions  
**Fields**:
- version: 3 (source map version)
- sources: string[] (input file names)
- sourcesContent?: string[] (original content)
- mappings: string (VLQ encoded mappings)
- names: string[] (symbol names)

## State Transitions

### Document Processing States
```
RAW -> PARSING -> PARSED -> TRANSFORMING -> TRANSFORMED -> RENDERING -> RENDERED
         ↓          ↓           ↓              ↓             ↓
       ERROR     ERROR       ERROR         ERROR         ERROR
```

### Incremental Session States
```
INIT -> ACTIVE -> DIRTY -> REPARSING -> CLEAN
          ↑         ↓          ↓          ↓
          ←─────────←──────────←──────────┘
```

### Plugin Execution States
```
REGISTERED -> VALIDATING -> READY -> EXECUTING -> COMPLETE
                  ↓           ↓          ↓
               INVALID    DISABLED    FAILED
```

## Relationships

### Component Relationships
```
Document ──uses──> ProcessorOptions
    ↓
  parses to
    ↓
ParseResult ──contains──> FMDAST
    ↓                       ↓
  may have              has many
    ↓                       ↓
Diagnostics             Nodes (recursive)
                            ↓
                        has optional
                            ↓
                        Position
```

### Plugin Integration
```
ProcessorOptions ──registers──> Plugin[]
                                   ↓
                              transforms
                                   ↓
                                FMDAST
```

### MDX Flow
```
MDX Document ──parse──> FMDAST with MDX nodes
                            ↓
                        transform
                            ↓
                    ESM Module string
                            ↓
                    runtime evaluation
                            ↓
                    Rendered Component
```

## Constraints

### Performance Constraints
- AST node creation must use arena allocation
- Position tracking overhead < 5% of parse time
- Plugin execution must not block main parse
- Incremental reparse must reuse ≥90% unchanged nodes

### Memory Constraints
- Document size limit: 10MB default (configurable)
- AST memory usage ≤ 1.5× input size (typical)
- Maximum nesting depth: 100 levels
- Node cache size: 10,000 nodes max

### Security Constraints
- No code execution during parse/compile
- HTML sanitization enabled by default
- URL validation for links and images
- Plugin code must be sandboxed (future)

## Validation Rules

### Document Level
- Input must be valid UTF-8
- No null bytes in content
- Line endings normalized to LF

### AST Level
- Tree must be well-formed (valid parent-child)
- No circular references
- All positions must be within document bounds

### MDX Level
- JSX must have matching open/close tags
- JavaScript expressions must be syntactically valid
- ESM imports/exports at top level only

### Output Level
- HTML must be well-formed
- All dangerous content sanitized (unless opted out)
- Source maps must have valid mappings