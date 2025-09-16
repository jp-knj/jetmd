// Inline parser for faster-md

use crate::ast::{Node, NodeType};
use crate::position::{Point, Position};
use std::collections::HashMap;

pub struct InlineParser {
    input: String,
    position: usize,
    line: usize,
    column: usize,
    track_position: bool,
    #[allow(dead_code)]
    link_references: HashMap<String, LinkReference>,
}

#[derive(Debug, Clone)]
pub struct LinkReference {
    pub url: String,
    pub title: Option<String>,
}

impl InlineParser {
    pub fn new(input: String, track_position: bool) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
            track_position,
            link_references: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Node> {
        self.parse_inline_content()
    }

    pub fn parse_inline_content(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut text_buffer = String::new();

        while !self.is_at_end() {
            let start_pos = self.current_position();
            let c = self.current_char();

            match c {
                '\\' => {
                    // Backslash escape
                    self.advance();
                    if !self.is_at_end() {
                        text_buffer.push(self.advance());
                    }
                }
                '`' => {
                    // Code span
                    if !text_buffer.is_empty() {
                        nodes.push(self.create_text_node(text_buffer.clone()));
                        text_buffer.clear();
                    }
                    if let Some(node) = self.parse_code_span(start_pos) {
                        nodes.push(node);
                    }
                }
                '*' | '_' => {
                    // Emphasis or strong
                    if !text_buffer.is_empty() {
                        nodes.push(self.create_text_node(text_buffer.clone()));
                        text_buffer.clear();
                    }
                    if let Some(node) = self.parse_emphasis(start_pos) {
                        nodes.push(node);
                    } else {
                        text_buffer.push(c);
                        self.advance();
                    }
                }
                '[' => {
                    // Link or image reference
                    if !text_buffer.is_empty() {
                        nodes.push(self.create_text_node(text_buffer.clone()));
                        text_buffer.clear();
                    }
                    if let Some(node) = self.parse_link(start_pos) {
                        nodes.push(node);
                    } else {
                        text_buffer.push(c);
                        self.advance();
                    }
                }
                '!' if self.peek() == Some('[') => {
                    // Image
                    if !text_buffer.is_empty() {
                        nodes.push(self.create_text_node(text_buffer.clone()));
                        text_buffer.clear();
                    }
                    if let Some(node) = self.parse_image(start_pos) {
                        nodes.push(node);
                    } else {
                        text_buffer.push(c);
                        self.advance();
                    }
                }
                '<' => {
                    // Autolink or HTML
                    let saved_pos = self.save_position();
                    self.advance();

                    if let Some(node) = self.try_parse_autolink(start_pos) {
                        if !text_buffer.is_empty() {
                            nodes.push(self.create_text_node(text_buffer.clone()));
                            text_buffer.clear();
                        }
                        nodes.push(node);
                    } else {
                        self.restore_position(saved_pos);
                        text_buffer.push(c);
                        self.advance();
                    }
                }
                '\n' => {
                    // Line break - check for hard break (two spaces before newline)
                    if text_buffer.ends_with("  ") {
                        text_buffer.truncate(text_buffer.len() - 2);
                        if !text_buffer.is_empty() {
                            nodes.push(self.create_text_node(text_buffer.clone()));
                            text_buffer.clear();
                        }
                        nodes.push(Node {
                            node_type: NodeType::Break,
                            children: vec![],
                            value: None,
                            position: self.make_position(start_pos),
                            ..Default::default()
                        });
                    } else {
                        text_buffer.push(' '); // Convert newline to space
                    }
                    self.advance();
                }
                _ => {
                    text_buffer.push(c);
                    self.advance();
                }
            }
        }

        // Flush remaining text
        if !text_buffer.is_empty() {
            nodes.push(self.create_text_node(text_buffer));
        }

        nodes
    }

