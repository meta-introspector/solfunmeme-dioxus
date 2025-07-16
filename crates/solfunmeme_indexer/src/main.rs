use anyhow::{Result, anyhow};
use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_search_tantivy::SearchIndex;
use clap::Parser;
use std::io::{self, BufRead};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "solfunmeme_indexer_cli")]
#[command(about = "Indexes CodeChunk JSON objects from stdin into a Tantivy index.", long_about = None)]
struct Cli {
    #[arg(long, help = "Path to the Tantivy index directory.")]
    index_path: PathBuf,
    #[arg(long, help = "If specified, deletes the existing index before re-indexing.")]
    overwrite: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let index_path = &cli.index_path;

    if cli.overwrite && index_path.exists() {
        eprintln!("[INFO] Overwriting existing index at: {}", index_path.display());
        std::fs::remove_dir_all(index_path)?;
    }

    let mut search_index = SearchIndex::new(index_path)?;

    eprintln!("[INFO] Reading CodeChunks from stdin...");
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut processed_count = 0;
    for line in reader.lines() {
        let line = line?;
        let chunk: CodeChunk = serde_json::from_str(&line)?;
        search_index.add_chunk(&chunk)?;

        processed_count += 1;
        if processed_count % 100 == 0 {
            eprintln!("[INFO] Indexed {} chunks so far...", processed_count);
        }
    }

    search_index.commit()?;
    eprintln!("[INFO] Finished indexing all {} chunks.", processed_count);

    Ok(())
}