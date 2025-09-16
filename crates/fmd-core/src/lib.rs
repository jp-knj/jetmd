// Core parser, lexer, and AST for faster-md

use serde::{Deserialize, Serialize};

// Re-export main types
pub use ast::*;
pub use error::{ErrorCollector, ParseError, ParseErrorKind};
pub use incremental::*;
pub use position::*;

pub mod ast;
pub mod error;
pub mod incremental;
pub mod inline;
pub mod parser_impl;
pub mod position;
pub mod rope;
pub mod scanner;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub content: String,
    pub source: Option<String>,
}

impl Document {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            source: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ProcessorOptions {
    #[serde(default)]
    pub gfm: bool,
    #[serde(default)]
    pub frontmatter: bool,
    #[serde(default)]
    pub directives: bool,
    #[serde(default)]
    pub math: bool,
    #[serde(default)]
    pub allow_dangerous_html: bool,
    #[serde(default)]
    pub sanitize: bool,
    #[serde(default)]
    pub position: bool,
    #[serde(default)]
    pub incremental: bool,
    #[serde(default)]
    pub track_positions: bool,
    #[serde(default)]
    pub gfm_options: GfmOptions,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct GfmOptions {
    #[serde(default)]
    pub tables: bool,
    #[serde(default)]
    pub strikethrough: bool,
    #[serde(default)]
    pub autolinks: bool,
    #[serde(default)]
    pub tasklists: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub success: bool,
    pub ast: Node,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub frontmatter: Option<serde_json::Value>,
    pub reused_nodes: usize,
    pub total_nodes: usize,
    pub changed_ranges: Vec<Range>,
    pub parse_time_ns: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edit {
    pub range: Range,
    pub text: String,
}

#[derive(Debug, Default)]
pub struct IncrementalSession {
    tree: Option<Node>,
}

impl IncrementalSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_tree(&self) -> bool {
        self.tree.is_some()
    }

    pub fn reset(&mut self) {
        self.tree = None;
    }

    pub fn apply_edits(&mut self, _edits: &[Edit]) {
        // TODO: Implement incremental editing
    }
}

// Main parsing functions
pub fn parse(doc: &Document, options: ProcessorOptions) -> ParseResult {
    use parser_impl::parse_with_pulldown;

    // Use web-sys for timing in WASM, std::time::Instant for native
    #[cfg(target_arch = "wasm32")]
    let start_time = web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0);

    #[cfg(not(target_arch = "wasm32"))]
    let start_time = std::time::Instant::now();

    let mut error_collector = ErrorCollector::new();
    let mut warnings = Vec::new();

    // Validate document size
    if doc.content.len() > 10 * 1024 * 1024 {
        let error = ParseError::new(
            ParseErrorKind::Custom("Document too large".to_string()),
            "Document exceeds maximum size of 10MB",
        );
        error_collector.add_error(error);
        return ParseResult {
            success: false,
            ast: Node::default(),
            errors: vec!["Document exceeds maximum size of 10MB".to_string()],
            warnings,
            frontmatter: None,
            reused_nodes: 0,
            total_nodes: 0,
            changed_ranges: vec![],
            parse_time_ns: get_elapsed_ns(start_time),
        };
    }

    // Parse using pulldown-cmark
    let ast = parse_with_pulldown(doc, options);
    let total_nodes = count_nodes(&ast);

    // Extract frontmatter if present
    let frontmatter = if options.frontmatter {
        extract_frontmatter(&ast)
    } else {
        None
    };

    // Add warnings for dangerous HTML if needed
    if options.allow_dangerous_html {
        let warning = ParseError::new(
            ParseErrorKind::Custom("Dangerous HTML".to_string()),
            "Dangerous HTML is allowed - ensure input is trusted",
        );
        error_collector.add_warning(warning);
        warnings.push("Dangerous HTML is allowed - ensure input is trusted".to_string());
    }

    // Convert errors from ErrorCollector to strings for backward compatibility
    let errors: Vec<String> = error_collector
        .errors()
        .iter()
        .map(|e| e.to_string())
        .collect();

    ParseResult {
        success: !error_collector.has_errors(),
        ast,
        errors,
        warnings,
        frontmatter,
        reused_nodes: 0,
        total_nodes,
        changed_ranges: vec![],
        parse_time_ns: get_elapsed_ns(start_time),
    }
}

// Helper function to get elapsed time in nanoseconds
#[cfg(target_arch = "wasm32")]
fn get_elapsed_ns(start_time: f64) -> u64 {
    // Get current time and calculate difference
    let end_time = web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0);
    // Performance.now() returns milliseconds, convert to nanoseconds
    ((end_time - start_time) * 1_000_000.0) as u64
}

#[cfg(not(target_arch = "wasm32"))]
fn get_elapsed_ns(start_time: std::time::Instant) -> u64 {
    start_time.elapsed().as_nanos() as u64
}

fn extract_frontmatter(ast: &Node) -> Option<serde_json::Value> {
    // Look for YAML frontmatter node in the AST
    if let Some(first_child) = ast.children.first() {
        if first_child.node_type == NodeType::Yaml {
            if let Some(yaml_content) = &first_child.value {
                // Try to parse the YAML content as JSON for now
                // In a real implementation, we'd use a YAML parser
                return Some(serde_json::json!({
                    "raw": yaml_content
                }));
            }
        }
    }
    None
}

fn count_nodes(node: &Node) -> usize {
    1 + node.children.iter().map(count_nodes).sum::<usize>()
}

pub fn parse_incremental(
    doc: &Document,
    options: ProcessorOptions,
    previous_ast: &Node,
    cache: &mut IncrementalCache,
) -> ParseResult {
    use incremental::{calculate_diff, content_hash};

    // For now, do a full parse but track what could be reused
    let mut reused_nodes = 0;

    // Calculate diff to find unchanged sections (would compare with previous content)
    let diff = calculate_diff(&doc.content, &doc.content);

    // Try to reuse nodes from cache
    let content_hash = content_hash(&doc.content);
    if let Some(cached_node) = cache.get(content_hash) {
        reused_nodes = count_nodes(cached_node);
        // If we have an exact match in cache, return it
        if reused_nodes == count_nodes(previous_ast) {
            return ParseResult {
                success: true,
                ast: cached_node.clone(),
                errors: vec![],
                warnings: vec![],
                frontmatter: extract_frontmatter(cached_node),
                reused_nodes,
                total_nodes: reused_nodes,
                changed_ranges: vec![],
                parse_time_ns: 0, // Instant cache hit
            };
        }
    }

    // Do full parse for now
    let mut result = parse(doc, options);

    // Store in cache for future reuse
    cache.put(content_hash, result.ast.clone());

    // Update reuse statistics
    result.reused_nodes = reused_nodes;
    result.changed_ranges = diff
        .changed
        .into_iter()
        .map(|(start, end)| Range { start, end })
        .collect();

    result
}
