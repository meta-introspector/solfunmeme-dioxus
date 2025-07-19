use clap::Subcommand;
use solfunmeme_serde_utils::serde::{Deserialize, Serialize};

#[derive(Subcommand, Debug, Serialize, Deserialize)]
pub enum TaskCommands {
    /// List all available tasks
    List,
    
    /// Run a specific task
    Run {
        /// Task name to run
        task_name: String,
    },
    
    /// Generate task report
    Report {
        /// Output format
        #[arg(short, long, default_value = "json")]
        format: String,
    },
}
