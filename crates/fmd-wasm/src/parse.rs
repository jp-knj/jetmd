// WASM parsing module
use wasm_bindgen::prelude::*;
use fmd_core::{Document, ProcessorOptions, ParseResult, Node};
use serde_wasm_bindgen::{from_value, to_value};

/// Parse Markdown to AST
#[wasm_bindgen]
pub fn parseToAst(content: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let opts: ProcessorOptions = if options.is_undefined() || options.is_null() {
        ProcessorOptions::default()
    } else {
        from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
    };
    
    let doc = Document::new(content);
    let result = fmd_core::parse(&doc, opts);
    
    // Return full parse result with AST and metadata
    let js_result = serde_json::json!({
        "ast": result.ast,
        "success": result.success,
        "errors": result.errors,
        "parseTime": result.parse_time_ns,
        "nodeCount": count_nodes(&result.ast),
    });
    
    to_value(&js_result).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Parse with position information
#[wasm_bindgen]
pub fn parseWithPositions(content: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let mut opts: ProcessorOptions = if options.is_undefined() || options.is_null() {
        ProcessorOptions::default()
    } else {
        from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
    };
    
    // Ensure positions are tracked
    opts.track_positions = true;
    
    let doc = Document::new(content);
    let result = fmd_core::parse(&doc, opts);
    
    to_value(&result).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get parsing statistics
#[wasm_bindgen]
pub fn getParseStats(content: &str) -> Result<JsValue, JsValue> {
    let doc = Document::new(content);
    let result = fmd_core::parse(&doc, ProcessorOptions::default());
    
    let stats = serde_json::json!({
        "nodeCount": count_nodes(&result.ast),
        "parseTime": result.parse_time_ns,
        "success": result.success,
        "errorCount": result.errors.len(),
        "documentLength": content.len(),
        "lineCount": content.lines().count(),
    });
    
    to_value(&stats).map_err(|e| JsValue::from_str(&format!("Statistics error: {}", e)))
}

fn count_nodes(node: &Node) -> usize {
    let mut count = 1;
    for child in &node.children {
        count += count_nodes(child);
    }
    count
}