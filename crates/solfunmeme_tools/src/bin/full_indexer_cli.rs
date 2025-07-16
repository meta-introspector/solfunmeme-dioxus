use anyhow::{Result, anyhow};
use clap::Parser;
use std::path::PathBuf;
use std::process::{Command, Stdio};
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

    // Run solfunmeme_indexer_cli with --overwrite if cli.overwrite is true
    let mut indexer_command = Command::new("cargo");
    indexer_command.args([
            "run",
            "-p",
            "solfunmeme_indexer",
            "--bin",
            "solfunmeme_indexer_cli",
            "--",
            "--index-path",
            index_path.to_str().ok_or_else(|| anyhow!("Invalid index path"))?,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    if cli.overwrite {
        indexer_command.arg("--overwrite");
    }

    let mut indexer_child = indexer_command.spawn()?;
    let mut indexer_stdin = indexer_child.stdin.take().ok_or_else(|| anyhow!("Failed to open stdin for indexer"))?;

    for dir_path in cli.directories {
        println!("Processing directory with prepare_sources: {}", dir_path.display());

        let mut prepare_sources_command = Command::new("cargo");
        prepare_sources_command.args([
                "run",
                "-p",
                "prepare_sources",
                "--bin",
                "prepare_sources",
                "--",
                dir_path.to_str().ok_or_else(|| anyhow!("Invalid directory path"))?,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());

        if cli.debug_backtrace {
            prepare_sources_command.env("RUST_BACKTRACE", "full");
        }

        let mut prepare_sources_child = prepare_sources_command.spawn()?;
        let prepare_sources_stdout = prepare_sources_child.stdout.take().ok_or_else(|| anyhow!("Failed to open stdout for prepare_sources"))?;

        // Pipe stdout of prepare_sources to stdin of solfunmeme_indexer_cli
        std::io::copy(&mut prepare_sources_stdout.lock(), &mut indexer_stdin)?;

        let status = prepare_sources_child.wait()?;
        if !status.success() {
            return Err(anyhow!("prepare_sources failed for directory {}: {:?}", dir_path.display(), status.code()));
        }
    }

    // Close stdin for the indexer to signal end of input
    drop(indexer_stdin);

    let indexer_status = indexer_child.wait()?;
    if !indexer_status.success() {
        return Err(anyhow!("solfunmeme_indexer_cli failed: {:?}", indexer_status.code()));
    }

    println!("Indexing complete.");
    Ok(())
}
