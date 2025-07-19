use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "prepare_sources")]
#[command(about = "Prepares source code by reading and chunking it into JSON format.", long_about = None)]
pub struct Cli {
    #[arg(help = "Path to the directory or file to process. Defaults to current directory.")]
    pub path: Option<PathBuf>,
    #[arg(long, help = "Limit processing to N files/chunks.")]
    pub limit: Option<usize>,
    #[arg(long, help = "Output file path for JSON chunks. If not specified, outputs to stdout.")]
    pub output: Option<PathBuf>,
}
