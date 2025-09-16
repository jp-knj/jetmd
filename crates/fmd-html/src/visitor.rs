// Visitor pattern for HTML generation

use fmd_core::{Node, NodeType};
use std::fmt::Write;

/// HTML visitor for AST traversal
pub struct HtmlVisitor {
    output: String,
    in_pre: bool,
    list_stack: Vec<ListContext>,
}

#[derive(Debug, Clone)]
struct ListContext {
    #[allow(dead_code)]
    ordered: bool,
    #[allow(dead_code)]
    start: usize,
    tight: bool,
}

impl Default for HtmlVisitor {
    fn default() -> Self {
        Self {
            output: String::with_capacity(1024),
            in_pre: false,
            list_stack: Vec::new(),
        }
    }
}

impl HtmlVisitor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Visit a node and generate HTML
    pub fn visit(&mut self, node: &Node) {
        match node.node_type {
            NodeType::Root => {
                for child in &node.children {
                    self.visit(child);
                }
            }
            NodeType::Paragraph => {
                // Don't wrap in <p> if in tight list
                let in_tight_list = self.list_stack.last().is_some_and(|ctx| ctx.tight);
                if !in_tight_list {
                    self.output.push_str("<p>");
                }
                for child in &node.children {
                    self.visit(child);
                }
                if !in_tight_list {
                    self.output.push_str("</p>\n");
                } else {
                    self.output.push('\n');
                }
            }
            NodeType::Heading => {
                let depth = node.depth.unwrap_or(1);
                write!(self.output, "<h{}>", depth).unwrap();
                for child in &node.children {
                    self.visit(child);
                }
                writeln!(self.output, "</h{}>", depth).unwrap();
            }
            NodeType::ThematicBreak => {
                self.output.push_str("<hr />\n");
            }
            NodeType::Blockquote => {
                self.output.push_str("<blockquote>\n");
                for child in &node.children {
                    self.visit(child);
                }
                self.output.push_str("</blockquote>\n");
            }
            NodeType::List => {
                let ordered = node.ordered.unwrap_or(false);
                let tight = self.is_tight_list(node);

                self.list_stack.push(ListContext {
                    ordered,
                    start: 1,
                    tight,
                });

                if ordered {
                    self.output.push_str("<ol>\n");
                } else {
                    self.output.push_str("<ul>\n");
                }

                for child in &node.children {
                    self.visit(child);
                }

                if ordered {
                    self.output.push_str("</ol>\n");
                } else {
                    self.output.push_str("</ul>\n");
                }

                self.list_stack.pop();
            }
            NodeType::ListItem => {
                self.output.push_str("<li>");

                // Check if item has checkbox
                if let Some(checked) = node.checked {
                    if checked {
                        self.output
                            .push_str("<input type=\"checkbox\" checked disabled /> ");
                    } else {
                        self.output
                            .push_str("<input type=\"checkbox\" disabled /> ");
                    }
                }

                for child in &node.children {
                    self.visit(child);
                }
                self.output.push_str("</li>\n");
            }
            NodeType::Code => {
                self.output.push_str("<pre>");
                if let Some(lang) = &node.lang {
                    write!(
                        self.output,
                        "<code class=\"language-{}\">",
                        escape_html(lang)
                    )
                    .unwrap();
                } else {
                    self.output.push_str("<code>");
                }

                if let Some(value) = &node.value {
                    self.output.push_str(&escape_html(value));
                }

                self.output.push_str("</code></pre>\n");
            }
            NodeType::Html => {
                if let Some(value) = &node.value {
                    self.output.push_str(value);
                }
            }
            NodeType::Text => {
                if let Some(value) = &node.value {
                    if self.in_pre {
                        self.output.push_str(value);
                    } else {
                        self.output.push_str(&escape_html(value));
                    }
                }
            }
            NodeType::Emphasis => {
                self.output.push_str("<em>");
                for child in &node.children {
                    self.visit(child);
                }
                self.output.push_str("</em>");
            }
            NodeType::Strong => {
                self.output.push_str("<strong>");
                for child in &node.children {
                    self.visit(child);
                }
                self.output.push_str("</strong>");
            }
            NodeType::InlineCode => {
                self.output.push_str("<code>");
                if let Some(value) = &node.value {
                    self.output.push_str(&escape_html(value));
                }
                self.output.push_str("</code>");
            }
            NodeType::Break => {
                self.output.push_str("<br />\n");
            }
            NodeType::Link => {
                self.output.push_str("<a");
                if let Some(url) = &node.url {
                    write!(self.output, " href=\"{}\"", escape_attr(url)).unwrap();
                }
                if let Some(title) = &node.title {
                    write!(self.output, " title=\"{}\"", escape_attr(title)).unwrap();
                }
                self.output.push('>');
                for child in &node.children {
                    self.visit(child);
                }
                self.output.push_str("</a>");
            }
            NodeType::Image => {
                self.output.push_str("<img");
                if let Some(url) = &node.url {
                    write!(self.output, " src=\"{}\"", escape_attr(url)).unwrap();
                }
                if let Some(alt) = &node.alt {
                    write!(self.output, " alt=\"{}\"", escape_attr(alt)).unwrap();
                }
                if let Some(title) = &node.title {
                    write!(self.output, " title=\"{}\"", escape_attr(title)).unwrap();
                }
                self.output.push_str(" />");
            }
            NodeType::LinkReference | NodeType::ImageReference => {
                // These should be resolved before rendering
                // For now, render as text
                for child in &node.children {
                    self.visit(child);
                }
            }
            NodeType::Definition => {
                // Definitions are not rendered
            }
            // GFM Extensions
            NodeType::Table => {
                self.output.push_str("<table>\n");

                // Separate thead and tbody
                let mut in_header = true;
                let mut thead_content = String::new();
                let mut tbody_content = String::new();

                for child in &node.children {
                    if child.node_type == NodeType::TableRow {
                        let mut row_visitor = HtmlVisitor::new();
                        row_visitor.visit(child);

                        if in_header {
                            thead_content.push_str(&row_visitor.output);
                            in_header = false;
                        } else {
                            tbody_content.push_str(&row_visitor.output);
                        }
                    }
                }

                if !thead_content.is_empty() {
                    self.output.push_str("<thead>\n");
                    self.output.push_str(&thead_content);
                    self.output.push_str("</thead>\n");
                }

                if !tbody_content.is_empty() {
                    self.output.push_str("<tbody>\n");
                    self.output.push_str(&tbody_content);
                    self.output.push_str("</tbody>\n");
                }

                self.output.push_str("</table>\n");
            }
            NodeType::TableRow => {
                self.output.push_str("<tr>");
                for child in &node.children {
                    self.visit(child);
                }
                self.output.push_str("</tr>\n");
            }
            NodeType::TableCell => {
                // Check if header cell (simplified - would need context)
                let tag = "td"; // Would be "th" for header cells
                write!(self.output, "<{}>", tag).unwrap();
                for child in &node.children {
                    self.visit(child);
                }
                write!(self.output, "</{}>", tag).unwrap();
            }
            NodeType::Delete => {
                self.output.push_str("<del>");
                for child in &node.children {
                    self.visit(child);
                }
                self.output.push_str("</del>");
            }
            NodeType::FootnoteDefinition => {
                // Footnotes need special handling
                if let Some(id) = &node.identifier {
                    writeln!(
                        self.output,
                        "<div class=\"footnote\" id=\"fn-{}\">",
                        escape_attr(id)
                    )
                    .unwrap();
                    for child in &node.children {
                        self.visit(child);
                    }
                    self.output.push_str("</div>\n");
                }
            }
            NodeType::FootnoteReference => {
                if let Some(id) = &node.identifier {
                    write!(
                        self.output,
                        "<sup><a href=\"#fn-{}\">{}</a></sup>",
                        escape_attr(id),
                        escape_html(id)
                    )
                    .unwrap();
                }
            }
            // MDX nodes - render as comments for now
            NodeType::MdxjsEsm
            | NodeType::MdxJsxFlowElement
            | NodeType::MdxJsxTextElement
            | NodeType::MdxFlowExpression
            | NodeType::MdxTextExpression => {
                write!(self.output, "<!-- MDX: {:?} -->", node.node_type).unwrap();
            }
            // Directives
            NodeType::ContainerDirective | NodeType::LeafDirective | NodeType::TextDirective => {
                self.output.push_str("<div class=\"directive\">");
                for child in &node.children {
                    self.visit(child);
                }
                self.output.push_str("</div>");
            }
            // Math
            NodeType::Math => {
                self.output.push_str("<div class=\"math math-display\">");
                if let Some(value) = &node.value {
                    self.output.push_str(&escape_html(value));
                }
                self.output.push_str("</div>\n");
            }
            NodeType::InlineMath => {
                self.output.push_str("<span class=\"math math-inline\">");
                if let Some(value) = &node.value {
                    self.output.push_str(&escape_html(value));
                }
                self.output.push_str("</span>");
            }
            // YAML frontmatter (typically not rendered in HTML)
            NodeType::Yaml | NodeType::FrontMatter => {
                // Skip rendering frontmatter in HTML output
            }
            _ => {
                // Fallback for unhandled types
                for child in &node.children {
                    self.visit(child);
                }
            }
        }
    }

    /// Check if a list is tight (no blank lines between items)
    fn is_tight_list(&self, _list: &Node) -> bool {
        // Simplified check - would need to analyze spacing
        true
    }

    /// Get generated HTML
    pub fn finish(self) -> String {
        self.output
    }
}

/// Escape HTML special characters
pub fn escape_html(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

/// Escape attribute values
pub fn escape_attr(text: &str) -> String {
    escape_html(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("Hello <world>"), "Hello &lt;world&gt;");
        assert_eq!(escape_html("A & B"), "A &amp; B");
        assert_eq!(escape_html("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_simple_paragraph() {
        let mut visitor = HtmlVisitor::new();
        let node = Node {
            node_type: NodeType::Paragraph,
            children: vec![Node {
                node_type: NodeType::Text,
                value: Some("Hello world".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };

        visitor.visit(&node);
        assert_eq!(visitor.finish(), "<p>Hello world</p>\n");
    }
}
