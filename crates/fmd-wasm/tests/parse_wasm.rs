// Contract tests for WASM parseToAst() function
// These tests define the expected API and MUST FAIL until implementation

use wasm_bindgen_test::*;
use fmd_wasm::{parse_to_ast, ProcessorOptions};
use serde_json::Value;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_parse_to_ast_basic() {
    let markdown = "# Hello World";
    let options = ProcessorOptions::new();
    
    let result = parse_to_ast(markdown, &options);
    let ast: Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(ast["type"], "root");
    assert_eq!(ast["children"][0]["type"], "heading");
    assert_eq!(ast["children"][0]["depth"], 1);
    assert_eq!(ast["children"][0]["children"][0]["type"], "text");
    assert_eq!(ast["children"][0]["children"][0]["value"], "Hello World");
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_parse_to_ast_with_position() {
    let markdown = "# Heading";
    let mut options = ProcessorOptions::new();
    options.set_position(true);
    
    let result = parse_to_ast(markdown, &options);
    let ast: Value = serde_json::from_str(&result).unwrap();
    
    assert!(ast["children"][0]["position"].is_object());
    assert_eq!(ast["children"][0]["position"]["start"]["line"], 1);
    assert_eq!(ast["children"][0]["position"]["start"]["column"], 1);
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_parse_to_ast_with_gfm() {
    let markdown = "~~strikethrough~~";
    let mut options = ProcessorOptions::new();
    options.set_gfm(true);
    
    let result = parse_to_ast(markdown, &options);
    let ast: Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(ast["children"][0]["children"][0]["type"], "delete");
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_parse_to_ast_performance() {
    // 50KB document
    let markdown = "# Heading\n\nParagraph\n\n".repeat(2500);
    let options = ProcessorOptions::new();
    
    let start = js_sys::Date::now();
    let _result = parse_to_ast(&markdown, &options);
    let duration = js_sys::Date::now() - start;
    
    // Should parse in <15ms (p99 target)
    assert!(duration < 15.0, "Parse took {}ms, expected <15ms", duration);
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_parse_to_ast_with_frontmatter() {
    let markdown = "---\ntitle: Test\n---\n\n# Content";
    let mut options = ProcessorOptions::new();
    options.set_frontmatter(true);
    
    let result = parse_to_ast(markdown, &options);
    let ast: Value = serde_json::from_str(&result).unwrap();
    
    // Frontmatter should be in the AST
    assert_eq!(ast["children"][0]["type"], "frontmatter");
    assert_eq!(ast["children"][0]["format"], "yaml");
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_parse_to_ast_error_handling() {
    let markdown = "```\nunclosed code block";
    let options = ProcessorOptions::new();
    
    // Should not panic, should return valid AST
    let result = parse_to_ast(markdown, &options);
    let ast: Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(ast["type"], "root");
    // Should have a code block even if unclosed
    assert_eq!(ast["children"][0]["type"], "code");
}