    fn parse_code_span(&mut self, start: Option<Position>) -> Option<Node> {
        let mut backtick_count = 0;
        while self.current_char() == '`' {
            backtick_count += 1;
            self.advance();
        }

        // Find closing backticks
        let mut code = String::new();
        let mut found_closing = false;

        while !self.is_at_end() {
            if self.current_char() == '`' {
                let mut count = 0;
                let saved = self.save_position();
                while self.current_char() == '`' && count < backtick_count {
                    count += 1;
                    self.advance();
                }

                if count == backtick_count {
                    found_closing = true;
                    break;
                } else {
                    self.restore_position(saved);
                    code.push('`');
                    self.advance();
                }
            } else {
                code.push(self.advance());
            }
        }

        if !found_closing {
            return None;
        }

        // Normalize spaces in code
        let normalized = if code.starts_with(' ') && code.ends_with(' ') && code.len() > 2 {
            code[1..code.len() - 1].to_string()
        } else {
            code
        };

        Some(Node {
            node_type: NodeType::InlineCode,
            children: vec![],
            value: Some(normalized),
            position: self.make_position(start),
            ..Default::default()
        })
    }

    fn parse_emphasis(&mut self, start: Option<Position>) -> Option<Node> {
        let marker = self.current_char();
        let mut marker_count = 0;

        while self.current_char() == marker {
            marker_count += 1;
            self.advance();
        }

        // Try to find closing markers
        let content_start = self.position;
        let mut found_closing = false;
        let mut content_end = self.position;

        while !self.is_at_end() {
            if self.current_char() == marker {
                let saved = self.save_position();
                let mut count = 0;
                while self.current_char() == marker {
                    count += 1;
                    self.advance();
                }

                if count >= marker_count {
                    content_end = saved.0;
                    found_closing = true;
                    break;
                } else {
                    self.restore_position(saved);
                    self.advance();
                }
            } else {
                self.advance();
            }
        }

        if !found_closing {
            return None;
        }

        // Parse inner content
        let inner_content = self.input[content_start..content_end].to_string();
        let mut inner_parser = InlineParser::new(inner_content, self.track_position);
        let children = inner_parser.parse();

        let node_type = if marker_count >= 2 {
            NodeType::Strong
        } else {
            NodeType::Emphasis
        };

        Some(Node {
            node_type,
            children,
            value: None,
            position: self.make_position(start),
            ..Default::default()
        })
    }

