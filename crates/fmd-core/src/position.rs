// Position tracking utilities

pub use crate::ast::{Point, Position};

/// Tracks position in source text
#[derive(Debug, Clone)]
pub struct PositionTracker {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    line_starts: Vec<usize>,
}

impl PositionTracker {
    pub fn new() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
            line_starts: vec![0],
        }
    }
    
    /// Advance position by a character
    pub fn advance(&mut self, c: char) {
        self.offset += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.column = 1;
            self.line_starts.push(self.offset);
        } else if c == '\t' {
            // Tab advances to next multiple of 4
            self.column = ((self.column - 1) / 4 + 1) * 4 + 1;
        } else {
            self.column += 1;
        }
    }
    
    /// Advance by a string slice
    pub fn advance_str(&mut self, s: &str) {
        for c in s.chars() {
            self.advance(c);
        }
    }
    
    /// Get current point
    pub fn current_point(&self) -> Point {
        Point {
            line: self.line,
            column: self.column,
            offset: self.offset,
        }
    }
    
    /// Create a position from start to current
    pub fn make_position(&self, start: Point) -> Position {
        Position {
            start,
            end: self.current_point(),
            source: None,
        }
    }
    
    /// Get position from byte offsets
    pub fn position_from_offsets(&self, start_offset: usize, end_offset: usize) -> Position {
        let start = self.point_from_offset(start_offset);
        let end = self.point_from_offset(end_offset);
        Position {
            start,
            end,
            source: None,
        }
    }
    
    /// Convert byte offset to Point
    pub fn point_from_offset(&self, offset: usize) -> Point {
        // Binary search for line
        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => idx.saturating_sub(1),
        };
        
        let line = line_idx + 1;
        let line_start = self.line_starts[line_idx];
        let column = offset - line_start + 1;
        
        Point {
            line,
            column,
            offset,
        }
    }
    
    /// Reset tracker
    pub fn reset(&mut self) {
        self.line = 1;
        self.column = 1;
        self.offset = 0;
        self.line_starts.clear();
        self.line_starts.push(0);
    }
}

/// Calculate edit distance between positions
pub fn calculate_edit_distance(old_pos: &Position, new_pos: &Position) -> usize {
    let start_diff = old_pos.start.offset.abs_diff(new_pos.start.offset);
    let end_diff = old_pos.end.offset.abs_diff(new_pos.end.offset);
    start_diff + end_diff
}

/// Check if positions overlap
pub fn positions_overlap(a: &Position, b: &Position) -> bool {
    !(a.end.offset <= b.start.offset || b.end.offset <= a.start.offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_position_tracker() {
        let mut tracker = PositionTracker::new();
        
        assert_eq!(tracker.line, 1);
        assert_eq!(tracker.column, 1);
        assert_eq!(tracker.offset, 0);
        
        tracker.advance('H');
        assert_eq!(tracker.column, 2);
        assert_eq!(tracker.offset, 1);
        
        tracker.advance('\n');
        assert_eq!(tracker.line, 2);
        assert_eq!(tracker.column, 1);
        assert_eq!(tracker.offset, 2);
        
        tracker.advance_str("Hello");
        assert_eq!(tracker.column, 6);
        assert_eq!(tracker.offset, 7);
    }
    
    #[test]
    fn test_tab_handling() {
        let mut tracker = PositionTracker::new();
        tracker.advance('\t');
        assert_eq!(tracker.column, 5); // Next multiple of 4 + 1
        
        tracker.advance('x');
        tracker.advance('\t');
        assert_eq!(tracker.column, 9); // Next multiple of 4 + 1
    }
    
    #[test]
    fn test_offset_to_point() {
        let mut tracker = PositionTracker::new();
        tracker.advance_str("Line 1\nLine 2\nLine 3");
        
        let point = tracker.point_from_offset(0);
        assert_eq!(point.line, 1);
        assert_eq!(point.column, 1);
        
        let point = tracker.point_from_offset(7); // Start of line 2
        assert_eq!(point.line, 2);
        assert_eq!(point.column, 1);
        
        let point = tracker.point_from_offset(10); // Middle of line 2
        assert_eq!(point.line, 2);
        assert_eq!(point.column, 4);
    }
}