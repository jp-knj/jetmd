// Contract tests for WASM renderHtml() function
// These tests define the expected API and MUST FAIL until implementation

use fmd_wasm::{render_html, HtmlOptions, ProcessorOptions};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_render_html_basic() {
    let markdown = "# Hello World\n\nThis is a paragraph.";
    let processor_options = ProcessorOptions::new();
    let html_options = HtmlOptions::new();

    let html = render_html(markdown, &processor_options, &html_options);

    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<p>This is a paragraph.</p>"));
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_render_html_with_sanitization() {
    let markdown = "<script>alert('xss')</script>\n\n# Safe Content";
    let processor_options = ProcessorOptions::new();
    let mut html_options = HtmlOptions::new();
    html_options.set_sanitize(true);

    let html = render_html(markdown, &processor_options, &html_options);

    assert!(!html.contains("<script>"));
    assert!(!html.contains("alert"));
    assert!(html.contains("<h1>Safe Content</h1>"));
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_render_html_with_gfm_tables() {
    let markdown = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
    let mut processor_options = ProcessorOptions::new();
    processor_options.set_gfm(true);
    let html_options = HtmlOptions::new();

    let html = render_html(markdown, &processor_options, &html_options);

    assert!(html.contains("<table>"));
    assert!(html.contains("<thead>"));
    assert!(html.contains("<tbody>"));
    assert!(html.contains("<th>Header 1</th>"));
    assert!(html.contains("<td>Cell 1</td>"));
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_render_html_code_blocks() {
    let markdown = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    let processor_options = ProcessorOptions::new();
    let html_options = HtmlOptions::new();

    let html = render_html(markdown, &processor_options, &html_options);

    assert!(html.contains(r#"<pre><code class="language-rust">"#));
    assert!(html.contains("fn main()"));
    assert!(html.contains(r#"println!("Hello")"#));
    assert!(html.contains("</code></pre>"));
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_render_html_performance() {
    // 50KB document
    let markdown = "# Heading\n\n**Bold** and *italic* text.\n\n".repeat(2000);
    let processor_options = ProcessorOptions::new();
    let html_options = HtmlOptions::new();

    let start = js_sys::Date::now();
    let _html = render_html(&markdown, &processor_options, &html_options);
    let duration = js_sys::Date::now() - start;

    // Should render in <20ms total (parse + render)
    assert!(
        duration < 20.0,
        "Render took {}ms, expected <20ms",
        duration
    );
}

#[wasm_bindgen_test]
#[ignore = "Implementation not complete - will fail"]
fn test_render_html_with_dangerous_html() {
    let markdown = "<div onclick='alert()'>Click me</div>";
    let processor_options = ProcessorOptions::new();
    let mut html_options = HtmlOptions::new();
    html_options.set_allow_dangerous_html(true);
    html_options.set_sanitize(false);

    let html = render_html(markdown, &processor_options, &html_options);

    assert!(html.contains("onclick"));
    assert!(html.contains("<div"));
}
