// WASM parsing module
use fmd_core::{Document, Node, ProcessorOptions};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

/// Parse Markdown to AST
#[wasm_bindgen(js_name = parseToAst)]
pub fn parse_to_ast(content: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let mut opts: ProcessorOptions = if options.is_undefined() || options.is_null() {
        ProcessorOptions::default()
    } else {
        from_value(options).map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
    };

    // Enable all GFM features when gfm is true
    if opts.gfm {
        opts.gfm_options.tables = true;
        opts.gfm_options.strikethrough = true;
        opts.gfm_options.autolinks = true;
        opts.gfm_options.tasklists = true;
    }

    let doc = Document::new(content);
    let result = fmd_core::parse(&doc, opts);

    // Debug logging disabled for production

    // Return full parse result with AST and metadata
    let js_result = serde_json::json!({
        "ast": result.ast,
        "success": result.success,
        "errors": result.errors,
        "parseTime": result.parse_time_ns,
        "nodeCount": count_nodes(&result.ast),
    });

    // Convert to JSON string then parse in JavaScript
    Ok(JsValue::from_str(&js_result.to_string()))
}

/// Parse with position information
#[wasm_bindgen(js_name = parseWithPositions)]
pub fn parse_with_positions(content: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let mut opts: ProcessorOptions = if options.is_undefined() || options.is_null() {
        ProcessorOptions::default()
    } else {
        from_value(options).map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
    };

    // Ensure positions are tracked
    opts.track_positions = true;

    let doc = Document::new(content);
    let result = fmd_core::parse(&doc, opts);

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Get parsing statistics
#[wasm_bindgen(js_name = getParseStats)]
pub fn get_parse_stats(content: &str) -> Result<JsValue, JsValue> {
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

    serde_wasm_bindgen::to_value(&stats)
        .map_err(|e| JsValue::from_str(&format!("Statistics error: {}", e)))
}

fn count_nodes(node: &Node) -> usize {
    let mut count = 1;
    for child in &node.children {
        count += count_nodes(child);
    }
    count
}
