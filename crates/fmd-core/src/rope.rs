// Rope data structure for efficient incremental parsing

use crate::ast::Node;
use crate::{Edit, Range};
use std::collections::BTreeMap;

/// Rope structure for incremental text editing
#[derive(Debug, Clone)]
pub struct Rope {
    /// Total length of the rope
    total_length: usize,
    /// Root node of the rope tree
    root: RopeNode,
    /// Line break positions for fast line lookup
    line_breaks: Vec<usize>,
}

#[derive(Debug, Clone)]
enum RopeNode {
    Leaf {
        text: String,
        start_offset: usize,
    },
    Branch {
        left: Box<RopeNode>,
        right: Box<RopeNode>,
        weight: usize, // Total length of left subtree
    },
}

impl Rope {
    /// Create a new rope from text
    pub fn new(text: &str) -> Self {
        let line_breaks = text
            .char_indices()
            .filter_map(|(i, c)| if c == '\n' { Some(i) } else { None })
            .collect();

        Self {
            total_length: text.len(),
            root: RopeNode::from_str(text, 0),
            line_breaks,
        }
    }

    /// Get total length
    pub fn len(&self) -> usize {
        self.total_length
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.total_length == 0
    }

    /// Apply an edit to the rope
    pub fn apply_edit(&mut self, edit: &Edit) {
        let new_text = self.replace_range(edit.range.start, edit.range.end, &edit.text);
        *self = Self::new(&new_text);
    }

    /// Replace a range with new text
    fn replace_range(&self, start: usize, end: usize, replacement: &str) -> String {
        let mut result =
            String::with_capacity(self.total_length - (end - start) + replacement.len());
        self.collect_text(&mut result, 0, start);
        result.push_str(replacement);
        self.collect_text(&mut result, end, self.total_length);
        result
    }

    /// Collect text in range into string
    fn collect_text(&self, output: &mut String, start: usize, end: usize) {
        self.root.collect_range(output, start, end);
    }

    /// Get text at position
    pub fn char_at(&self, offset: usize) -> Option<char> {
        self.root.char_at(offset)
    }

    /// Get line number from offset
    pub fn line_at_offset(&self, offset: usize) -> usize {
        match self.line_breaks.binary_search(&offset) {
            Ok(idx) => idx + 1,
            Err(idx) => idx + 1,
        }
    }

    /// Get line start offset
    pub fn line_start(&self, line: usize) -> usize {
        if line == 1 {
            0
        } else if line > 1 && line <= self.line_breaks.len() + 1 {
            self.line_breaks[line - 2] + 1
        } else {
            self.total_length
        }
    }
}

impl std::fmt::Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::with_capacity(self.total_length);
        self.collect_text(&mut result, 0, self.total_length);
        write!(f, "{}", result)
    }
}

impl RopeNode {
    /// Create from string
    fn from_str(text: &str, start_offset: usize) -> Self {
        const SPLIT_THRESHOLD: usize = 1024;

        if text.len() <= SPLIT_THRESHOLD {
            RopeNode::Leaf {
                text: text.to_string(),
                start_offset,
            }
        } else {
            let mid = text.len() / 2;
            // Find a good split point (preferably at line boundary)
            let split = text[..mid].rfind('\n').map(|i| i + 1).unwrap_or(mid);

            let left = Box::new(RopeNode::from_str(&text[..split], start_offset));
            let right = Box::new(RopeNode::from_str(&text[split..], start_offset + split));

            RopeNode::Branch {
                weight: split,
                left,
                right,
            }
        }
    }

    /// Collect text in range
    fn collect_range(&self, output: &mut String, start: usize, end: usize) {
        match self {
            RopeNode::Leaf { text, start_offset } => {
                let node_end = start_offset + text.len();
                if start < node_end && end > *start_offset {
                    let text_start = start.saturating_sub(*start_offset);
                    let text_end = (end - start_offset).min(text.len());
                    output.push_str(&text[text_start..text_end]);
                }
            }
            RopeNode::Branch {
                left,
                right,
                weight,
            } => {
                if start < *weight {
                    left.collect_range(output, start, end.min(*weight));
                }
                if end > *weight {
                    right.collect_range(output, start.saturating_sub(*weight), end - weight);
                }
            }
        }
    }