    fn parse_link(&mut self, start: Option<Position>) -> Option<Node> {
        self.advance(); // Skip '['

        // Parse link text
        let text_start = self.position;
        let mut bracket_depth = 1;
        let mut text_end = self.position;

        while !self.is_at_end() && bracket_depth > 0 {
            match self.current_char() {
                '[' => bracket_depth += 1,
                ']' => {
                    bracket_depth -= 1;
                    if bracket_depth == 0 {
                        text_end = self.position;
                    }
                }
                '\\' => {
                    self.advance();
                    if !self.is_at_end() {
                        self.advance();
                    }
                    continue;
                }
                _ => {}
            }
            self.advance();
        }

        if bracket_depth != 0 {
            return None;
        }

        let link_text = self.input[text_start..text_end].to_string();

        // Check what follows the closing bracket
        if self.current_char() == '(' {
            // Inline link
            self.advance();
            self.skip_whitespace();

            // Parse URL
            let url_start = self.position;
            let mut paren_depth = 1;
            let mut url_end = self.position;
            let mut title_start = None;
            let mut title_end = None;

            while !self.is_at_end() && paren_depth > 0 {
                match self.current_char() {
                    '(' => paren_depth += 1,
                    ')' => {
                        paren_depth -= 1;
                        if paren_depth == 0 {
                            url_end = self.position;
                        }
                    }
                    ' ' | '\t' | '\n' if title_start.is_none() => {
                        url_end = self.position;
                        self.skip_whitespace();

                        // Check for title
                        let quote = self.current_char();
                        if quote == '"' || quote == '\'' || quote == '(' {
                            let closing = if quote == '(' { ')' } else { quote };
                            self.advance();
                            title_start = Some(self.position);

                            while !self.is_at_end() && self.current_char() != closing {
                                if self.current_char() == '\\' {
                                    self.advance();
                                }
                                self.advance();
                            }

                            if self.current_char() == closing {
                                title_end = Some(self.position);
                                self.advance();
                            }
                        }
                    }
                    '\\' => {
                        self.advance();
                        if !self.is_at_end() {
                            self.advance();
                        }
                        continue;
                    }
                    _ => {}
                }

                if title_start.is_none() {
                    self.advance();
                }
            }

            if paren_depth != 0 {
                return None;
            }

            let url = self.input[url_start..url_end].to_string();
            let title = if let (Some(start), Some(end)) = (title_start, title_end) {
                Some(self.input[start..end].to_string())
            } else {
                None
            };

            // Parse link text as inline content
            let mut text_parser = InlineParser::new(link_text, self.track_position);
            let children = text_parser.parse();

            Some(Node {
                node_type: NodeType::Link,
                children,
                url: Some(url),
                title,
                position: self.make_position(start),
                ..Default::default()
            })
        } else if self.current_char() == '[' {
            // Reference link
            self.advance();

            let ref_start = self.position;
            let mut ref_end = self.position;

            while !self.is_at_end() && self.current_char() != ']' {
                ref_end = self.position;
                self.advance();
            }

            if self.current_char() != ']' {
                return None;
            }

            let reference = if ref_start == ref_end {
                // Collapsed reference
                link_text.clone()
            } else {
                self.input[ref_start..ref_end].to_string()
            };

            self.advance(); // Skip closing ']'

            // Parse link text as inline content
            let mut text_parser = InlineParser::new(link_text, self.track_position);
            let children = text_parser.parse();

            Some(Node {
                node_type: NodeType::LinkReference,
                children,
                identifier: Some(reference.to_lowercase()),
                position: self.make_position(start),
                ..Default::default()
            })
        } else {
            None
        }
    }

    fn parse_image(&mut self, start: Option<Position>) -> Option<Node> {
        self.advance(); // Skip '!'

        if self.current_char() != '[' {
            return None;
        }

        // Parse similarly to link but create Image node
        let saved = self.save_position();
        if let Some(mut node) = self.parse_link(start) {
            match node.node_type {
                NodeType::Link => {
                    node.node_type = NodeType::Image;
                    // Move children text to alt
                    if !node.children.is_empty() {
                        let alt_text = Self::extract_text_from_nodes(&node.children);
                        node.alt = Some(alt_text);
                        node.children.clear();
                    }
                    Some(node)
                }
                NodeType::LinkReference => {
                    node.node_type = NodeType::ImageReference;
                    // Move children text to alt
                    if !node.children.is_empty() {
                        let alt_text = Self::extract_text_from_nodes(&node.children);
                        node.alt = Some(alt_text);
                        node.children.clear();
                    }
                    Some(node)
                }
                _ => None,
            }
        } else {
            self.restore_position(saved);
            None
        }
    }

    fn try_parse_autolink(&mut self, start: Option<Position>) -> Option<Node> {
        // Already consumed '<'
        let content_start = self.position;

        // Check for URL autolink
        if self.check_url_scheme() {
            while !self.is_at_end() && self.current_char() != '>' && !self.is_whitespace() {
                self.advance();
            }

            if self.current_char() == '>' {
                let url = self.input[content_start..self.position].to_string();
                self.advance(); // Skip '>'

                return Some(Node {
                    node_type: NodeType::Link,
                    children: vec![self.create_text_node(url.clone())],
                    url: Some(url),
                    title: None,
                    position: self.make_position(start),
                    ..Default::default()
                });
            }
        }

        // Check for email autolink
        let saved = self.save_position();
        self.position = content_start;

        if self.check_email() {
            while !self.is_at_end() && self.current_char() != '>' && !self.is_whitespace() {
                self.advance();
            }

            if self.current_char() == '>' {
                let email = self.input[content_start..self.position].to_string();
                self.advance(); // Skip '>'

                return Some(Node {
                    node_type: NodeType::Link,
                    children: vec![self.create_text_node(email.clone())],
                    url: Some(format!("mailto:{}", email)),
                    title: None,
                    position: self.make_position(start),
                    ..Default::default()
                });
            }
        }

        self.restore_position(saved);
        None
    }

