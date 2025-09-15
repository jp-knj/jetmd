// GFM strikethrough parser

use fmd_core::{Node, NodeType};

/// Parse strikethrough text (~~text~~)
pub fn parse_strikethrough(input: &str, pos: usize) -> Option<(Node, usize)> {
    let bytes = input.as_bytes();

    // Check for opening ~~
    if pos + 1 >= bytes.len() || bytes[pos] != b'~' || bytes[pos + 1] != b'~' {
        return None;
    }

    // Find closing ~~
    let mut end_pos = pos + 2;
    while end_pos + 1 < bytes.len() {
        if bytes[end_pos] == b'~' && bytes[end_pos + 1] == b'~' {
            // Found closing tildes
            let content = &input[pos + 2..end_pos];

            // Don't match empty strikethrough
            if content.is_empty() {
                return None;
            }

            // Parse inner content
            use fmd_core::inline::InlineParser;
            let mut parser = InlineParser::new(content.to_string(), false);
            let children = parser.parse();

            let node = Node {
                node_type: NodeType::Delete,
                children,
                ..Default::default()
            };

            return Some((node, end_pos + 2));
        }
        end_pos += 1;
    }

    None
}

/// Check if position has potential strikethrough
pub fn has_strikethrough_at(input: &str, pos: usize) -> bool {
    let bytes = input.as_bytes();
    pos + 1 < bytes.len() && bytes[pos] == b'~' && bytes[pos + 1] == b'~'
}

/// Process text with strikethrough support
pub fn process_strikethrough(text: &str) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut pos = 0;
    let mut plain_text = String::new();

    while pos < text.len() {
        if has_strikethrough_at(text, pos) {
            // Try to parse strikethrough
            if let Some((node, new_pos)) = parse_strikethrough(text, pos) {
                // Add any accumulated plain text
                if !plain_text.is_empty() {
                    nodes.push(Node {
                        node_type: NodeType::Text,
                        value: Some(plain_text.clone()),
                        ..Default::default()
                    });
                    plain_text.clear();
                }

                // Add strikethrough node
                nodes.push(node);
                pos = new_pos;
            } else {
                // Not valid strikethrough, treat as plain text
                plain_text.push('~');
                pos += 1;
            }
        } else {
            // Regular character
            if let Some(ch) = text.chars().nth(pos) {
                plain_text.push(ch);
                pos += ch.len_utf8();
            } else {
                pos += 1;
            }
        }
    }

    // Add any remaining plain text
    if !plain_text.is_empty() {
        nodes.push(Node {
            node_type: NodeType::Text,
            value: Some(plain_text),
            ..Default::default()
        });
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_strikethrough() {
        let input = "~~deleted text~~";
        let result = parse_strikethrough(input, 0);

        assert!(result.is_some());
        let (node, end_pos) = result.unwrap();
        assert_eq!(node.node_type, NodeType::Delete);
        assert_eq!(end_pos, input.len());

        // Check children
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].node_type, NodeType::Text);
        assert_eq!(node.children[0].value, Some("deleted text".to_string()));
    }

    #[test]
    fn test_parse_nested_strikethrough() {
        let input = "~~deleted **bold** text~~";
        let result = parse_strikethrough(input, 0);

        assert!(result.is_some());
        let (node, _) = result.unwrap();
        assert_eq!(node.node_type, NodeType::Delete);

        // Should have parsed inner emphasis
        assert!(node
            .children
            .iter()
            .any(|n| n.node_type == NodeType::Strong));
    }

    #[test]
    fn test_invalid_strikethrough() {
        // Single tilde
        assert!(parse_strikethrough("~text~", 0).is_none());

        // Unclosed
        assert!(parse_strikethrough("~~text", 0).is_none());

        // Empty
        assert!(parse_strikethrough("~~~~", 0).is_none());
    }

    #[test]
    fn test_process_mixed_text() {
        let text = "Normal ~~deleted~~ text";
        let nodes = process_strikethrough(text);

        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes[0].node_type, NodeType::Text);
        assert_eq!(nodes[0].value, Some("Normal ".to_string()));
        assert_eq!(nodes[1].node_type, NodeType::Delete);
        assert_eq!(nodes[2].node_type, NodeType::Text);
        assert_eq!(nodes[2].value, Some(" text".to_string()));
    }
}
