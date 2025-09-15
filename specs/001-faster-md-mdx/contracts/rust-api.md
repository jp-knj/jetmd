# Rust API Contract

## Core Functions

### parse
```rust
pub fn parse(input: &str, options: Options) -> Result<Ast, Error>
```
**Purpose**: Parse Markdown/MDX input into AST  
**Input**: UTF-8 text, configuration options  
**Output**: AST or error with diagnostics  
**Errors**: ParseError with position and message

### parse_incremental
```rust
pub fn parse_incremental(
    session_id: &str,
    patch: TextPatch
) -> Result<AstDelta, Error>
```
**Purpose**: Incrementally update existing parse  
**Input**: Session ID, text changes  
**Output**: AST changes or error  
**Errors**: SessionNotFound, InvalidPatch

### to_html
```rust
pub fn to_html(ast: &Ast, options: HtmlOptions) -> Result<String, Error>
```
**Purpose**: Render AST to HTML  
**Input**: AST, HTML options  
**Output**: HTML string  
**Errors**: RenderError with node info

### to_html_stream
```rust
pub fn to_html_stream(
    ast: &Ast,
    options: HtmlOptions
) -> impl Stream<Item = Result<String, Error>>
```
**Purpose**: Stream HTML output  
**Input**: AST, HTML options  
**Output**: Async stream of HTML chunks  
**Errors**: RenderError per chunk

### to_events
```rust
pub fn to_events(ast: &Ast) -> impl Iterator<Item = Event>
```
**Purpose**: Convert AST to event stream  
**Input**: AST  
**Output**: Iterator of parse events  
**Errors**: None (infallible)

## Types

### Options
```rust
pub struct Options {
    pub gfm: bool,
    pub frontmatter: bool,
    pub directives: bool,
    pub math: bool,
    pub allow_dangerous_html: bool,
    pub position: bool,
    pub source: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            gfm: false,
            frontmatter: true,
            directives: false,
            math: false,
            allow_dangerous_html: false,
            position: true,
            source: None,
        }
    }
}
```

### HtmlOptions
```rust
pub struct HtmlOptions {
    pub sanitize: bool,
    pub sanitize_policy: Option<SanitizePolicy>,
    pub slugger: Option<Box<dyn Slugger>>,
    pub highlight: Option<Box<dyn Highlighter>>,
}

impl Default for HtmlOptions {
    fn default() -> Self {
        Self {
            sanitize: true,
            sanitize_policy: None,
            slugger: None,
            highlight: None,
        }
    }
}
```

### Ast
```rust
pub struct Ast {
    pub root: Node,
    pub definitions: HashMap<String, Definition>,
    pub footnotes: HashMap<String, FootnoteDefinition>,
}
```

### Node
```rust
pub enum Node {
    // Block nodes
    Root { children: Vec<Node>, position: Option<Position> },
    Paragraph { children: Vec<Node>, position: Option<Position> },
    Heading { depth: u8, children: Vec<Node>, position: Option<Position> },
    BlockQuote { children: Vec<Node>, position: Option<Position> },
    List { ordered: bool, start: Option<u32>, tight: bool, children: Vec<Node>, position: Option<Position> },
    ListItem { checked: Option<bool>, children: Vec<Node>, position: Option<Position> },
    Code { lang: Option<String>, meta: Option<String>, value: String, position: Option<Position> },
    Html { value: String, position: Option<Position> },
    ThematicBreak { position: Option<Position> },
    
    // Inline nodes
    Text { value: String, position: Option<Position> },
    Emphasis { children: Vec<Node>, position: Option<Position> },
    Strong { children: Vec<Node>, position: Option<Position> },
    Delete { children: Vec<Node>, position: Option<Position> },
    Link { url: String, title: Option<String>, children: Vec<Node>, position: Option<Position> },
    Image { url: String, title: Option<String>, alt: String, position: Option<Position> },
    InlineCode { value: String, position: Option<Position> },
    Break { position: Option<Position> },
    
    // Extensions
    Table { align: Vec<Option<Align>>, children: Vec<Node>, position: Option<Position> },
    TableRow { children: Vec<Node>, position: Option<Position> },
    TableCell { children: Vec<Node>, position: Option<Position> },
    Frontmatter { format: FrontmatterFormat, value: String, position: Option<Position> },
    Math { value: String, display: bool, position: Option<Position> },
    
    // MDX
    MdxJsxElement { name: String, attributes: Vec<MdxAttribute>, children: Vec<Node>, position: Option<Position> },
    MdxExpression { value: String, position: Option<Position> },
    MdxEsm { value: String, position: Option<Position> },
}
```

