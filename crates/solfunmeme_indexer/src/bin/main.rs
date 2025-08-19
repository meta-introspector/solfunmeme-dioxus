use clap::{Parser, Subcommand};
use solfunmeme_indexer::{index_directory, report_top_entries};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Indexes a directory
    Index {
        /// The path to the directory to index
        #[arg(value_parser = clap::value_parser!(PathBuf))]
        path: PathBuf,

        /// The path to the index
        #[arg(value_parser = clap::value_parser!(PathBuf))]
        index_path: PathBuf,
    },
    /// Generates a top N report from an index
    Report {
        /// The path to the index
        #[arg(value_parser = clap::value_parser!(PathBuf))]
        index_path: PathBuf,

        /// The type of report to generate (e.g., "emoji", "content")
        #[clap(short, long)]
        report_type: String,

        /// The number of entries to show in the report
        #[clap(short, long, default_value_t = 10)]
        limit: usize,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Index { path, index_path } => {
            index_directory(path, index_path)?;
        }
        Commands::Report { index_path, report_type, limit } => {
            report_top_entries(index_path, report_type, *limit)?;
        }
    }

    Ok(())
}
