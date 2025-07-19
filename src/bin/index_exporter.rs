use std::path::PathBuf;
use std::error::Error;
use solfunmeme_core_utils::clap::{Parser, arg, command};

use index_exporter_lib::exporter::IndexExporter;
use index_exporter_lib::formats::json_exporter::export_to_json;
use index_exporter_lib::formats::csv_exporter::export_to_csv;
use index_exporter_lib::formats::markdown_exporter::export_to_markdown;
use index_exporter_lib::formats::stats_exporter::export_stats;
use solfunmeme_messages;

#[derive(Parser)]
#[command(name = solfunmeme_messages::EXPORTER_CLI_NAME)]
#[command(about = solfunmeme_messages::EXPORTER_CLI_ABOUT)]
struct Cli {
    /// Path to the Tantivy index directory
    #[arg(short, long, default_value = "tmp/tantivy_index")]
    index_path: PathBuf,
    
    /// Output format: json, csv, markdown, or stats
    #[arg(short, long, default_value = "json")]
    format: String,
    
    /// Output file path (defaults to stdout if not specified)
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Maximum number of documents to export (0 = all)
    #[arg(short, long, default_value = "0")]
    limit: usize,
    
    /// Filter by emoji
    #[arg(long)]
    emoji: Option<String>,
    
    /// Filter by path pattern
    #[arg(long)]
    path_pattern: Option<String>,
    
    /// Show only statistics
    #[arg(long)]
    stats_only: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    println!("🔍 {}", solfunmeme_messages::EXPORTER_CLI_NAME);
    println!("=========================");
    
    if !cli.index_path.exists() {
        eprintln!("{}: {:?}", solfunmeme_messages::INDEX_PATH_NOT_EXIST_ERROR, cli.index_path);
        return Ok(());
    }
    
    let exporter = IndexExporter::new(&cli.index_path)?;
    
    if cli.stats_only {
        println!("{}", solfunmeme_messages::GENERATING_INDEX_STATS);
        let stats = exporter.get_stats()?;
        export_stats(&stats, &cli.output)?;
    } else {
        println!("{}", solfunmeme_messages::EXPORTING_DOCUMENTS);
        let documents = exporter.export_all_documents(cli.limit)?;
        
        match cli.format.as_str() {
            "json" => export_to_json(&documents, &cli.output)?,
            "csv" => export_to_csv(&documents, &cli.output)?,
            "markdown" => export_to_markdown(&documents, cli.output.as_ref())?,
            _ => {
                eprintln!("{}: {}", solfunmeme_messages::UNSUPPORTED_FORMAT_ERROR, cli.format);
                return Ok(());
            }
        }
    }
    
    println!("{}", solfunmeme_messages::EXPORT_COMPLETED_SUCCESS);
    
    Ok(())
}