// Contract tests for incremental parsing
// These tests define the expected API and MUST FAIL until implementation

use fmd_core::{
    parse_incremental, Document, Edit, IncrementalCache, IncrementalSession, Node, Range,
};

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_incremental_basic_edit() {
    let mut cache = IncrementalCache::new();
    let doc = Document::new("# Hello\n\nWorld");
    let previous_ast = Node::default();

    // Initial parse
    let result = parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);
    assert!(result.success);
    assert_eq!(result.ast.children.len(), 2);

    // Edit the heading
    let _edit = Edit {
        range: Range {
            start: 2, // Position after '#'
            end: 7,   // End of "Hello"
        },
        text: "Goodbye".to_string(),
    };

    let doc = Document::new("# Goodbye\n\nWorld");
    let previous_ast = result.ast.clone();
    let result = parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);

    assert!(result.success);
    assert_eq!(result.reused_nodes, 1); // Paragraph should be reused
    assert!(result.changed_ranges.len() == 1);
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_incremental_node_reuse() {
    let mut cache = IncrementalCache::new();
    let previous_ast = Node::default();

    let markdown = "# Title\n\n## Section 1\n\nContent 1\n\n## Section 2\n\nContent 2";
    let doc = Document::new(markdown);
    let initial = parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);

    // Edit only Section 1's content
    let edited = "# Title\n\n## Section 1\n\nEdited content\n\n## Section 2\n\nContent 2";
    let doc = Document::new(edited);
    let previous_ast = initial.ast.clone();
    let result = parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);

    assert!(result.success);
    // Should reuse Title, Section 2 heading, and Section 2 content
    assert!(result.reused_nodes >= 3);
    assert!(result.total_nodes >= 5);

    let reuse_percentage = (result.reused_nodes as f64 / result.total_nodes as f64) * 100.0;
    assert!(reuse_percentage >= 60.0); // At least 60% reuse
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_incremental_multiple_edits() {
    let mut cache = IncrementalCache::new();
    let mut session = IncrementalSession::new();
    let previous_ast = Node::default();

    let doc = Document::new("Line 1\n\nLine 2\n\nLine 3");
    let result = parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);

    // Apply multiple edits
    let edits = vec![
        Edit {
            range: Range { start: 5, end: 6 },
            text: "A".to_string(),
        },
        Edit {
            range: Range { start: 19, end: 20 },
            text: "B".to_string(),
        },
    ];

    session.apply_edits(&edits);

    let doc = Document::new("Line A\n\nLine 2\n\nLine B");
    let previous_ast = result.ast.clone();
    let result = parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);

    assert!(result.success);
    assert_eq!(result.changed_ranges.len(), 2);
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_incremental_performance() {
    let mut cache = IncrementalCache::new();
    let previous_ast = Node::default();

    // Create a large document (1000 paragraphs)
    let mut content = String::new();
    for i in 0..1000 {
        content.push_str(&format!("Paragraph {}\n\n", i));
    }

    let doc = Document::new(&content);
    let initial_start = std::time::Instant::now();
    let initial = parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);
    let initial_duration = initial_start.elapsed();

    // Edit just one paragraph in the middle
    content.replace_range(2500..2510, "EDITED");
    let doc = Document::new(&content);

    let incremental_start = std::time::Instant::now();
    let previous_ast = initial.ast.clone();
    let result = parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);
    let incremental_duration = incremental_start.elapsed();

    assert!(result.success);
    // Incremental parse should be much faster
    assert!(incremental_duration < initial_duration / 10);

    // Should reuse >90% of nodes
    let reuse_percentage = (result.reused_nodes as f64 / result.total_nodes as f64) * 100.0;
    assert!(reuse_percentage >= 90.0);
}

#[test]
#[ignore = "Implementation not complete - will fail"]
fn test_incremental_session_reset() {
    let mut session = IncrementalSession::new();
    let mut cache = IncrementalCache::new();
    let previous_ast = Node::default();

    let doc = Document::new("# Initial");
    parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);

    assert!(session.has_tree());

    session.reset();
    assert!(!session.has_tree());

    // Should work like initial parse after reset
    let doc = Document::new("# New Document");
    let previous_ast = Node::default();
    let mut cache = IncrementalCache::new();
    let result = parse_incremental(&doc, Default::default(), &previous_ast, &mut cache);
    assert!(result.success);
    assert_eq!(result.reused_nodes, 0);
}
