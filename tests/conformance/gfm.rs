// GitHub Flavored Markdown (GFM) Conformance Test Suite
// These tests MUST FAIL until implementation is complete

use fmd_core::{parse, Document, ProcessorOptions};
use fmd_gfm::GfmExtensions;
use fmd_html::render_html;

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_gfm_tables() {
    let markdown = r#"
| Header 1 | Header 2 | Header 3 |
|----------|:--------:|---------:|
| Left     | Center   | Right    |
| Cell 1   | Cell 2   | Cell 3   |
"#;

    let doc = Document::new(markdown);
    let options = ProcessorOptions {
        gfm: true,
        ..Default::default()
    };
    
    let result = parse(&doc, options);
    let html = render_html(&result.ast, Default::default());
    
    assert!(html.contains("<table>"));
    assert!(html.contains("<thead>"));
    assert!(html.contains("<tbody>"));
    assert!(html.contains("<th>Header 1</th>"));
    assert!(html.contains("text-align: center"));
    assert!(html.contains("text-align: right"));
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_gfm_strikethrough() {
    let markdown = "~~strikethrough text~~";
    
    let doc = Document::new(markdown);
    let options = ProcessorOptions {
        gfm: true,
        ..Default::default()
    };
    
    let result = parse(&doc, options);
    let html = render_html(&result.ast, Default::default());
    
    assert!(html.contains("<del>"));
    assert!(html.contains("strikethrough text"));
    assert!(html.contains("</del>"));
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_gfm_autolinks() {
    let markdown = "Visit https://github.com for code";
    
    let doc = Document::new(markdown);
    let options = ProcessorOptions {
        gfm: true,
        ..Default::default()
    };
    
    let result = parse(&doc, options);
    let html = render_html(&result.ast, Default::default());
    
    assert!(html.contains(r#"<a href="https://github.com">"#));
    assert!(html.contains("https://github.com</a>"));
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_gfm_task_lists() {
    let markdown = r#"
- [x] Completed task
- [ ] Pending task
- Regular item
"#;

    let doc = Document::new(markdown);
    let options = ProcessorOptions {
        gfm: true,
        ..Default::default()
    };
    
    let result = parse(&doc, options);
    let html = render_html(&result.ast, Default::default());
    
    assert!(html.contains(r#"<input type="checkbox" checked"#));
    assert!(html.contains(r#"<input type="checkbox""#));
    assert!(!html.contains(r#"<input type="checkbox">Regular"#));
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_gfm_table_alignment() {
    let markdown = r#"
| Left | Center | Right | Default |
|:-----|:------:|------:|---------|
| L    | C      | R     | D       |
"#;

    let doc = Document::new(markdown);
    let options = ProcessorOptions {
        gfm: true,
        ..Default::default()
    };
    
    let result = parse(&doc, options);
    
    // Check AST structure
    let table = &result.ast.children[0];
    assert_eq!(table.node_type, "table");
    
    let alignments = table.data["align"].as_array().unwrap();
    assert_eq!(alignments[0], "left");
    assert_eq!(alignments[1], "center");
    assert_eq!(alignments[2], "right");
    assert_eq!(alignments[3], serde_json::Value::Null);
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_gfm_extensions_disabled() {
    // When GFM is disabled, these should not be parsed as special elements
    let markdown = "~~not strikethrough~~ and https://example.com";
    
    let doc = Document::new(markdown);
    let options = ProcessorOptions {
        gfm: false,
        ..Default::default()
    };
    
    let result = parse(&doc, options);
    let html = render_html(&result.ast, Default::default());
    
    assert!(!html.contains("<del>"));
    assert!(!html.contains(r#"<a href="https://example.com""#));
    assert!(html.contains("~~not strikethrough~~"));
}