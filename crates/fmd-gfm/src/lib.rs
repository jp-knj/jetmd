// GitHub Flavored Markdown extensions for faster-md

pub mod autolink;
pub mod strikethrough;
pub mod table;

use fmd_core::Node;
use serde::{Deserialize, Serialize};

/// GFM configuration options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GfmOptions {
    pub tables: bool,
    pub strikethrough: bool,
    pub autolinks: bool,
    pub tasklists: bool,
}

impl Default for GfmOptions {
    fn default() -> Self {
        Self {
            tables: true,
            strikethrough: true,
            autolinks: true,
            tasklists: true,
        }
    }
}

pub struct GfmExtensions {
    enable_tables: bool,
    enable_strikethrough: bool,
    enable_autolinks: bool,
    #[allow(dead_code)]
    enable_task_lists: bool,
}

impl GfmExtensions {
    pub fn new() -> Self {
        Self::all()
    }

    /// Enable all GFM extensions
    pub fn all() -> Self {
        Self {
            enable_tables: true,
            enable_strikethrough: true,
            enable_autolinks: true,
            enable_task_lists: true,
        }
    }

    /// Create with specific extensions
    pub fn with_options(
        tables: bool,
        strikethrough: bool,
        autolinks: bool,
        task_lists: bool,
    ) -> Self {
        Self {
            enable_tables: tables,
            enable_strikethrough: strikethrough,
            enable_autolinks: autolinks,
            enable_task_lists: task_lists,
        }
    }

    /// Parse table from lines
    pub fn parse_table(&self, lines: &[String]) -> Option<Node> {
        if !self.enable_tables {
            return None;
        }

        let parser = table::TableParser::new(false);
        parser.parse_table(lines, 0)
    }

    /// Parse strikethrough text
    pub fn parse_strikethrough(&self, input: &str, pos: usize) -> Option<(Node, usize)> {
        if !self.enable_strikethrough {
            return None;
        }

        strikethrough::parse_strikethrough(input, pos)
    }

    /// Parse autolink
    pub fn parse_autolink(&self, input: &str, pos: usize) -> Option<(Node, usize)> {
        if !self.enable_autolinks {
            return None;
        }

        // Try URL autolink
        if let Some(result) = autolink::AutolinkParser::parse_url_autolink(input, pos) {
            return Some(result);
        }

        // Try email autolink
        autolink::AutolinkParser::parse_email_autolink(input, pos)
    }

    /// Process text with GFM extensions
    pub fn process_inline(&self, text: &str) -> Vec<Node> {
        let mut nodes = Vec::new();

        // Apply autolinks first
        let current_nodes = if self.enable_autolinks {
            autolink::AutolinkParser::process_autolinks(text)
        } else {
            vec![Node {
                node_type: fmd_core::NodeType::Text,
                value: Some(text.to_string()),
                ..Default::default()
            }]
        };

        // Apply strikethrough to text nodes
        if self.enable_strikethrough {
            for node in current_nodes {
                if node.node_type == fmd_core::NodeType::Text {
                    if let Some(value) = node.value {
                        nodes.extend(strikethrough::process_strikethrough(&value));
                    }
                } else {
                    nodes.push(node);
                }
            }
        } else {
            nodes = current_nodes;
        }

        nodes
    }

    /// Check if lines might be a table
    pub fn is_table(&self, lines: &[&str]) -> bool {
        self.enable_tables && table::is_table(lines)
    }
}

impl Default for GfmExtensions {
    fn default() -> Self {
        Self::all()
    }
}
