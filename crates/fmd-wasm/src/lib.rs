// WASM bindings for faster-md
use wasm_bindgen::prelude::*;

mod parse;
mod render;
mod session;

pub use parse::*;
pub use render::*;
pub use session::*;

// Module initialization
#[wasm_bindgen(start)]
pub fn init() {
    // Set panic hook for better error messages in browser
}

// Version information
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Feature detection
#[wasm_bindgen]
pub fn features() -> JsValue {
    let features = serde_json::json!({
        "gfm": true,
        "tables": true,
        "strikethrough": true,
        "autolinks": true,
        "incremental": true,
        "sanitization": true,
        "positions": true,
        "mdx": false, // Will be true when MDX is implemented
    });

    JsValue::from_str(&features.to_string())
}
