// GFM table parser

use fmd_core::{Node, NodeType, Position};

pub struct TableParser {
    track_position: bool,
}

impl TableParser {
    pub fn new(track_position: bool) -> Self {
        Self { track_position }
    }

    /// Try to parse a table from lines
    pub fn parse_table(&self, lines: &[String], start_line: usize) -> Option<Node> {
        if lines.len() < 2 {
            return None;
        }

        // Parse header row
        let header_cells = self.parse_row(&lines[0])?;

        // Parse delimiter row (must be second line)
        let alignments = self.parse_delimiter_row(&lines[1])?;

        if header_cells.len() != alignments.len() {
            return None;
        }

        // Create header row node
        let header_row = Node {
            node_type: NodeType::TableRow,
            children: header_cells
                .into_iter()
                .enumerate()
                .map(|(i, content)| self.create_cell(content, true, alignments.get(i).copied()))
                .collect(),
            ..Default::default()
        };

        let mut rows = vec![header_row];

        // Parse body rows
        for line in &lines[2..] {
            if line.trim().is_empty() {
                break; // Empty line ends table
            }

            if let Some(cells) = self.parse_row(line) {
                let row = Node {
                    node_type: NodeType::TableRow,
                    children: cells
                        .into_iter()
                        .enumerate()
                        .map(|(i, content)| {
                            self.create_cell(content, false, alignments.get(i).copied())
                        })
                        .collect(),
                    ..Default::default()
                };
                rows.push(row);
            } else {
                break; // Invalid row ends table
            }
        }

        Some(Node {
            node_type: NodeType::Table,
            children: rows,
            ..Default::default()
        })
    }

    /// Parse a table row into cells
    fn parse_row(&self, line: &str) -> Option<Vec<String>> {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') && !trimmed.ends_with('|') && !trimmed.contains('|') {
            return None;
        }

        // Split by pipe, handling escaped pipes
        let mut cells = Vec::new();
        let mut current_cell = String::new();
        let mut escaped = false;

        for ch in trimmed.chars() {
            if escaped {
                current_cell.push(ch);
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
                current_cell.push(ch);
            } else if ch == '|' {
                cells.push(current_cell.trim().to_string());
                current_cell.clear();
            } else {
                current_cell.push(ch);
            }
        }

        // Add last cell if not empty
        if !current_cell.trim().is_empty() || !cells.is_empty() {
            cells.push(current_cell.trim().to_string());
        }

        // Remove empty cells from start/end if they're from leading/trailing pipes
        if cells.first() == Some(&String::new()) {
            cells.remove(0);
        }
        if cells.last() == Some(&String::new()) {
            cells.pop();
        }

        if cells.is_empty() {
            None
        } else {
            Some(cells)
        }
    }

    /// Parse delimiter row to get column alignments
    fn parse_delimiter_row(&self, line: &str) -> Option<Vec<Alignment>> {
        let cells = self.parse_row(line)?;

        let mut alignments = Vec::new();
        for cell in cells {
            let trimmed = cell.trim();

            // Check if it's a valid delimiter (contains dashes, optionally with colons)
            if !trimmed.chars().any(|c| c == '-') {
                return None;
            }

            // Check for alignment markers
            let left_colon = trimmed.starts_with(':');
            let right_colon = trimmed.ends_with(':');

            // Verify the middle is all dashes
            let middle = if left_colon && right_colon {
                &trimmed[1..trimmed.len() - 1]
            } else if left_colon {
                &trimmed[1..]
            } else if right_colon {
                &trimmed[..trimmed.len() - 1]
            } else {
                trimmed
            };

            if !middle.chars().all(|c| c == '-' || c == ' ') {
                return None;
            }

            let alignment = match (left_colon, right_colon) {
                (true, true) => Alignment::Center,
                (true, false) => Alignment::Left,
                (false, true) => Alignment::Right,
                (false, false) => Alignment::None,
            };

            alignments.push(alignment);
        }

        Some(alignments)
    }

    /// Create a table cell node
    fn create_cell(&self, content: String, is_header: bool, alignment: Option<Alignment>) -> Node {
        use fmd_core::inline::InlineParser;

        let mut parser = InlineParser::new(content, self.track_position);
        let children = parser.parse();

        let mut node = Node {
            node_type: NodeType::TableCell,
            children,
            ..Default::default()
        };

        // Store header and alignment info in data field
        if is_header {
            node.data
                .insert("header".to_string(), serde_json::Value::Bool(true));
        }

        if let Some(align) = alignment {
            let align_str = match align {
                Alignment::Left => "left",
                Alignment::Right => "right",
                Alignment::Center => "center",
                Alignment::None => "none",
            };
            node.data.insert(
                "align".to_string(),
                serde_json::Value::String(align_str.to_string()),
            );
        }

        node
    }
}

#[derive(Debug, Clone, Copy)]
enum Alignment {
    None,
    Left,
    Right,
    Center,
}

/// Check if lines form a valid GFM table
pub fn is_table(lines: &[&str]) -> bool {
    if lines.len() < 2 {
        return false;
    }

    // First line must have pipes
    if !lines[0].contains('|') {
        return false;
    }

    // Second line must be delimiter row
    let delimiter = lines[1].trim();
    if !delimiter.contains('|') || !delimiter.contains('-') {
        return false;
    }

    // Check delimiter row format
    let parts: Vec<&str> = delimiter.split('|').collect();
    for part in parts {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Must be dashes with optional colons
        let without_colons = trimmed.trim_start_matches(':').trim_end_matches(':');
        if !without_colons.chars().all(|c| c == '-' || c == ' ') {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_table() {
        let parser = TableParser::new(false);
        let lines = vec![
            "| Header 1 | Header 2 |".to_string(),
            "| --- | --- |".to_string(),
            "| Cell 1 | Cell 2 |".to_string(),
        ];

        let table = parser.parse_table(&lines, 0);
        assert!(table.is_some());

        let table = table.unwrap();
        assert_eq!(table.node_type, NodeType::Table);
        assert_eq!(table.children.len(), 2); // Header + 1 body row
    }

    #[test]
    fn test_parse_aligned_table() {
        let parser = TableParser::new(false);
        let lines = vec![
            "| Left | Center | Right |".to_string(),
            "| :--- | :---: | ---: |".to_string(),
            "| A | B | C |".to_string(),
        ];

        let table = parser.parse_table(&lines, 0);
        assert!(table.is_some());
    }

    #[test]
    fn test_is_table() {
        assert!(is_table(&["| Header |", "| --- |", "| Cell |"]));

        assert!(is_table(&[
            "Header 1 | Header 2",
            "--- | ---",
            "Cell 1 | Cell 2"
        ]));

        assert!(!is_table(&["Not a table", "Just text"]));
    }
}
