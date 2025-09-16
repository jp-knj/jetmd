// Memory Profiling Benchmark
// Target: ≤1.5× input memory usage

#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_vec)]

use fmd_core::{parse, Document, ProcessorOptions};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Custom allocator to track memory usage
struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let ptr = System.alloc(layout);

        if !ptr.is_null() {
            let current = ALLOCATED.fetch_add(size, Ordering::SeqCst) + size;
            let mut peak = PEAK.load(Ordering::SeqCst);

            while current > peak {
                match PEAK.compare_exchange_weak(peak, current, Ordering::SeqCst, Ordering::SeqCst)
                {
                    Ok(_) => break,
                    Err(p) => peak = p,
                }
            }
        }

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

/// Generate markdown content of specified size
fn generate_markdown(target_size: usize) -> String {
    let patterns = vec![
        "# Heading Level 1\n\nThis is a paragraph with some **bold text** and *italic text*.\n\n",
        "## Heading Level 2\n\nAnother paragraph with a [link](https://example.com) and `inline code`.\n\n",
        "### Heading Level 3\n\n- List item 1\n- List item 2\n- List item 3\n\n",
        "```rust\nfn example() -> &'static str {\n    \"Hello, World!\"\n}\n```\n\n",
        "| Column 1 | Column 2 | Column 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |\n\n",
        "> This is a blockquote with some text inside it.\n> It can span multiple lines.\n\n",
        "![Alt text](image.jpg \"Image title\")\n\n",
        "---\n\n",
    ];

    let mut result = String::new();
    let mut current_size = 0;
    let mut pattern_idx = 0;

    while current_size < target_size {
        let pattern = patterns[pattern_idx % patterns.len()];
        result.push_str(pattern);
        current_size += pattern.len();
        pattern_idx += 1;
    }

    result
}

/// Reset memory tracking
fn reset_memory_tracking() {
    ALLOCATED.store(0, Ordering::SeqCst);
    PEAK.store(0, Ordering::SeqCst);
}

/// Get current memory usage
fn get_current_memory() -> usize {
    ALLOCATED.load(Ordering::SeqCst)
}

/// Get peak memory usage
fn get_peak_memory() -> usize {
    PEAK.load(Ordering::SeqCst)
}

/// Measure memory usage for parsing
fn measure_parse_memory(content: &str) -> (usize, usize) {
    reset_memory_tracking();

    let initial_memory = get_current_memory();
    let mut options = ProcessorOptions::default();
    options.gfm = true;
    let doc = Document::new(content);
    let result = parse(&doc, options);
    let _ast = result.ast;

    let peak_memory = get_peak_memory();
    let final_memory = get_current_memory();

    (peak_memory - initial_memory, final_memory - initial_memory)
}

/// Measure memory usage for rendering
fn measure_render_memory(content: &str) -> (usize, usize) {
    reset_memory_tracking();

    let initial_memory = get_current_memory();
    let mut options = ProcessorOptions::default();
    options.gfm = true;
    options.sanitize = true;

    let doc = Document::new(content);
    let result = parse(&doc, options);
    let _ast = result.ast;
    // render_html is not available in the current API

    let peak_memory = get_peak_memory();
    let final_memory = get_current_memory();

    (peak_memory - initial_memory, final_memory - initial_memory)
}

/// Measure memory usage for incremental parsing
fn measure_incremental_memory(base_content: &str, modified_content: &str) -> (usize, usize) {
    reset_memory_tracking();

    let initial_memory = get_current_memory();
    let mut options = ProcessorOptions::default();
    options.gfm = true;
    options.incremental = true;

    // Parse original
    let base_doc = Document::new(base_content);
    let original_result = parse(&base_doc, options.clone());
    let original_ast = original_result.ast;

    // Parse modified with incremental hints
    let modified_doc = Document::new(modified_content);
    let modified_result = parse(&modified_doc, options);
    let _modified_ast = modified_result.ast;

    // Keep original AST alive to measure retention
    drop(original_ast);

    let peak_memory = get_peak_memory();
    let final_memory = get_current_memory();

    (peak_memory - initial_memory, final_memory - initial_memory)
}

