// MDX core parsing and transformation

use fmd_core::{Node, NodeType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdxOptions {
    pub jsx: bool,
    pub jsx_import_source: Option<String>,
    pub jsx_runtime: JsxRuntime,
    pub development: bool,
    pub pragma: Option<String>,
    pub pragma_frag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JsxRuntime {
    Classic,
    Automatic,
}

impl Default for MdxOptions {
    fn default() -> Self {
        Self {
            jsx: true,
            jsx_import_source: Some("react".to_string()),
            jsx_runtime: JsxRuntime::Automatic,
            development: false,
            pragma: None,
            pragma_frag: None,
        }
    }
}

pub struct MdxParser {
    options: MdxOptions,
}

impl MdxParser {
    pub fn new(options: MdxOptions) -> Self {
        Self { options }
    }

    pub fn parse_jsx_element(&self, input: &str) -> Option<Node> {
        // TODO: Implement JSX element parsing
        // This will parse <Component prop="value">content</Component>
        None
    }

    pub fn parse_jsx_expression(&self, input: &str) -> Option<Node> {
        // TODO: Implement JSX expression parsing
        // This will parse {expression}
        None
    }

    pub fn parse_esm_import(&self, input: &str) -> Option<Node> {
        // TODO: Implement ESM import/export parsing
        // This will parse import/export statements
        None
    }
}

pub fn transform_mdx_ast(ast: Node, options: MdxOptions) -> TransformResult {
    // TODO: Transform MDX AST to include JSX nodes
    TransformResult {
        ast,
        imports: vec![],
        exports: vec![],
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformResult {
    pub ast: Node,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

pub fn compile_to_js(ast: Node, options: MdxOptions) -> String {
    // TODO: Compile MDX AST to JavaScript/JSX
    let mut output = String::new();

    // Add runtime imports based on jsx_runtime
    match options.jsx_runtime {
        JsxRuntime::Automatic => {
            if let Some(source) = &options.jsx_import_source {
                output.push_str(&format!(
                    "import {{jsx as _jsx}} from '{}/jsx-runtime';\n",
                    source
                ));
            }
        }
        JsxRuntime::Classic => {
            if let Some(pragma) = &options.pragma {
                output.push_str(&format!("import {} from 'react';\n", pragma));
            } else {
                output.push_str("import React from 'react';\n");
            }
        }
    }

    // TODO: Generate component function
    output.push_str("\nexport default function MDXContent(props) {\n");
    output.push_str("  return _jsx('div', {children: 'MDX compiler not implemented'});\n");
    output.push_str("}\n");

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let opts = MdxOptions::default();
        assert!(opts.jsx);
        assert_eq!(opts.jsx_import_source, Some("react".to_string()));
    }
}
