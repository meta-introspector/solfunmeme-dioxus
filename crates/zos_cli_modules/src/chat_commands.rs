use clap::Subcommand;
use solfunmeme_serde_utils::serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Subcommand, Debug, Serialize, Deserialize)]
pub enum ChatCommands {
    /// Index all processed chats
    Index {
        /// Input directory containing processed chats
        #[arg(short, long, default_value = "processed_chats")]
        input: PathBuf,
        
        /// Output directory for chat index
        #[arg(short, long, default_value = "tmp/chat_index")]
        output: PathBuf,
    },
    
    /// Generate vibe proof from chat data
    Vibe {
        /// Ontology to use for proof
        #[arg(short, long, default_value = "0,1,2,3,5,7")]
        ontology: String,
        
        /// Output file for proof
        #[arg(short, long, default_value = "vibe_proof.json")]
        output: PathBuf,
    },
    
    /// Search chats semantically
    Search {
        /// Query string
        query: String,
        
        /// Number of results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    
    /// Analyze chat patterns and themes
    Analyze {
        /// Analysis type
        #[arg(short, long, default_value = "themes")]
        analysis_type: String,
        
        /// Output format
        #[arg(short, long, default_value = "json")]
        format: String,
    },
}
