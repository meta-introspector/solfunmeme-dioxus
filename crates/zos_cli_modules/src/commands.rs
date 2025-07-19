use clap::Subcommand;
use solfunmeme_serde_utils::serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::task_commands::TaskCommands;
use crate::chat_commands::ChatCommands;

#[derive(Subcommand, Debug, Serialize, Deserialize)]
pub enum Commands {
    /// Vendor dependencies and manage workspace
    Vendor {
        /// Path to vendor directory
        #[arg(short, long, default_value = "vendor")]
        path: PathBuf,
    },
    
    /// Index code and extract patterns
    Index {
        /// Source directory to index
        #[arg(short, long, default_value = "src")]
        source: PathBuf,
        
        /// Output directory for index
        #[arg(short, long, default_value = "tmp/tantivy_index")]
        output: PathBuf,
    },
    
    /// Find duplicate or similar code
    Dedup {
        /// Source directory to analyze
        #[arg(short, long, default_value = "src")]
        source: PathBuf,
        
        /// Minimum similarity threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.8")]
        threshold: f64,
    },
    
    /// Search indexed code semantically
    Search {
        /// Query string
        query: String,
        
        /// Number of results to return
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    
    /// Manage tasks and code analysis
    Tasks {
        /// Task operation
        #[command(subcommand)]
        operation: TaskCommands,
    },
    
    /// Analyze code quality and generate reports
    Analyze {
        /// Output format for reports
        #[arg(short, long, default_value = "json")]
        format: String,
        
        /// Output directory for reports
        #[arg(short, long, default_value = "reports")]
        output: PathBuf,
    },
    
    /// Index and analyze chat data for easter eggs
    Chat {
        /// Chat operation
        #[command(subcommand)]
        operation: ChatCommands,
    },
}
