// CommonMark 0.30 Conformance Test Suite
// These tests MUST FAIL until implementation is complete
// Target: ≥99.5% conformance

use fmd_core::{parse, Document};
use fmd_html::render_html;
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone)]
struct CommonMarkTest {
    markdown: String,
    html: String,
    section: String,
    example: u32,
}

fn load_commonmark_tests() -> Vec<CommonMarkTest> {
    // Load CommonMark 0.30 spec tests
    // Download from: https://spec.commonmark.org/0.30/spec.json
    let spec_json = fs::read_to_string("tests/fixtures/commonmark-spec.json")
        .expect("CommonMark spec.json not found. Download from https://spec.commonmark.org/0.30/spec.json");
    
    let spec: Vec<Value> = serde_json::from_str(&spec_json)
        .expect("Failed to parse CommonMark spec.json");
    
    spec.iter()
        .map(|test| CommonMarkTest {
            markdown: test["markdown"].as_str().unwrap().to_string(),
            html: test["html"].as_str().unwrap().to_string(),
            section: test["section"].as_str().unwrap().to_string(),
            example: test["example"].as_u64().unwrap() as u32,
        })
        .collect()
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_commonmark_conformance() {
    let tests = load_commonmark_tests();
    let total_tests = tests.len();
    let mut passed = 0;
    let mut failed_examples = Vec::new();
    
    for test in tests {
        let doc = Document::new(&test.markdown);
        let result = parse(&doc, Default::default());
        let html = render_html(&result.ast, Default::default());
        
        if html.trim() == test.html.trim() {
            passed += 1;
        } else {
            failed_examples.push((test.example, test.section.clone()));
            
            // Log failures for debugging
            eprintln!(
                "Example {} ({}) failed:",
                test.example, test.section
            );
            eprintln!("  Input: {:?}", test.markdown);
            eprintln!("  Expected: {:?}", test.html);
            eprintln!("  Got: {:?}", html);
            eprintln!();
        }
    }
    
    let conformance = (passed as f64 / total_tests as f64) * 100.0;
    
    println!("CommonMark Conformance: {:.2}%", conformance);
    println!("Passed: {}/{}", passed, total_tests);
    
    if !failed_examples.is_empty() {
        println!("\nFailed examples:");
        for (example, section) in failed_examples.iter().take(10) {
            println!("  - Example {} ({})", example, section);
        }
        if failed_examples.len() > 10 {
            println!("  ... and {} more", failed_examples.len() - 10);
        }
    }
    
    // Require ≥99.5% conformance
    assert!(
        conformance >= 99.5,
        "CommonMark conformance {:.2}% is below required 99.5%",
        conformance
    );
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_commonmark_blocks() {
    let doc = Document::new("# Heading\n\nParagraph\n\n```rust\ncode\n```");
    let result = parse(&doc, Default::default());
    
    assert_eq!(result.ast.children.len(), 3);
    assert_eq!(result.ast.children[0].node_type, "heading");
    assert_eq!(result.ast.children[1].node_type, "paragraph");
    assert_eq!(result.ast.children[2].node_type, "code");
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_commonmark_inlines() {
    let doc = Document::new("**bold** *italic* `code` [link](url)");
    let result = parse(&doc, Default::default());
    let para = &result.ast.children[0];
    
    assert_eq!(para.node_type, "paragraph");
    assert!(para.children.len() >= 7); // Text and inline elements
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_commonmark_lists() {
    let doc = Document::new("- Item 1\n- Item 2\n  - Nested\n\n1. Ordered\n2. Second");
    let result = parse(&doc, Default::default());
    
    assert_eq!(result.ast.children.len(), 2);
    assert_eq!(result.ast.children[0].node_type, "list");
    assert_eq!(result.ast.children[0].data["ordered"], false);
    assert_eq!(result.ast.children[1].node_type, "list");
    assert_eq!(result.ast.children[1].data["ordered"], true);
}