    fn check_url_scheme(&self) -> bool {
        let schemes = ["http://", "https://", "ftp://"];
        for scheme in schemes {
            if self.input[self.position..].starts_with(scheme) {
                return true;
            }
        }
        false
    }

    fn check_email(&self) -> bool {
        // Simple email validation
        let remaining = &self.input[self.position..];
        let at_pos = remaining.find('@');
        let gt_pos = remaining.find('>');

        if let (Some(at), Some(gt)) = (at_pos, gt_pos) {
            at > 0 && at < gt && remaining[at + 1..gt].contains('.')
        } else {
            false
        }
    }

    fn extract_text_from_nodes(nodes: &[Node]) -> String {
        let mut text = String::new();
        for node in nodes {
            match node.node_type {
                NodeType::Text => {
                    if let Some(value) = &node.value {
                        text.push_str(value);
                    }
                }
                _ => {
                    text.push_str(&Self::extract_text_from_nodes(&node.children));
                }
            }
        }
        text
    }

    fn create_text_node(&self, value: String) -> Node {
        Node {
            node_type: NodeType::Text,
            children: vec![],
            value: Some(value),
            position: None,
            ..Default::default()
        }
    }

    // Helper methods
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input.chars().nth(self.position).unwrap_or('\0')
        }
    }

    fn peek(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            self.input.chars().nth(self.position + 1)
        } else {
            None
        }
    }

    fn advance(&mut self) -> char {
        let c = self.current_char();
        if !self.is_at_end() {
            self.position += c.len_utf8();
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while self.is_whitespace() {
            self.advance();
        }
    }

    fn is_whitespace(&self) -> bool {
        matches!(self.current_char(), ' ' | '\t' | '\n' | '\r')
    }

    fn save_position(&self) -> (usize, usize, usize) {
        (self.position, self.line, self.column)
    }

    fn restore_position(&mut self, saved: (usize, usize, usize)) {
        self.position = saved.0;
        self.line = saved.1;
        self.column = saved.2;
    }

    fn current_position(&self) -> Option<Position> {
        if self.track_position {
            Some(Position {
                start: Point {
                    line: self.line,
                    column: self.column,
                    offset: self.position,
                },
                end: Point {
                    line: self.line,
                    column: self.column,
                    offset: self.position,
                },
                source: None,
            })
        } else {
            None
        }
    }

    fn make_position(&self, start: Option<Position>) -> Option<Position> {
        if let Some(s) = start {
            let end = self.current_position()?;
            Some(Position {
                start: s.start,
                end: end.end,
                source: None,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text() {
        let mut parser = InlineParser::new("Hello world".to_string(), false);
        let nodes = parser.parse();

        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].node_type, NodeType::Text);
        assert_eq!(nodes[0].value, Some("Hello world".to_string()));
    }

    #[test]
    fn test_parse_emphasis() {
        let mut parser = InlineParser::new("*emphasis* and **strong**".to_string(), false);
        let nodes = parser.parse();

        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].node_type, NodeType::Emphasis);
        assert_eq!(nodes[2].node_type, NodeType::Strong);
    }

    #[test]
    fn test_parse_code_span() {
        let mut parser =
            InlineParser::new("`code` and ``code with ` backtick``".to_string(), false);
        let nodes = parser.parse();

        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].node_type, NodeType::InlineCode);
        assert_eq!(nodes[0].value, Some("code".to_string()));
        assert_eq!(nodes[2].node_type, NodeType::InlineCode);
        assert_eq!(nodes[2].value, Some("code with ` backtick".to_string()));
    }
}
