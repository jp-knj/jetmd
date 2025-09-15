// GFM autolink extension parser

use fmd_core::{Node, NodeType};

/// Extended autolink detection for GFM
pub struct AutolinkParser;

impl AutolinkParser {
    /// Parse a URL autolink (not in angle brackets)
    pub fn parse_url_autolink(text: &str, pos: usize) -> Option<(Node, usize)> {
        // Check if we're at the start of a URL
        let remaining = &text[pos..];

        // Look for common URL schemes
        let schemes = ["http://", "https://", "ftp://", "ftps://"];
        let mut matched_scheme = None;

        for scheme in &schemes {
            if remaining.starts_with(scheme) {
                matched_scheme = Some(*scheme);
                break;
            }
        }

        let scheme = matched_scheme?;

        // Find the end of the URL
        let mut end_pos = pos + scheme.len();
        let bytes = text.as_bytes();

        while end_pos < bytes.len() {
            let ch = bytes[end_pos];

            // URL ends at whitespace or certain punctuation
            if ch.is_ascii_whitespace() {
                break;
            }

            // Check for trailing punctuation that shouldn't be included
            if matches!(
                ch,
                b'.' | b',' | b';' | b':' | b'!' | b'?' | b')' | b']' | b'}'
            ) {
                // Look ahead to see if this is really the end
                if end_pos + 1 >= bytes.len() || bytes[end_pos + 1].is_ascii_whitespace() {
                    break;
                }
            }

            end_pos += 1;
        }

        // Make sure we have something after the scheme
        if end_pos <= pos + scheme.len() {
            return None;
        }

        let url = text[pos..end_pos].to_string();

        // Create link node
        let node = Node {
            node_type: NodeType::Link,
            url: Some(url.clone()),
            children: vec![Node {
                node_type: NodeType::Text,
                value: Some(url.clone()),
                ..Default::default()
            }],
            ..Default::default()
        };

        Some((node, end_pos))
    }

    /// Parse an email autolink
    pub fn parse_email_autolink(text: &str, pos: usize) -> Option<(Node, usize)> {
        // Simple email pattern: alphanumeric@domain.tld
        let bytes = text.as_bytes();
        let mut end_pos = pos;
        let mut has_at = false;
        let mut has_dot_after_at = false;
        let mut at_pos = 0;

        // Scan for email pattern
        while end_pos < bytes.len() {
            let ch = bytes[end_pos];

            if ch == b'@' {
                if has_at {
                    // Multiple @ signs, not valid
                    return None;
                }
                has_at = true;
                at_pos = end_pos;
            } else if ch == b'.' && has_at {
                has_dot_after_at = true;
            } else if ch.is_ascii_whitespace()
                || ch == b'<'
                || ch == b'>'
                || (!ch.is_ascii_alphanumeric() && !matches!(ch, b'-' | b'_' | b'.' | b'+'))
            {
                break;
            }

            end_pos += 1;
        }

        // Validate email
        if !has_at || !has_dot_after_at || at_pos == pos || at_pos >= end_pos - 1 {
            return None;
        }

        // Make sure there's something before @ and after the last dot
        let email = &text[pos..end_pos];
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return None;
        }

        // Check domain has at least one dot
        if !parts[1].contains('.') {
            return None;
        }

        let email_str = email.to_string();

        // Create link node with mailto
        let node = Node {
            node_type: NodeType::Link,
            url: Some(format!("mailto:{}", email_str)),
            children: vec![Node {
                node_type: NodeType::Text,
                value: Some(email_str),
                ..Default::default()
            }],
            ..Default::default()
        };

