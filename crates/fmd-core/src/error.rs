// Enhanced error handling for fmd-core
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorKind {
    InvalidSyntax,
    UnclosedBlock,
    InvalidFrontmatter,
    InvalidGfmTable,
    InvalidMdxComponent,
    UnexpectedEof,
    InvalidUtf8,
    RecursionLimit,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub length: usize,
    pub source: Option<String>,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, message: impl Into<String>) -> Self {
        ParseError {
            kind,
            message: message.into(),
            line: 0,
            column: 0,
            offset: 0,
            length: 0,
            source: None,
        }
    }

    pub fn with_position(mut self, line: usize, column: usize, offset: usize) -> Self {
        self.line = line;
        self.column = column;
        self.offset = offset;
        self
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn format_with_context(&self) -> String {
        let mut output = format!(
            "Parse error at line {}, column {}: {}\n",
            self.line + 1,
            self.column + 1,
            self.message
        );

        if let Some(ref source) = self.source {
            let lines: Vec<&str> = source.lines().collect();
            if self.line < lines.len() {
                output.push_str(&format!("\n{}\n", lines[self.line]));
                output.push_str(&format!(
                    "{:>width$}\n",
                    "^".repeat(self.length.max(1)),
                    width = self.column + self.length
                ));
            }
        }

        output
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line + 1,
            self.column + 1,
            self.message
        )
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Vec<ParseError>,
    warnings: Vec<ParseError>,
    max_errors: usize,
}

impl ErrorCollector {
    pub fn new() -> Self {
        ErrorCollector {
            errors: Vec::new(),
            warnings: Vec::new(),
            max_errors: 100,
        }
    }

    pub fn with_max_errors(mut self, max: usize) -> Self {
        self.max_errors = max;
        self
    }

    pub fn add_error(&mut self, error: ParseError) -> bool {
        if self.errors.len() < self.max_errors {
            self.errors.push(error);
            true
        } else {
            false
        }
    }

    pub fn add_warning(&mut self, warning: ParseError) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    pub fn warnings(&self) -> &[ParseError] {
        &self.warnings
    }

    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
    }

    pub fn format_all(&self) -> String {
        let mut output = String::new();

        if !self.errors.is_empty() {
            output.push_str(&format!("Found {} error(s):\n", self.errors.len()));
            for error in &self.errors {
                output.push_str(&error.format_with_context());
                output.push('\n');
            }
        }

        if !self.warnings.is_empty() {
            output.push_str(&format!("Found {} warning(s):\n", self.warnings.len()));
            for warning in &self.warnings {
                output.push_str(&warning.format_with_context());
                output.push('\n');
            }
        }

        output
    }
}

pub type ParseResult<T> = Result<T, ParseError>;