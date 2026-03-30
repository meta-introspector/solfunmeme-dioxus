use std::path::PathBuf;
use clap::{Parser, Subcommand};
use solfunmeme_tools::chat_processing::{ChunkProcessor, find_files_with_pattern};
use solfunmeme_tools::utils;
use rayon::prelude::*;

#[derive(Parser)]
#[command(name = "chunk-processor")]
#[command(about = "Split documents into organized chunks with external files")]
struct ChunkProcessorCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a single file into chunks
    File {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output directory for processed chunks
        #[arg(short, long, default_value = "processed_docs")]
        output: PathBuf,
    },
    
    /// Process all files in a directory
    Directory {
        /// Input directory path
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output directory for processed chunks
        #[arg(short, long, default_value = "processed_docs")]
        output: PathBuf,
        
        /// File patterns to include (e.g., "*.md", "*.txt")
        #[arg(short, long, default_value = "*.md")]
        pattern: String,
        
        /// Number of threads to use (default: auto-detect)
        #[arg(short, long)]
        threads: Option<usize>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = ChunkProcessorCli::parse();
    
    match cli.command {
        Commands::File { input, output } => {
            process_single_file(&input, &output)?;
        }
        Commands::Directory { input, output, pattern, threads } => {
            process_directory(&input, &output, &pattern, threads)?;
        }
    }
    
    Ok(())
}

fn process_single_file(input_path: &PathBuf, output_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("[INFO] Processing file: {}", input_path.display());
    
    // Normalize paths
    let input_path = utils::normalize_path(input_path);
    let output_dir = utils::normalize_path(output_dir);
    
    // Read file content
    let content = std::fs::read_to_string(&input_path)?;
    
    // Create chunk processor
    let processor = ChunkProcessor::new(output_dir);
    
    // Process the document
    processor.process_document(&input_path, &content)?;
    
    println!("[SUCCESS] File processed successfully!");
    Ok(())
}

fn process_directory(input_dir: &PathBuf, output_dir: &PathBuf, pattern: &str, threads: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
    println!("[INFO] Processing directory: {}", input_dir.display());
    println!("[INFO] Pattern: {}", pattern);
    
    // Set thread pool size if specified
    if let Some(num_threads) = threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .unwrap();
        println!("[INFO] Using {} threads", num_threads);
    } else {
        println!("[INFO] Using auto-detected thread count");
    }
    
    // Normalize paths
    let input_dir = utils::normalize_path(input_dir);
    let output_dir = utils::normalize_path(output_dir);
    
    // Find files matching pattern
    let files = find_files_with_pattern(&input_dir, pattern, "md", None);
    
    if files.is_empty() {
        println!("[WARNING] No files found matching pattern: {}", pattern);
        return Ok(());
    }
    
    println!("[INFO] Found {} files to process", files.len());
    
    // Process files in parallel
    let results: Vec<_> = files.par_iter().map(|file_path| {
        println!("[INFO] Processing: {}", file_path.display());
        
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                let processor = ChunkProcessor::new(output_dir.clone());
                match processor.process_document(file_path, &content) {
                    Ok(_) => {
                        println!("[SUCCESS] Processed: {}", file_path.display());
                        Ok(())
                    }
                    Err(e) => {
                        println!("[ERROR] Failed to process {}: {}", file_path.display(), e);
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                println!("[ERROR] Failed to read {}: {}", file_path.display(), e);
                Err(e.to_string())
            }
        }
    }).collect();
    
    // Count successes and failures
    let successes = results.iter().filter(|r| r.is_ok()).count();
    let failures = results.iter().filter(|r| r.is_err()).count();
    
    println!("[SUCCESS] Directory processing completed!");
    println!("[INFO] Successfully processed: {} files", successes);
    if failures > 0 {
        println!("[WARNING] Failed to process: {} files", failures);
    }
    
    Ok(())
} 