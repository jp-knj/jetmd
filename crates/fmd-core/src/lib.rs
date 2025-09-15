// Core parser, lexer, and AST for faster-md

use serde::{Deserialize, Serialize};

// Re-export main types
pub use ast::*;
pub use incremental::*;
pub use position::*;

pub mod ast;
pub mod incremental;
pub mod inline;
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
    pub gfm: bool,
    pub frontmatter: bool,
    pub directives: bool,
    pub math: bool,
    pub allow_dangerous_html: bool,
    pub sanitize: bool,
    pub position: bool,
    pub incremental: bool,
    pub track_positions: bool,
    pub gfm_options: GfmOptions,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct GfmOptions {
    pub tables: bool,
    pub strikethrough: bool,
    pub autolinks: bool,
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

#[derive(Debug)]
pub struct IncrementalSession {
    tree: Option<Node>,
}

impl IncrementalSession {
    pub fn new() -> Self {
        Self { tree: None }
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
    use scanner::Scanner;
    use std::time::Instant;

    let start_time = Instant::now();

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Validate document size
    if doc.content.len() > 10 * 1024 * 1024 {
        errors.push("Document exceeds maximum size of 10MB".to_string());
        return ParseResult {
            success: false,
            ast: Node::default(),
            errors,
            warnings,
            frontmatter: None,
            reused_nodes: 0,
            total_nodes: 0,
            changed_ranges: vec![],
            parse_time_ns: start_time.elapsed().as_nanos() as u64,
        };
    }

    // Scan blocks
    let mut scanner = Scanner::new(&doc.content, options.position);
    let block_tokens = scanner.scan_blocks();

    // Build AST from tokens
    let mut root = Node {
        node_type: NodeType::Root,
        children: Vec::new(),
        ..Default::default()
    };

    let mut total_nodes = 1; // Count root

    for token in block_tokens {
        let node = build_node_from_token(token, &options);
        total_nodes += count_nodes(&node);
        root.children.push(node);
    }

    // Add warnings for dangerous HTML if needed
    if options.allow_dangerous_html {
        warnings.push("Dangerous HTML is allowed - ensure input is trusted".to_string());
    }

    ParseResult {
        success: errors.is_empty(),
        ast: root,
        errors,
        warnings,
        frontmatter: None,
        reused_nodes: 0,
        total_nodes,
        changed_ranges: vec![],
        parse_time_ns: start_time.elapsed().as_nanos() as u64,
    }
}

fn build_node_from_token(token: scanner::BlockToken, options: &ProcessorOptions) -> Node {
    use inline::InlineParser;
    use scanner::BlockTokenType;

    match token.token_type {
        BlockTokenType::Heading(depth) => {
            let mut parser = InlineParser::new(token.content, options.position);
            Node {
                node_type: NodeType::Heading,
                depth: Some(depth),
                children: parser.parse(),
                position: token.position,
                ..Default::default()
            }
        }
        BlockTokenType::Paragraph => {
            let mut parser = InlineParser::new(token.content, options.position);
            Node {
                node_type: NodeType::Paragraph,
                children: parser.parse(),
                position: token.position,
                ..Default::default()
            }
        }
        BlockTokenType::ThematicBreak => Node {
            node_type: NodeType::ThematicBreak,
            position: token.position,
            ..Default::default()
        },
        BlockTokenType::Blockquote => {
            // Recursively parse blockquote content
            let inner_doc = Document::new(&token.content);
            let inner_result = parse(&inner_doc, options.clone());
            Node {
                node_type: NodeType::Blockquote,
                children: inner_result.ast.children,
                position: token.position,
                ..Default::default()
            }
        }
        BlockTokenType::ListItem { ordered, .. } => {
            // Parse list item content
            let inner_doc = Document::new(&token.content);
            let inner_result = parse(&inner_doc, options.clone());
            Node {
                node_type: NodeType::ListItem,
                children: inner_result.ast.children,
                ordered: Some(ordered),
                position: token.position,
                ..Default::default()
            }
        }
        BlockTokenType::CodeFence { lang, meta } => Node {
            node_type: NodeType::Code,
            value: Some(token.content),
            lang,
            meta,
            position: token.position,
            ..Default::default()
        },
        BlockTokenType::Html => Node {
            node_type: NodeType::Html,
            value: Some(token.content),
            position: token.position,
            ..Default::default()
        },
    }
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

    // Calculate diff to find unchanged sections
    let diff = calculate_diff(&doc.content, &doc.content); // Would compare with previous content

    // Try to reuse nodes from cache
    let content_hash = content_hash(&doc.content);
    if let Some(cached_node) = cache.get(content_hash) {
        reused_nodes = count_nodes(cached_node);
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
