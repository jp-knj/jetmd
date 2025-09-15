// Block scanner with optimization for faster-md

use crate::ast::{Point, Position};

pub struct Scanner<'a> {
    input: &'a str,
    bytes: &'a [u8],
    position: usize,
    line: usize,
    column: usize,
    track_position: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str, track_position: bool) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            position: 0,
            line: 1,
            column: 1,
            track_position,
        }
    }

    pub fn scan_blocks(&mut self) -> Vec<BlockToken> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            if let Some(token) = self.scan_block() {
                tokens.push(token);
            }
        }

        tokens
    }

    fn scan_block(&mut self) -> Option<BlockToken> {
        self.skip_blank_lines();

        if self.is_at_end() {
            return None;
        }

        let start_pos = self.current_position();

        // Check for ATX heading
        if self.check_heading() {
            return Some(self.scan_heading(start_pos));
        }

        // Check for thematic break
        if self.check_thematic_break() {
            return Some(self.scan_thematic_break(start_pos));
        }

        // Check for code fence
        if self.check_code_fence() {
            return Some(self.scan_code_fence(start_pos));
        }

        // Check for blockquote
        if self.check_char('>') {
            return Some(self.scan_blockquote(start_pos));
        }

        // Check for list
        if self.check_list_marker() {
            return Some(self.scan_list_item(start_pos));
        }

        // Default to paragraph
        Some(self.scan_paragraph(start_pos))
    }

    fn scan_heading(&mut self, start: Option<Position>) -> BlockToken {
        let mut depth = 0;
        while self.check_char('#') && depth < 6 {
            self.advance();
            depth += 1;
        }

        // Require space after hashes for ATX heading
        if !self.check_char(' ') && !self.is_at_line_end() {
            // Not a valid heading, treat as paragraph
            return self.scan_paragraph(start);
        }

        self.skip_spaces();
        let content_start = self.position;
        self.advance_to_line_end();
        let content = self.input[content_start..self.position].trim_end();

        // Remove optional closing hashes
        let content = content.trim_end_matches('#').trim_end();

        let end = self.current_position();
        self.advance_line();

        BlockToken {
            token_type: BlockTokenType::Heading(depth as u8),
            content: content.to_string(),
            position: self.make_position(start, end),
        }
    }

    fn scan_thematic_break(&mut self, start: Option<Position>) -> BlockToken {
        let marker = self.current_char();
        let mut count = 0;

        while !self.is_at_line_end() {
            if self.check_char(marker) {
                count += 1;
                self.advance();
            } else if self.check_char(' ') || self.check_char('\t') {
                self.advance();
            } else {
                // Invalid thematic break
                return self.scan_paragraph(start);
            }
        }

        if count < 3 {
            return self.scan_paragraph(start);
        }

        let end = self.current_position();
        self.advance_line();

        BlockToken {
            token_type: BlockTokenType::ThematicBreak,
            content: String::new(),
            position: self.make_position(start, end),
        }
    }

    fn scan_code_fence(&mut self, start: Option<Position>) -> BlockToken {
        let marker = self.current_char();
        let indent = self.count_indent();

        if indent > 3 {
            return self.scan_paragraph(start);
        }

        let mut fence_count = 0;
        while self.check_char(marker) {
            fence_count += 1;
            self.advance();
        }

        if fence_count < 3 {
            return self.scan_paragraph(start);
        }

        // Get info string (language)
        self.skip_spaces();
        let info_start = self.position;
        self.advance_to_line_end();
        let info = self.input[info_start..self.position].trim();
        self.advance_line();

        // Collect code content
        let mut content = String::new();
        let mut _found_closing = false;

        while !self.is_at_end() {
            let line_start = self.position;

            // Check for closing fence
            let mut temp_pos = self.position;
            let mut close_count = 0;
            while temp_pos < self.bytes.len() && self.bytes[temp_pos] == marker as u8 {
                close_count += 1;
                temp_pos += 1;
            }

            if close_count >= fence_count {
                // Found closing fence
                self.advance_to_line_end();
                self.advance_line();
                _found_closing = true;
                break;
            }

            // Add line to content
            self.advance_to_line_end();
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(&self.input[line_start..self.position]);
            self.advance_line();
        }

        let end = self.current_position();

        BlockToken {
            token_type: BlockTokenType::CodeFence {
                lang: if info.is_empty() {
                    None
                } else {
                    Some(info.to_string())
                },
                meta: None,
            },
            content,
            position: self.make_position(start, end),
        }
    }

    fn scan_blockquote(&mut self, start: Option<Position>) -> BlockToken {
        let mut lines = Vec::new();

        while self.check_char('>') {
            self.advance();
            self.skip_spaces();

            let line_start = self.position;
            self.advance_to_line_end();
            lines.push(self.input[line_start..self.position].to_string());
            self.advance_line();

            // Check for continuation
            if !self.check_char('>') && !self.is_blank_line() {
                break;
            }
        }

        let end = self.current_position();

        BlockToken {
            token_type: BlockTokenType::Blockquote,
            content: lines.join("\n"),
            position: self.make_position(start, end),
        }
    }

    fn scan_list_item(&mut self, start: Option<Position>) -> BlockToken {
        let indent = self.count_indent();
        let marker = self.current_char();
        let ordered = marker.is_ascii_digit();

        if ordered {
            // Scan number
            while self.current_char().is_ascii_digit() {
                self.advance();
            }
            // Must be followed by . or )
            if !self.check_char('.') && !self.check_char(')') {
                return self.scan_paragraph(start);
            }
            self.advance();
        } else {
            // Unordered list marker (-, +, *)
            self.advance();
        }

        // Require space after marker
        if !self.check_char(' ') && !self.check_char('\t') && !self.is_at_line_end() {
            return self.scan_paragraph(start);
        }

        self.skip_spaces();

        // Collect list item content
        let mut content = String::new();
        let content_start = self.position;
        self.advance_to_line_end();
        content.push_str(&self.input[content_start..self.position]);
        self.advance_line();

        // Check for continuation lines
        while !self.is_at_end() && !self.is_blank_line() {
            let line_indent = self.count_indent();
            if line_indent <= indent {
                break;
            }

            let line_start = self.position;
            self.advance_to_line_end();
            content.push('\n');
            content.push_str(&self.input[line_start..self.position]);
            self.advance_line();
        }

        let end = self.current_position();

        BlockToken {
            token_type: BlockTokenType::ListItem { ordered, marker },
            content,
            position: self.make_position(start, end),
        }
    }

    fn scan_paragraph(&mut self, start: Option<Position>) -> BlockToken {
        let mut content = String::new();

        while !self.is_at_end() && !self.is_blank_line() {
            // Check if next line starts a new block
            if self.position > 0 && self.bytes[self.position - 1] == b'\n' {
                let saved_pos = self.position;
                let saved_line = self.line;
                let saved_col = self.column;

                // Look ahead
                if self.check_heading()
                    || self.check_thematic_break()
                    || self.check_code_fence()
                    || self.check_char('>')
                    || self.check_list_marker()
                {
                    // Restore position and break
                    self.position = saved_pos;
                    self.line = saved_line;
                    self.column = saved_col;
                    break;
                }
            }

            let line_start = self.position;
            self.advance_to_line_end();
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(&self.input[line_start..self.position].trim_end());
            self.advance_line();
        }

        let end = self.current_position();

        BlockToken {
            token_type: BlockTokenType::Paragraph,
            content,
            position: self.make_position(start, end),
        }
    }

    // Optimized line scanning using memchr for better performance
    pub fn find_line_ends(&self) -> Vec<usize> {
        let mut line_ends = Vec::new();
        let mut pos = 0;

        // Use memchr for fast newline detection when available
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        {
            while pos < self.bytes.len() {
                if let Some(newline_pos) = memchr::memchr(b'\n', &self.bytes[pos..]) {
                    let absolute_pos = pos + newline_pos;
                    line_ends.push(absolute_pos);
                    pos = absolute_pos + 1;
                } else {
                    break;
                }
            }
        }

        // Fallback for other architectures
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            for (i, &byte) in self.bytes.iter().enumerate() {
                if byte == b'\n' {
                    line_ends.push(i);
                }
            }
        }

        line_ends
    }

    // Vectorized search for block delimiters (portable)
    pub fn find_block_delimiters(&self) -> Vec<(usize, char)> {
        let mut delimiters = Vec::new();
        let delim_chars = [b'#', b'>', b'-', b'_', b'*', b'`', b'~'];

        for (i, &byte) in self.bytes.iter().enumerate() {
            if delim_chars.contains(&byte) {
                delimiters.push((i, byte as char));
            }
        }

        delimiters
    }

    // Helper methods
    fn is_at_end(&self) -> bool {
        self.position >= self.bytes.len()
    }

    fn current_char(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.bytes[self.position] as char
        }
    }

    fn check_char(&self, c: char) -> bool {
        self.current_char() == c
    }

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            let c = self.current_char();
            self.position += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            c
        } else {
            '\0'
        }
    }

    fn skip_spaces(&mut self) {
        while self.check_char(' ') || self.check_char('\t') {
            self.advance();
        }
    }

    fn skip_blank_lines(&mut self) {
        while !self.is_at_end() && self.is_blank_line() {
            self.advance_line();
        }
    }

    fn is_blank_line(&self) -> bool {
        let mut pos = self.position;
        while pos < self.bytes.len() && (self.bytes[pos] == b' ' || self.bytes[pos] == b'\t') {
            pos += 1;
        }
        pos >= self.bytes.len() || self.bytes[pos] == b'\n'
    }

    fn is_at_line_end(&self) -> bool {
        self.is_at_end() || self.check_char('\n')
    }

    fn advance_to_line_end(&mut self) {
        while !self.is_at_line_end() {
            self.advance();
        }
    }

    fn advance_line(&mut self) {
        self.advance_to_line_end();
        if self.check_char('\n') {
            self.advance();
        }
    }

    fn count_indent(&self) -> usize {
        let mut count = 0;
        let mut pos = self.position;
        while pos < self.bytes.len() {
            match self.bytes[pos] {
                b' ' => count += 1,
                b'\t' => count += 4,
                _ => break,
            }
            pos += 1;
        }
        count
    }

    fn check_heading(&self) -> bool {
        self.check_char('#')
    }

    fn check_thematic_break(&self) -> bool {
        let c = self.current_char();
        (c == '-' || c == '_' || c == '*') && {
            // Look ahead to see if we have at least 3
            let mut count = 0;
            let mut pos = self.position;
            while pos < self.bytes.len() && self.bytes[pos] != b'\n' {
                if self.bytes[pos] == c as u8 {
                    count += 1;
                } else if self.bytes[pos] != b' ' && self.bytes[pos] != b'\t' {
                    return false;
                }
                pos += 1;
            }
            count >= 3
        }
    }

    fn check_code_fence(&self) -> bool {
        let c = self.current_char();
        (c == '`' || c == '~') && {
            let mut count = 0;
            let mut pos = self.position;
            while pos < self.bytes.len() && self.bytes[pos] == c as u8 {
                count += 1;
                pos += 1;
            }
            count >= 3
        }
    }

    fn check_list_marker(&self) -> bool {
        let c = self.current_char();
        c == '-' || c == '+' || c == '*' || c.is_ascii_digit()
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

    fn make_position(&self, start: Option<Position>, end: Option<Position>) -> Option<Position> {
        if let (Some(s), Some(e)) = (start, end) {
            Some(Position {
                start: s.start,
                end: e.end,
                source: None,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockToken {
    pub token_type: BlockTokenType,
    pub content: String,
    pub position: Option<Position>,
}

#[derive(Debug, Clone)]
pub enum BlockTokenType {
    Heading(u8),
    Paragraph,
    ThematicBreak,
    Blockquote,
    ListItem {
        ordered: bool,
        marker: char,
    },
    CodeFence {
        lang: Option<String>,
        meta: Option<String>,
    },
    Html,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_heading() {
        let input = "# Hello World\n\nParagraph";
        let mut scanner = Scanner::new(input, false);
        let tokens = scanner.scan_blocks();

        assert_eq!(tokens.len(), 2);
        match &tokens[0].token_type {
            BlockTokenType::Heading(depth) => assert_eq!(*depth, 1),
            _ => panic!("Expected heading"),
        }
        assert_eq!(tokens[0].content, "Hello World");
    }

    #[test]
    fn test_line_detection() {
        let input = "Line 1\nLine 2\nLine 3\n";
        let scanner = Scanner::new(input, false);
        let line_ends = scanner.find_line_ends();

        assert_eq!(line_ends, vec![6, 13, 20]);
    }
}
