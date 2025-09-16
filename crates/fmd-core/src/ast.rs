// AST node types for faster-md

use crate::position::Position;
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

    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    // Document
    #[default]
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

    // Math
    Math,
    InlineMath,

    // YAML
    Yaml,
}
