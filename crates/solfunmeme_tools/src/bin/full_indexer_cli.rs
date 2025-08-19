use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use solfunmeme_indexer::index_directory;
use tempfile::TempDir;

#[derive(Parser)]
#[command(name = "full_indexer")]
#[command(about = "Indexes specified directories into the Tantivy search index.", long_about = None)]
struct Cli {
    #[arg(help = "Paths to directories to index")]
    directories: Vec<PathBuf>,
    #[arg(long, help = "Overwrite existing index if it exists.")]
    overwrite: bool,
    #[arg(long, help = "Create index in a temporary directory (sandbox mode).")]
    sandbox: bool,
    #[arg(long, help = "Enable RUST_BACKTRACE=full for subprocesses.")]
    debug_backtrace: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.directories.is_empty() {
        println!("Usage: full_indexer <directory1> [directory2 ...] [--overwrite] [--sandbox] [--debug-backtrace]");
        println!("Example: full_indexer crates/ vendor/ --overwrite --debug-backtrace");
        return Ok(());
    }

    let mut temp_dir_holder: Option<TempDir> = None;
    let index_path = if cli.sandbox {
        let temp_dir = TempDir::new()?;
        println!("Creating sandbox index at: {}", temp_dir.path().display());
        let path = temp_dir.path().to_path_buf();
        temp_dir_holder = Some(temp_dir);
        path
    } else {
        PathBuf::from("codebase_index")
    };

    if cli.overwrite && index_path.exists() {
        println!("Overwriting existing index at: {}", index_path.display());
        std::fs::remove_dir_all(&index_path)?;
    }

    for dir_path in cli.directories {
        println!("Indexing directory: {}", dir_path.display());
        index_directory(&dir_path, &index_path, cli.debug_backtrace)?;
        println!("Finished indexing: {}", dir_path.display());
    }

    println!("Indexing complete.");
    Ok(())
}