        Some((node, end_pos))
    }

    /// Process text for GFM autolinks
    pub fn process_autolinks(text: &str) -> Vec<Node> {
        let mut nodes = Vec::new();
        let mut pos = 0;
        let mut plain_text = String::new();

        while pos < text.len() {
            // Try URL autolink first
            if let Some((node, new_pos)) = Self::parse_url_autolink(text, pos) {
                // Add accumulated plain text
                if !plain_text.is_empty() {
                    nodes.push(Node {
                        node_type: NodeType::Text,
                        value: Some(plain_text.clone()),
                        ..Default::default()
                    });
                    plain_text.clear();
                }

                nodes.push(node);
                pos = new_pos;
                continue;
            }

            // Try email autolink
            if pos == 0 || text.as_bytes()[pos - 1].is_ascii_whitespace() {
                if let Some((node, new_pos)) = Self::parse_email_autolink(text, pos) {
                    // Add accumulated plain text
                    if !plain_text.is_empty() {
                        nodes.push(Node {
                            node_type: NodeType::Text,
                            value: Some(plain_text.clone()),
                            ..Default::default()
                        });
                        plain_text.clear();
                    }

                    nodes.push(node);
                    pos = new_pos;
                    continue;
                }
            }

            // Regular character
            if let Some(ch) = text.chars().nth(pos) {
                plain_text.push(ch);
                pos += ch.len_utf8();
            } else {
                pos += 1;
            }
        }

        // Add remaining plain text
        if !plain_text.is_empty() {
            nodes.push(Node {
                node_type: NodeType::Text,
                value: Some(plain_text),
                ..Default::default()
            });
        }

        nodes
    }
}

/// Check if text contains a potential URL
pub fn contains_url(text: &str) -> bool {
    text.contains("http://")
        || text.contains("https://")
        || text.contains("ftp://")
        || text.contains("ftps://")
}

/// Check if text contains potential email
pub fn contains_email(text: &str) -> bool {
    // Simple check for @ with dots after it
    if let Some(at_pos) = text.find('@') {
        let after_at = &text[at_pos + 1..];
        after_at.contains('.') && !after_at.starts_with('.') && !after_at.ends_with('.')
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_autolink() {
        let text = "Visit https://example.com for more";
        let result = AutolinkParser::parse_url_autolink(text, 6);

        assert!(result.is_some());
        let (node, end_pos) = result.unwrap();
        assert_eq!(node.node_type, NodeType::Link);
        assert_eq!(node.url, Some("https://example.com".to_string()));
        assert_eq!(end_pos, 25);
    }

    #[test]
    fn test_parse_url_with_path() {
        let text = "See http://example.com/path/to/page.html";
        let result = AutolinkParser::parse_url_autolink(text, 4);

        assert!(result.is_some());
        let (node, _) = result.unwrap();
        assert_eq!(
            node.url,
            Some("http://example.com/path/to/page.html".to_string())
        );
    }

    #[test]
    fn test_parse_email_autolink() {
        let text = "Contact user@example.com for help";
        let result = AutolinkParser::parse_email_autolink(text, 8);

        assert!(result.is_some());
        let (node, end_pos) = result.unwrap();
        assert_eq!(node.node_type, NodeType::Link);
        assert_eq!(node.url, Some("mailto:user@example.com".to_string()));
        assert_eq!(end_pos, 24);
    }

    #[test]
    fn test_invalid_email() {
        // No domain
        assert!(AutolinkParser::parse_email_autolink("user@", 0).is_none());

        // No user
        assert!(AutolinkParser::parse_email_autolink("@example.com", 0).is_none());

        // No dot in domain
        assert!(AutolinkParser::parse_email_autolink("user@example", 0).is_none());
    }

    #[test]
    fn test_process_mixed_autolinks() {
        let text = "Visit https://example.com or email user@example.com";
        let nodes = AutolinkParser::process_autolinks(text);

        // Should have: text, link, text, link
        assert_eq!(nodes.len(), 4);
        assert_eq!(nodes[0].node_type, NodeType::Text);
        assert_eq!(nodes[1].node_type, NodeType::Link);
        assert_eq!(nodes[2].node_type, NodeType::Text);
        assert_eq!(nodes[3].node_type, NodeType::Link);
    }

    #[test]
    fn test_url_trailing_punctuation() {
        let text = "See https://example.com.";
        let result = AutolinkParser::parse_url_autolink(text, 4);

        assert!(result.is_some());
        let (node, end_pos) = result.unwrap();
        // Should not include trailing period
        assert_eq!(node.url, Some("https://example.com".to_string()));
        assert_eq!(end_pos, 23);
    }
}
