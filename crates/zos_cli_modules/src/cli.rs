use solfunmeme_core_utils::clap::{Parser, command};
use solfunmeme_serde_utils::serde::{Deserialize, Serialize};
use crate::commands::Commands;

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(about = "A CLI for the Solfunmeme project")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
