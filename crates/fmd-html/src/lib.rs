// HTML renderer for faster-md AST

pub mod visitor;
pub mod sanitize;

use fmd_core::Node;
use serde::{Deserialize, Serialize};
use visitor::HtmlVisitor;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HtmlOptions {
    pub sanitize: bool,
    pub allow_dangerous_html: bool,
    #[serde(skip)]
    pub sanitize_options: sanitize::SanitizeOptions,
}

pub fn render_html(ast: &Node, options: HtmlOptions) -> String {
    to_html(ast, options)
}

pub fn to_html(ast: &Node, options: HtmlOptions) -> String {
    // Generate HTML using visitor pattern
    let mut visitor = HtmlVisitor::new();
    visitor.visit(ast);
    let html = visitor.finish();
    
    // Sanitize if needed
    if options.sanitize && !options.allow_dangerous_html {
        sanitize::sanitize_html(&html, &options.sanitize_options)
    } else {
        html
    }
}

/// Render HTML with default options
pub fn render_html_default(ast: &Node) -> String {
    render_html(ast, HtmlOptions::default())
}

/// Render HTML without sanitization (dangerous!)
pub fn render_html_unsafe(ast: &Node) -> String {
    render_html(ast, HtmlOptions {
        sanitize: false,
        allow_dangerous_html: true,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use fmd_core::NodeType;
    
    #[test]
    fn test_render_paragraph() {
        let ast = Node {
            node_type: NodeType::Root,
            children: vec![
                Node {
                    node_type: NodeType::Paragraph,
                    children: vec![
                        Node {
                            node_type: NodeType::Text,
                            value: Some("Hello world".to_string()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        };
        
        let html = render_html_default(&ast);
        assert!(html.contains("<p>Hello world</p>"));
    }
}