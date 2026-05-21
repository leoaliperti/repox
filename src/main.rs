use anyhow::{Context, Result};
use arboard::Clipboard;
use clap::Parser;
use content_inspector::{inspect, ContentType};
use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "repox",
    author,
    version,
    about = "⚡ Pack your codebase into a single AI-ready context file or clipboard ⚡"
)]
struct Args {
    /// The directory to analyze
    #[arg(default_value = ".")]
    path: PathBuf,

    /// The output file name
    #[arg(short, long, default_value = "ai_project_context.md")]
    output: String,

    /// Maximum size of a single file in KB to include
    #[arg(short, long, default_value_t = 500)]
    max_size_kb: u64,

    /// Copy the result directly to the clipboard instead of saving a file
    #[arg(short, long)]
    clipboard: bool,

    /// Custom files/folders to exclude (comma-separated, e.g., package-lock.json,cargo.lock)
    #[arg(short, long, value_delimiter = ',')]
    exclude: Vec<String>,

    /// Ignore rules from .gitignore files
    #[arg(long)]
    no_gitignore: bool,

    /// Keep empty lines in the source code (disabled by default to save tokens)
    #[arg(long)]
    keep_empty_lines: bool,
}

/// Detects extension for Markdown code block syntax highlighting
fn get_syntax_highlight(path: &Path) -> &str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("rs") => "rust",
        Some("js") => "javascript",
        Some("ts") => "typescript",
        Some("tsx") => "tsx",
        Some("jsx") => "jsx",
        Some("py") => "python",
        Some("html") => "html",
        Some("css") => "css",
        Some("json") => "json",
        Some("md") => "markdown",
        Some("toml") => "toml",
        Some("yaml") | Some("yml") => "yaml",
        Some("sh") | Some("bash") => "bash",
        _ => "",
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let start_dir = fs::canonicalize(&args.path)
        .context("Error: Unable to find or access the specified directory")?;

    println!(">> Analyzing directory: {}", start_dir.display());

    let max_bytes = args.max_size_kb * 1024;
    let output_file_name = &args.output;

    // Configure the directory walker (ripgrep algorithm)
    let mut walker_builder = WalkBuilder::new(&start_dir);
    walker_builder
        .git_ignore(!args.no_gitignore)
        .git_global(!args.no_gitignore)
        .git_exclude(!args.no_gitignore)
        .hidden(false);

    let walker = walker_builder.build();
    let mut valid_paths = Vec::new();

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if name == output_file_name {
            continue;
        }

        if args.exclude.iter().any(|ex| name.contains(ex) || path.to_string_lossy().contains(ex)) {
            continue;
        }

        if path.is_file() {
            if let Ok(metadata) = fs::metadata(path) {
                if metadata.len() > max_bytes {
                    continue;
                }
            }
            valid_paths.push(path.to_path_buf());
        } else if path.is_dir() && path != start_dir {
            valid_paths.push(path.to_path_buf());
        }
    }

    valid_paths.sort();

    let mut final_output = String::new();

    // 1. Professional Tree Structure Generation (No emojis, highly token-efficient)
    final_output.push_str("# PROJECT STRUCTURE\n\n```text\n");
    final_output.push_str(&format!("{}/\n", start_dir.file_name().unwrap_or_default().to_string_lossy()));
    
    for path in &valid_paths {
        let rel_path = path.strip_prefix(&start_dir)?;
        let depth = rel_path.components().count();
        if depth == 0 { continue; }
        
        let indent = "  ".repeat(depth);
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        let trailing_slash = if path.is_dir() { "/" } else { "" };
        
        final_output.push_str(&format!("{}{}{}\n", indent, name, trailing_slash));
    }
    final_output.push_str("```\n\n");

    // 2. Processing File Contents
    final_output.push_str("# FILE CONTENTS\n");

    let mut files_count = 0;

    for path in &valid_paths {
        if path.is_dir() { continue; }

        // Binary check
        if let Ok(file) = File::open(path) {
            let mut reader = BufReader::new(file);
            let buffer = reader.fill_buf().unwrap_or(&[]);
            if inspect(buffer) == ContentType::BINARY {
                continue; 
            }
        }

        if let Ok(content) = fs::read_to_string(path) {
            let rel_path = path.strip_prefix(&start_dir)?;
            let syntax = get_syntax_highlight(path);
            
            // Token Optimization: Remove empty/whitespace-only lines by default
            let processed_content = if args.keep_empty_lines {
                content
            } else {
                content
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .collect::<Vec<&str>>()
                    .join("\n")
            };

            final_output.push_str(&format!("\n## File: `{}`\n", rel_path.display()));
            final_output.push_str(&format!("```{}\n", syntax));
            final_output.push_str(&processed_content);
            if !processed_content.ends_with('\n') {
                final_output.push('\n');
            }
            final_output.push_str("```\n");
            
            files_count += 1;
        }
    }

    // Token & Character calculation happens AFTER processing (accurate stats)
    let total_chars = final_output.len();
    let estimated_tokens = total_chars / 4; // Standard approximation for source code

    // Output delivery
    if args.clipboard {
        match Clipboard::new() {
            Ok(mut ctx) => {
                // AGGIUNTA LA '&' QUI SOTTO: prestiamo la variabile senza farla consumare
                if ctx.set_text(&final_output).is_ok() {
                    println!(">> Success: Context copied to clipboard!");
                } else {
                    println!(">> Warning: Clipboard failed. Falling back to file output.");
                    // Anche qui usiamo '&' per coerenza
                    fs::write(start_dir.join(output_file_name), &final_output)?;
                }
            }
            Err(_) => {
                println!(">> Warning: Clipboard unavailable. Falling back to file output.");
                fs::write(start_dir.join(output_file_name), &final_output)?;
            }
        }
    } else {
        let output_path = start_dir.join(output_file_name);
        fs::write(&output_path, &final_output)?;
        println!(">> Success: Context saved to {}", output_path.display());
    }

    // Clean, professional dashboard summary in English
    println!("\n========================================");
    println!("          AI CONTEXT GENERATED          ");
    println!("========================================");
    println!("  Files Processed:   {}", files_count);
    println!("  Total Characters:  {}", total_chars);
    println!("  Estimated Tokens:  ~{}", estimated_tokens);
    println!("  Empty Lines:       Optimized (Removed)");
    println!("========================================");

    Ok(())
}