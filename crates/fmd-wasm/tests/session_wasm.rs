// Contract tests for WASM session management
// These tests define the expected API and MUST FAIL until implementation

use wasm_bindgen_test::*;
use fmd_wasm::{Session, ProcessorOptions, Edit};
use serde_json::Value;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_session_create_and_parse() {
    let mut session = Session::new();
    let markdown = "# Initial Document";
    let options = ProcessorOptions::new();
    
    let result = session.parse(markdown, &options);
    let ast: Value = serde_json::from_str(&result).unwrap();
    
    assert_eq!(ast["type"], "root");
    assert_eq!(ast["children"][0]["type"], "heading");
    assert!(session.has_tree());
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_session_incremental_edit() {
    let mut session = Session::new();
    let options = ProcessorOptions::new();
    
    // Initial parse
    let markdown = "# Title\n\n## Section 1\n\nContent";
    session.parse(markdown, &options);
    
    // Apply edit
    let edit = Edit::new(2, 7, "Header"); // Replace "Title" with "Header"
    session.apply_edit(&edit);
    
    // Incremental parse
    let updated = "# Header\n\n## Section 1\n\nContent";
    let result = session.parse_incremental(updated, &options);
    let stats: Value = serde_json::from_str(&result).unwrap();
    
    assert!(stats["success"].as_bool().unwrap());
    assert!(stats["reusedNodes"].as_u64().unwrap() >= 2); // Section and content reused
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_session_multiple_edits() {
    let mut session = Session::new();
    let options = ProcessorOptions::new();
    
    let markdown = "Line 1\n\nLine 2\n\nLine 3";
    session.parse(markdown, &options);
    
    // Apply multiple edits
    let edits = vec![
        Edit::new(5, 6, "A"), // Line 1 -> Line A
        Edit::new(19, 20, "B"), // Line 3 -> Line B
    ];
    
    for edit in edits {
        session.apply_edit(&edit);
    }
    
    let updated = "Line A\n\nLine 2\n\nLine B";
    let result = session.parse_incremental(updated, &options);
    let stats: Value = serde_json::from_str(&result).unwrap();
    
    assert!(stats["success"].as_bool().unwrap());
    assert_eq!(stats["changedRanges"].as_array().unwrap().len(), 2);
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_session_reset() {
    let mut session = Session::new();
    let options = ProcessorOptions::new();
    
    session.parse("# Document", &options);
    assert!(session.has_tree());
    
    session.reset();
    assert!(!session.has_tree());
    
    // Should work like initial parse after reset
    let result = session.parse("# New Document", &options);
    let ast: Value = serde_json::from_str(&result).unwrap();
    assert_eq!(ast["children"][0]["children"][0]["value"], "New Document");
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_session_performance_benefit() {
    let mut session = Session::new();
    let options = ProcessorOptions::new();
    
    // Large document (100 paragraphs)
    let mut markdown = String::new();
    for i in 0..100 {
        markdown.push_str(&format!("Paragraph {}\n\n", i));
    }
    
    // Initial parse
    let start = js_sys::Date::now();
    session.parse(&markdown, &options);
    let initial_time = js_sys::Date::now() - start;
    
    // Small edit
    session.apply_edit(&Edit::new(10, 11, "X")); // Change one character
    markdown.replace_range(10..11, "X");
    
    // Incremental parse
    let start = js_sys::Date::now();
    let result = session.parse_incremental(&markdown, &options);
    let incremental_time = js_sys::Date::now() - start;
    
    let stats: Value = serde_json::from_str(&result).unwrap();
    let reuse_percentage = stats["reusePercentage"].as_f64().unwrap();
    
    // Incremental should be much faster
    assert!(incremental_time < initial_time / 5.0);
    assert!(reuse_percentage >= 90.0);
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_session_memory_management() {
    let mut session = Session::new();
    let options = ProcessorOptions::new();
    
    // Parse multiple documents
    for i in 0..10 {
        let markdown = format!("# Document {}", i);
        session.parse(&markdown, &options);
    }
    
    // Session should manage memory properly
    // This is more of a stability test - shouldn't crash or leak
    assert!(session.has_tree());
    
    // Get memory stats
    let stats = session.get_stats();
    let stats_value: Value = serde_json::from_str(&stats).unwrap();
    assert!(stats_value["memoryUsage"].is_number());
}