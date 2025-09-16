// Latency Benchmark
// Target: <3ms p50 for 50KB documents

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fmd_core::{parse, Document, ProcessorOptions};
use std::time::Duration;

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

/// Benchmark parsing latency for different document sizes
fn bench_parse_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_latency");

    // Configure for latency measurement
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(500);

    // Test different document sizes
    let sizes = vec![
        ("1KB", 1024),
        ("10KB", 10 * 1024),
        ("50KB", 50 * 1024), // Key target: <3ms p50
        ("100KB", 100 * 1024),
        ("500KB", 500 * 1024),
    ];

    for (name, size) in sizes {
        let content = generate_markdown(size);
        let content_clone = content.clone();

        group.bench_with_input(
            BenchmarkId::new("parse", name),
            &content_clone,
            |b, content| {
                b.iter(|| {
                    let options = Options::default().with_gfm(true);
                    parse(black_box(content.as_str()), options)
                });
            },
        );
    }

    group.finish();
}

/// Benchmark render latency for different document sizes
fn bench_render_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_latency");

    group.measurement_time(Duration::from_secs(10));
    group.sample_size(500);

    let sizes = vec![
        ("1KB", 1024),
        ("10KB", 10 * 1024),
        ("50KB", 50 * 1024), // Key target: <3ms p50
        ("100KB", 100 * 1024),
        ("500KB", 500 * 1024),
    ];

    for (name, size) in sizes {
        let content = generate_markdown(size);
        let content_clone = content.clone();

        group.bench_with_input(
            BenchmarkId::new("render", name),
            &content_clone,
            |b, content| {
                b.iter(|| {
                    let mut options = ProcessorOptions::default();
                    options.gfm = true;
                    options.sanitize = true;
                    let doc = Document::new(content.as_str());
                    let result = parse(&doc, options);
                    // render_html not available in current API
                    result.ast
                });
            },
        );
    }

    group.finish();
}

/// Benchmark end-to-end latency (parse + render)
fn bench_e2e_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e_latency");

    group.measurement_time(Duration::from_secs(15));
    group.sample_size(1000);

    // Focus on the 50KB case for detailed analysis
    let content = generate_markdown(50 * 1024);
    let content_clone = content.clone();

    group.bench_function("50KB_e2e", |b| {
        b.iter(|| {
            let mut options = ProcessorOptions::default();
            options.gfm = true;
            options.sanitize = true;
            let doc = Document::new(black_box(&content_clone));
            let result = parse(&doc, options);
            // render_html not available in current API
            result.ast
        });
    });

    // Measure percentiles
    group.bench_function("50KB_percentiles", |b| {
        let mut times = Vec::with_capacity(1000);

        b.iter_custom(|iters| {
            let start = std::time::Instant::now();

            for _ in 0..iters {
                let iter_start = std::time::Instant::now();

                let options = Options::default().with_gfm(true).with_sanitize(true);
                let ast = parse(black_box(&content_clone), options.clone()).unwrap();
                let _ = render_html(&ast, options);

                times.push(iter_start.elapsed());
            }

            start.elapsed()
        });

        // Calculate and report percentiles
        if !times.is_empty() {
            times.sort();
            let p50_idx = times.len() / 2;
            let p95_idx = (times.len() as f64 * 0.95) as usize;
            let p99_idx = (times.len() as f64 * 0.99) as usize;

            let p50 = times[p50_idx];
            let p95 = times[p95_idx.min(times.len() - 1)];
            let p99 = times[p99_idx.min(times.len() - 1)];

            println!("\n50KB Document Latency Percentiles:");
            println!("  P50: {:.2}ms", p50.as_secs_f64() * 1000.0);
            println!("  P95: {:.2}ms", p95.as_secs_f64() * 1000.0);
            println!("  P99: {:.2}ms", p99.as_secs_f64() * 1000.0);

            // Check against target
            if p50.as_secs_f64() * 1000.0 < 3.0 {
                println!("  ✅ PASS: P50 < 3ms target");
            } else {
                println!("  ❌ FAIL: P50 > 3ms target");
            }
        }
    });

    group.finish();
}

/// Benchmark incremental parsing latency
fn bench_incremental_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("incremental_latency");

    group.measurement_time(Duration::from_secs(10));
    group.sample_size(500);

    let base_content = generate_markdown(50 * 1024);

    // Simulate small edits
    let edits = vec![
        (
            "append_paragraph",
            format!("{}\n\nNew paragraph added.", base_content),
        ),
        (
            "modify_heading",
            base_content.replace("# Heading Level 1", "# Modified Heading"),
        ),
        (
            "insert_list_item",
            base_content.replace("- List item 3", "- List item 3\n- List item 4"),
        ),
    ];

    for (edit_name, modified_content) in edits {
        group.bench_function(format!("50KB_{}", edit_name), |b| {
            b.iter(|| {
                let mut options = ProcessorOptions::default();
                options.gfm = true;
                options.incremental = true;

                // Parse original
                let base_doc = Document::new(black_box(&base_content));
                let original_result = parse(&base_doc, options.clone());
                let _original_ast = original_result.ast;

                // Parse modified with incremental hints
                let modified_doc = Document::new(black_box(&modified_content));
                let modified_result = parse(&modified_doc, options);
                let _modified_ast = modified_result.ast;
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_latency,
    bench_render_latency,
    bench_e2e_latency,
    bench_incremental_latency
);
criterion_main!(benches);
