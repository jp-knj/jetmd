// WASM HTML rendering module
use wasm_bindgen::prelude::*;
use fmd_core::{Document, ProcessorOptions, GfmOptions};
use fmd_html::{HtmlOptions, render_html, sanitize::SanitizeOptions};
use serde_wasm_bindgen::{from_value, to_value};

/// Render Markdown to HTML
#[wasm_bindgen]
pub fn renderHtml(content: &str, options: JsValue) -> Result<String, JsValue> {
    // Parse options from JavaScript
    let js_opts = if options.is_undefined() || options.is_null() {
        serde_json::json!({})
    } else {
        from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
    };
    
    // Extract processor options
    let mut processor_opts = ProcessorOptions::default();
    if let Some(gfm) = js_opts.get("gfm").and_then(|v| v.as_bool()) {
        processor_opts.gfm = gfm;
    }
    
    // Parse the document
    let doc = Document::new(content);
    let parse_result = fmd_core::parse(&doc, processor_opts);
    
    if !parse_result.success {
        let errors = parse_result.errors.join("; ");
        return Err(JsValue::from_str(&format!("Parse error: {}", errors)));
    }
    
    // Extract HTML options
    let mut html_opts = HtmlOptions::default();
    if let Some(sanitize) = js_opts.get("sanitize").and_then(|v| v.as_bool()) {
        html_opts.sanitize = sanitize;
    }
    if let Some(xhtml) = js_opts.get("xhtml").and_then(|v| v.as_bool()) {
        html_opts.xhtml = xhtml;
    }
    
    Ok(render_html(&parse_result.ast, html_opts))
}

/// Render with custom sanitization options
#[wasm_bindgen]
pub fn renderHtmlSafe(content: &str) -> Result<String, JsValue> {
    let doc = Document::new(content);
    let parse_result = fmd_core::parse(&doc, ProcessorOptions::default());
    
    if !parse_result.success {
        return Err(JsValue::from_str("Parse error"));
    }
    
    let mut html_opts = HtmlOptions::default();
    html_opts.sanitize = true;
    html_opts.sanitize_options = SanitizeOptions::strict();
    
    Ok(render_html(&parse_result.ast, html_opts))
}

/// Render with GFM extensions
#[wasm_bindgen]
pub fn renderGfm(content: &str, options: JsValue) -> Result<String, JsValue> {
    let mut processor_opts = ProcessorOptions::default();
    processor_opts.gfm = true;
    
    // Configure GFM options if provided
    if !options.is_undefined() && !options.is_null() {
        let gfm_opts: serde_json::Value = from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid GFM options: {}", e)))?;
            
        if let Some(tables) = gfm_opts.get("tables").and_then(|v| v.as_bool()) {
            processor_opts.gfm_options.tables = tables;
        }
        if let Some(strikethrough) = gfm_opts.get("strikethrough").and_then(|v| v.as_bool()) {
            processor_opts.gfm_options.strikethrough = strikethrough;
        }
        if let Some(autolinks) = gfm_opts.get("autolinks").and_then(|v| v.as_bool()) {
            processor_opts.gfm_options.autolinks = autolinks;
        }
    }
    
    let doc = Document::new(content);
    let parse_result = fmd_core::parse(&doc, processor_opts);
    
    if !parse_result.success {
        return Err(JsValue::from_str("GFM parse error"));
    }
    
    Ok(render_html(&parse_result.ast, HtmlOptions::default()))
}

/// Get render statistics
#[wasm_bindgen]
pub fn getRenderStats(content: &str) -> Result<JsValue, JsValue> {
    let start = js_sys::Date::now();
    
    let doc = Document::new(content);
    let parse_result = fmd_core::parse(&doc, ProcessorOptions::default());
    let parse_time = js_sys::Date::now() - start;
    
    let render_start = js_sys::Date::now();
    let html = render_html(&parse_result.ast, HtmlOptions::default());
    let render_time = js_sys::Date::now() - render_start;
    
    let stats = serde_json::json!({
        "parseTime": parse_time,
        "renderTime": render_time,
        "totalTime": parse_time + render_time,
        "htmlLength": html.len(),
        "inputLength": content.len(),
        "compressionRatio": html.len() as f64 / content.len() as f64,
    });
    
    to_value(&stats).map_err(|e| JsValue::from_str(&format!("Statistics error: {}", e)))
}