// CLI for faster-md

use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use anyhow::{Context, Result};

use fmd_core::{Document, ProcessorOptions, parse};
use fmd_html::{HtmlOptions, render_html};

#[derive(Parser)]
#[command(name = "fmd")]
#[command(about = "High-performance Markdown processor", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Input file (stdin if not provided)
    #[arg(short, long)]
    input: Option<PathBuf>,
    
    /// Output file (stdout if not provided)
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Enable GitHub Flavored Markdown
    #[arg(long)]
    gfm: bool,
    
    /// Enable frontmatter parsing
    #[arg(long)]
    frontmatter: bool,
    
    /// Allow dangerous HTML (disabled by default)
    #[arg(long)]
    allow_dangerous_html: bool,
    
    /// Output format (html, ast, or events)
    #[arg(short, long, default_value = "html")]
    format: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse Markdown and output HTML
    Html {
        /// Input file
        input: Option<PathBuf>,
    },
    /// Parse Markdown and output AST as JSON
    Ast {
        /// Input file
        input: Option<PathBuf>,
    },
    /// Benchmark parsing performance
    Bench {
        /// Input file
        input: PathBuf,
        /// Number of iterations
        #[arg(short, long, default_value = "100")]
        iterations: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Html { input }) => {
            let content = read_input(input.as_ref())?;
            let html = process_to_html(&content, build_options(&cli))?;
            write_output(cli.output.as_ref(), &html)?;
        }
        Some(Commands::Ast { input }) => {
            let content = read_input(input.as_ref())?;
            let ast = process_to_ast(&content, build_options(&cli))?;
            write_output(cli.output.as_ref(), &ast)?;
        }
        Some(Commands::Bench { input, iterations }) => {
            let content = fs::read_to_string(&input)
                .with_context(|| format!("Failed to read file: {}", input.display()))?;
            benchmark(&content, *iterations)?;
        }
        None => {
            // Default: process input to specified format
            let content = read_input(cli.input.as_ref())?;
            let output = match cli.format.as_str() {
                "html" => process_to_html(&content, build_options(&cli))?,
                "ast" => process_to_ast(&content, build_options(&cli))?,
                _ => anyhow::bail!("Unknown format: {}", cli.format),
            };
            write_output(cli.output.as_ref(), &output)?;
        }
    }
    
    Ok(())
}

fn read_input(path: Option<&PathBuf>) -> Result<String> {
    match path {
        Some(p) => fs::read_to_string(p)
            .with_context(|| format!("Failed to read file: {}", p.display())),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn write_output(path: Option<&PathBuf>, content: &str) -> Result<()> {
    match path {
        Some(p) => fs::write(p, content)
            .with_context(|| format!("Failed to write file: {}", p.display())),
        None => {
            io::stdout().write_all(content.as_bytes())
                .context("Failed to write to stdout")?;
            Ok(())
        }
    }
}

fn build_options(cli: &Cli) -> ProcessorOptions {
    ProcessorOptions {
        gfm: cli.gfm,
        frontmatter: cli.frontmatter,
        allow_dangerous_html: cli.allow_dangerous_html,
        position: false,
        incremental: false,
        ..Default::default()
    }
}

fn process_to_html(content: &str, options: ProcessorOptions) -> Result<String> {
    let doc = Document::new(content);
    let parse_result = parse(&doc, options);
    
    if !parse_result.success {
        anyhow::bail!("Parse errors: {:?}", parse_result.errors);
    }
    
    let html_options = HtmlOptions {
        sanitize: !options.allow_dangerous_html,
        allow_dangerous_html: options.allow_dangerous_html,
        ..Default::default()
    };
    
    Ok(render_html(&parse_result.ast, html_options))
}

fn process_to_ast(content: &str, options: ProcessorOptions) -> Result<String> {
    let doc = Document::new(content);
    let parse_result = parse(&doc, options);
    
    if !parse_result.success {
        anyhow::bail!("Parse errors: {:?}", parse_result.errors);
    }
    
    serde_json::to_string_pretty(&parse_result.ast)
        .context("Failed to serialize AST")
}

fn benchmark(content: &str, iterations: usize) -> Result<()> {
    use std::time::Instant;
    
    let doc = Document::new(content);
    let options = ProcessorOptions::default();
    
    // Warmup
    for _ in 0..10 {
        let _ = parse(&doc, options);
    }
    
    // Benchmark
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parse(&doc, options);
    }
    let elapsed = start.elapsed();
    
    let per_iter = elapsed / iterations as u32;
    let throughput = (content.len() as f64 * iterations as f64) / elapsed.as_secs_f64();
    
    println!("Benchmark Results:");
    println!("  Iterations: {}", iterations);
    println!("  Total time: {:?}", elapsed);
    println!("  Per iteration: {:?}", per_iter);
    println!("  Throughput: {:.2} MB/s", throughput / 1_000_000.0);
    
    Ok(())
}