use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

#[derive(Parser)]
#[command(name = "plan_cli")]
#[command(about = "Estimates indexing cost for specified directories, with skip rules.", long_about = None)]
struct Cli {
    #[arg(help = "Paths to directories to analyze")]
    directories: Vec<PathBuf>,

    #[arg(long, help = "Comma-separated list of file extensions to skip (e.g. png,jpg,zip)")]
    skip_ext: Option<String>,

    #[arg(long, help = "Maximum file size in bytes to index (files larger will be skipped)")]
    max_size: Option<u64>,

    #[arg(long, help = "Output config file (default: index_config.json)")]
    config: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexConfig {
    skip_extensions: Vec<String>,
    max_size: Option<u64>,
    skipped_files: Vec<String>,
    indexed_files: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.directories.is_empty() {
        println!("Usage: plan_cli <directory1> [directory2 ...] [--skip-ext ext1,ext2 --max-size N]");
        println!("Example: plan_cli vendor/ --skip-ext png,jpg,zip --max-size 2000000");
        return Ok(());
    }

    let skip_exts: HashSet<String> = cli
        .skip_ext
        .as_deref()
        .unwrap_or("")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_lowercase())
        .collect();
    let max_size = cli.max_size;
    let mut total_files = 0;
    let mut total_lines = 0;
    let mut total_chunks_estimated = 0;
    let chunk_size_lines = 50; // Based on prepare_sources chunking
    let mut skipped_files = Vec::new();
    let mut indexed_files = Vec::new();

    println!("\n--- Indexing Cost Estimation (with skip rules) ---");
    println!("Skip extensions: {:?}", skip_exts);
    println!("Max file size: {:?}", max_size);

    for dir_path in cli.directories {
        println!("Analyzing directory: {}", dir_path.display());
        for entry in WalkDir::new(&dir_path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);

                // Skip if extension is in skip_exts
                if !skip_exts.is_empty() && skip_exts.contains(&ext) {
                    skipped_files.push(file_path.display().to_string());
                    continue;
                }
                // Skip if file is too large
                if let Some(max) = max_size {
                    if file_size > max {
                        skipped_files.push(file_path.display().to_string());
                        continue;
                    }
                }
                // Only process text/code files for estimation
                if matches!(ext.as_str(), "rs" | "md" | "json" | "ttl" | "toml" | "txt" | "js" | "ts" | "tsx" | "py" | "go" | "java" | "c" | "cpp" | "h" | "hpp") {
                    total_files += 1;
                    match fs::read(&file_path) {
                        Ok(bytes) => {
                            let content = String::from_utf8_lossy(&bytes);
                            let lines_in_file = content.lines().count();
                            total_lines += lines_in_file;
                            total_chunks_estimated += (lines_in_file as f64 / chunk_size_lines as f64).ceil() as usize;
                            indexed_files.push(file_path.display().to_string());
                        },
                        Err(e) => {
                            eprintln!("Warning: Could not read file {} ({}). Skipping.", file_path.display(), e);
                            skipped_files.push(file_path.display().to_string());
                        }
                    }
                } else {
                    skipped_files.push(file_path.display().to_string());
                }
            }
        }
    }

    println!("\n--- Estimation Summary ---");
    println!("Total files to process: {}", total_files);
    println!("Total lines to process: {}", total_lines);
    println!("Estimated total chunks: {}", total_chunks_estimated);
    println!("Skipped files: {}", skipped_files.len());
    println!("Indexed files: {}", indexed_files.len());
    println!("\nNote: This is an estimation. Actual indexing time may vary based on content complexity and system performance.");

    // Output config file
    let config = IndexConfig {
        skip_extensions: skip_exts.iter().cloned().collect(),
        max_size,
        skipped_files: skipped_files.clone(),
        indexed_files: indexed_files.clone(),
    };
    let config_path = cli.config.unwrap_or_else(|| "index_config.json".to_string());
    let config_json = serde_json::to_string_pretty(&config)?;
    std::fs::write(&config_path, config_json)?;
    println!("\nIndex config written to: {}", config_path);

    Ok(())
}