// GitHub Flavored Markdown extensions for faster-md

use fmd_core::Node;

pub struct GfmExtensions;

impl GfmExtensions {
    pub fn new() -> Self {
        Self
    }
    
    pub fn parse_table(&self, _input: &str) -> Option<Node> {
        // TODO: Implement table parsing
        None
    }
    
    pub fn parse_strikethrough(&self, _input: &str) -> Option<Node> {
        // TODO: Implement strikethrough parsing
        None
    }
    
    pub fn parse_autolink(&self, _input: &str) -> Option<Node> {
        // TODO: Implement autolink parsing
        None
    }
}