    /// Get character at offset
    fn char_at(&self, offset: usize) -> Option<char> {
        match self {
            RopeNode::Leaf { text, start_offset } => {
                let local_offset = offset.checked_sub(*start_offset)?;
                text.chars().nth(local_offset)
            }
            RopeNode::Branch {
                left,
                right,
                weight,
            } => {
                if offset < *weight {
                    left.char_at(offset)
                } else {
                    right.char_at(offset - weight)
                }
            }
        }
    }
}

/// Tree structure for incremental parsing
#[derive(Debug)]
pub struct IncrementalTree {
    /// Current AST
    pub tree: Node,
    /// Text rope
    rope: Rope,
    /// Cached node positions
    node_positions: BTreeMap<usize, NodeInfo>,
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// Node identifier (path in tree)
    #[allow(dead_code)]
    path: Vec<usize>,
    /// Start offset
    start: usize,
    /// End offset
    end: usize,
    /// Hash of node content for change detection
    hash: u64,
}

impl IncrementalTree {
    /// Create new incremental tree
    pub fn new(text: &str, tree: Node) -> Self {
        let rope = Rope::new(text);
        let mut node_positions = BTreeMap::new();
        Self::index_nodes(&tree, &mut node_positions, &[], 0);

        Self {
            tree,
            rope,
            node_positions,
        }
    }

    /// Index nodes by position
    fn index_nodes(
        node: &Node,
        map: &mut BTreeMap<usize, NodeInfo>,
        path: &[usize],
        _depth: usize,
    ) {
        if let Some(pos) = &node.position {
            let info = NodeInfo {
                path: path.to_vec(),
                start: pos.start.offset,
                end: pos.end.offset,
                hash: Self::hash_node(node),
            };
            map.insert(pos.start.offset, info);
        }

        for (i, child) in node.children.iter().enumerate() {
            let mut child_path = path.to_vec();
            child_path.push(i);
            Self::index_nodes(child, map, &child_path, _depth + 1);
        }
    }

    /// Simple hash function for node content
    fn hash_node(node: &Node) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        format!("{:?}", node.node_type).hash(&mut hasher);
        if let Some(value) = &node.value {
            value.hash(&mut hasher);
        }
        hasher.finish()
    }

    /// Apply edit and return affected range
    pub fn apply_edit(&mut self, edit: &Edit) -> Range {
        self.rope.apply_edit(edit);

        // Find affected nodes
        let affected_start = edit.range.start;
        let affected_end = edit.range.end;

        // Clear cached positions in affected range
        self.node_positions
            .retain(|&offset, _| offset < affected_start || offset >= affected_end);

        Range {
            start: affected_start,
            end: affected_start + edit.text.len(),
        }
    }

    /// Get nodes that can be reused
    pub fn get_reusable_nodes(&self, range: &Range) -> Vec<NodeInfo> {
        self.node_positions
            .values()
            .filter(|info| info.end <= range.start || info.start >= range.end)
            .cloned()
            .collect()
    }

    /// Calculate reuse percentage
    pub fn calculate_reuse_stats(&self, new_tree: &Node) -> (usize, usize) {
        let mut new_positions = BTreeMap::new();
        Self::index_nodes(new_tree, &mut new_positions, &[], 0);

        let mut reused = 0;
        let total = new_positions.len();

        for (_, new_info) in new_positions.iter() {
            if let Some(old_info) = self.node_positions.get(&new_info.start) {
                if old_info.hash == new_info.hash {
                    reused += 1;
                }
            }
        }

        (reused, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope_basic() {
        let rope = Rope::new("Hello\nWorld");
        assert_eq!(rope.len(), 11);
        assert_eq!(rope.char_at(0), Some('H'));
        assert_eq!(rope.char_at(6), Some('W'));
        assert_eq!(rope.to_string(), "Hello\nWorld");
    }

    #[test]
    fn test_rope_edit() {
        let mut rope = Rope::new("Hello World");
        rope.apply_edit(&Edit {
            range: Range { start: 6, end: 11 },
            text: "Rust".to_string(),
        });
        assert_eq!(rope.to_string(), "Hello Rust");
    }

    #[test]
    fn test_line_lookup() {
        let rope = Rope::new("Line 1\nLine 2\nLine 3");
        assert_eq!(rope.line_at_offset(0), 1);
        assert_eq!(rope.line_at_offset(7), 2);
        assert_eq!(rope.line_at_offset(14), 3);

        assert_eq!(rope.line_start(1), 0);
        assert_eq!(rope.line_start(2), 7);
        assert_eq!(rope.line_start(3), 14);
    }
}