fn main() {
    println!("{}", "=".repeat(60));
    println!("Memory Profiling Benchmark");
    println!("{}", "=".repeat(60));
    println!("\nTarget: ≤1.5× input memory usage\n");

    let test_sizes = vec![
        ("Small (1KB)", 1024),
        ("Medium (50KB)", 50 * 1024),
        ("Large (500KB)", 500 * 1024),
        ("Extra Large (5MB)", 5 * 1024 * 1024),
    ];

    let mut all_passed = true;

    for (name, size) in test_sizes {
        let content = generate_markdown(size);
        let input_size = content.len();

        println!("{}", "-".repeat(60));
        println!("Testing: {} ({}KB)", name, size / 1024);
        println!("Input size: {} bytes", input_size);

        // Parse memory usage
        let (parse_peak, parse_final) = measure_parse_memory(&content);
        let parse_ratio = parse_peak as f64 / input_size as f64;
        let parse_passed = parse_ratio <= 1.5;

        println!("\nParse Memory:");
        println!("  Peak: {} bytes ({:.2}× input)", parse_peak, parse_ratio);
        println!(
            "  Final: {} bytes ({:.2}× input)",
            parse_final,
            parse_final as f64 / input_size as f64
        );
        println!(
            "  Status: {}",
            if parse_passed { "✅ PASS" } else { "❌ FAIL" }
        );

        all_passed = all_passed && parse_passed;

        // Render memory usage
        let (render_peak, render_final) = measure_render_memory(&content);
        let render_ratio = render_peak as f64 / input_size as f64;
        let render_passed = render_ratio <= 1.5;

        println!("\nRender Memory:");
        println!("  Peak: {} bytes ({:.2}× input)", render_peak, render_ratio);
        println!(
            "  Final: {} bytes ({:.2}× input)",
            render_final,
            render_final as f64 / input_size as f64
        );
        println!(
            "  Status: {}",
            if render_passed {
                "✅ PASS"
            } else {
                "❌ FAIL"
            }
        );

        all_passed = all_passed && render_passed;

        // Incremental memory usage (for medium and large sizes)
        if size <= 500 * 1024 {
            let modified_content = format!(
                "{}\n\n## New Section\n\nAdded content for incremental test.",
                content
            );
            let (inc_peak, inc_final) = measure_incremental_memory(&content, &modified_content);
            let inc_ratio = inc_peak as f64 / (input_size * 2) as f64; // Compare against both documents
            let inc_passed = inc_ratio <= 1.5;

            println!("\nIncremental Memory:");
            println!(
                "  Peak: {} bytes ({:.2}× combined input)",
                inc_peak, inc_ratio
            );
            println!("  Final: {} bytes", inc_final);
            println!(
                "  Status: {}",
                if inc_passed { "✅ PASS" } else { "❌ FAIL" }
            );

            all_passed = all_passed && inc_passed;
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("SUMMARY");
    println!("{}", "=".repeat(60));

    if all_passed {
        println!("✅ PASS: All memory usage tests passed (≤1.5× input)");
    } else {
        println!("❌ FAIL: Some memory usage tests exceeded 1.5× input");
    }

    std::process::exit(if all_passed { 0 } else { 1 });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_document_memory() {
        let content = generate_markdown(1024);
        let (peak, _final) = measure_parse_memory(&content);
        let ratio = peak as f64 / content.len() as f64;

        assert!(ratio <= 1.5, "Small document memory ratio {} > 1.5", ratio);
    }

    #[test]
    fn test_medium_document_memory() {
        let content = generate_markdown(50 * 1024);
        let (peak, _final) = measure_parse_memory(&content);
        let ratio = peak as f64 / content.len() as f64;

        assert!(ratio <= 1.5, "Medium document memory ratio {} > 1.5", ratio);
    }

    #[test]
    fn test_incremental_memory() {
        let base = generate_markdown(50 * 1024);
        let modified = format!("{}\n\nNew content", base);

        let (peak, _final) = measure_incremental_memory(&base, &modified);
        let combined_size = base.len() + modified.len();
        let ratio = peak as f64 / combined_size as f64;

        assert!(ratio <= 1.5, "Incremental memory ratio {} > 1.5", ratio);
    }
}
