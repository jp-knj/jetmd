// WASM bindings for faster-md

use wasm_bindgen::prelude::*;
use fmd_core::{Document, ProcessorOptions, ParseResult};
use fmd_html::{HtmlOptions, render_html};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

// Re-export types needed by JavaScript
#[wasm_bindgen]
pub struct WasmProcessor {
    options: ProcessorOptions,
}

#[wasm_bindgen]
impl WasmProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            options: ProcessorOptions::default(),
        }
    }
    
    pub fn with_options(&mut self, options: JsValue) -> Result<(), JsValue> {
        let opts: ProcessorOptions = from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?;
        self.options = opts;
        Ok(())
    }
}

// Main parsing function exposed to JavaScript
#[wasm_bindgen]
pub fn parse_to_ast(content: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let opts: ProcessorOptions = if options.is_undefined() || options.is_null() {
        ProcessorOptions::default()
    } else {
        from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
    };
    
    let doc = Document::new(content);
    let result = fmd_core::parse(&doc, opts);
    
    to_value(&result.ast).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

// HTML rendering function
#[wasm_bindgen]
pub fn render_html_wasm(content: &str, options: JsValue) -> Result<String, JsValue> {
    let processor_opts: ProcessorOptions = if options.is_undefined() || options.is_null() {
        ProcessorOptions::default()
    } else {
        from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
    };
    
    let doc = Document::new(content);
    let parse_result = fmd_core::parse(&doc, processor_opts);
    
    if !parse_result.success {
        let errors = parse_result.errors.join("; ");
        return Err(JsValue::from_str(&format!("Parse error: {}", errors)));
    }
    
    let html_opts = HtmlOptions::default();
    Ok(render_html(&parse_result.ast, html_opts))
}

// Session management for incremental parsing
#[wasm_bindgen]
pub struct Session {
    last_ast: Option<fmd_core::Node>,
    last_content: String,
}

#[wasm_bindgen]
impl Session {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            last_ast: None,
            last_content: String::new(),
        }
    }
    
    pub fn parse_incremental(&mut self, content: &str, options: JsValue) -> Result<JsValue, JsValue> {
        let opts: ProcessorOptions = if options.is_undefined() || options.is_null() {
            ProcessorOptions::default()
        } else {
            from_value(options)
                .map_err(|e| JsValue::from_str(&format!("Invalid options: {}", e)))?
        };
        
        // TODO: Implement incremental parsing logic
        let doc = Document::new(content);
        let result = fmd_core::parse(&doc, opts);
        
        if result.success {
            self.last_ast = Some(result.ast.clone());
            self.last_content = content.to_string();
        }
        
        to_value(&result.ast).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }
    
    pub fn get_last_ast(&self) -> Result<JsValue, JsValue> {
        match &self.last_ast {
            Some(ast) => to_value(ast).map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
            None => Err(JsValue::from_str("No previous parse result"))
        }
    }
}

// Module initialization
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages in browser
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}