### Position
```rust
pub struct Position {
    pub start: Point,
    pub end: Point,
}

pub struct Point {
    pub line: usize,    // 1-indexed
    pub column: usize,  // 1-indexed  
    pub offset: usize,  // 0-indexed byte offset
}
```

### Error
```rust
pub enum Error {
    Parse(ParseError),
    Render(RenderError),
    Session(SessionError),
}

pub struct ParseError {
    pub code: String,
    pub message: String,
    pub position: Option<Position>,
    pub source: Option<String>,
}

pub struct RenderError {
    pub message: String,
    pub node_type: String,
}

pub enum SessionError {
    NotFound(String),
    InvalidPatch,
}
```

## Traits

### Slugger
```rust
pub trait Slugger: Send + Sync {
    fn slug(&mut self, text: &str) -> String;
}
```

### Highlighter
```rust
pub trait Highlighter: Send + Sync {
    fn highlight(&self, code: &str, lang: Option<&str>) -> String;
}
```

### Plugin
```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn transform(&self, ast: &mut Ast) -> Result<(), Error>;
}
```

## Contract Tests

### Parse Tests
```rust
#[test]
fn test_parse_empty() {
    let result = parse("", Options::default());
    assert!(result.is_ok());
    let ast = result.unwrap();
    assert_eq!(ast.root.children().len(), 0);
}

#[test]
fn test_parse_paragraph() {
    let result = parse("Hello world", Options::default());
    assert!(result.is_ok());
    let ast = result.unwrap();
    assert_eq!(ast.root.children().len(), 1);
}

#[test]
fn test_parse_with_position() {
    let opts = Options { position: true, ..Default::default() };
    let result = parse("# Header", opts);
    assert!(result.is_ok());
    let ast = result.unwrap();
    // Verify position is set
}

#[test]
fn test_parse_gfm_disabled() {
    let result = parse("~~strike~~", Options::default());
    // Should not parse as strikethrough
}

#[test]
fn test_parse_gfm_enabled() {
    let opts = Options { gfm: true, ..Default::default() };
    let result = parse("~~strike~~", opts);
    // Should parse as strikethrough
}
```

### HTML Tests
```rust
#[test]
fn test_html_sanitize_default() {
    let ast = parse("<script>alert(1)</script>", Options::default()).unwrap();
    let html = to_html(&ast, HtmlOptions::default()).unwrap();
    assert!(!html.contains("<script>"));
}

#[test]
fn test_html_dangerous_allowed() {
    let ast = parse("<div>test</div>", Options { allow_dangerous_html: true, ..Default::default() }).unwrap();
    let opts = HtmlOptions { sanitize: false, ..Default::default() };
    let html = to_html(&ast, opts).unwrap();
    assert!(html.contains("<div>"));
}
```

### Error Tests
```rust
#[test]
fn test_parse_error_position() {
    // Test malformed MDX
    let opts = Options { ..Default::default() };
    let result = parse("<Component", opts);
    assert!(result.is_err());
    match result {
        Err(Error::Parse(e)) => {
            assert!(e.position.is_some());
            assert!(e.code.starts_with("MDX"));
        }
        _ => panic!("Expected parse error"),
    }
}
```