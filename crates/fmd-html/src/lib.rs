// HTML renderer for faster-md AST

use fmd_core::{Node, NodeType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HtmlOptions {
    pub sanitize: bool,
    pub allow_dangerous_html: bool,
    pub sanitize_options: SanitizeOptions,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SanitizeOptions {
    pub allow_javascript_urls: bool,
}

pub fn render_html(ast: &Node, options: HtmlOptions) -> String {
    // TODO: Implement HTML rendering
    to_html(ast, options)
}

pub fn to_html(_ast: &Node, _options: HtmlOptions) -> String {
    // TODO: Implement HTML generation
    String::from("<p>HTML renderer not implemented</p>")
}