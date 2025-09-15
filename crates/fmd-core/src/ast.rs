// AST node types for faster-md

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "type")]
    pub node_type: NodeType,
    
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub data: HashMap<String, Value>,
    
    // Specific fields for certain node types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u8>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordered: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    // Document
    Root,
    
    // Block
    Paragraph,
    Heading,
    ThematicBreak,
    Blockquote,
    List,
    ListItem,
    Code,
    Html,
    Definition,
    FrontMatter,
    
    // Inline
    Text,
    Emphasis,
    Strong,
    InlineCode,
    Break,
    Link,
    Image,
    LinkReference,
    ImageReference,
    
    // GFM Extensions
    Table,
    TableRow,
    TableCell,
    Delete,
    FootnoteDefinition,
    FootnoteReference,
    
    // MDX
    MdxjsEsm,
    MdxJsxFlowElement,
    MdxJsxTextElement,
    MdxFlowExpression,
    MdxTextExpression,
    
    // Directives
    ContainerDirective,
    LeafDirective,
    TextDirective,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Root
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub start: Point,
    pub end: Point,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}