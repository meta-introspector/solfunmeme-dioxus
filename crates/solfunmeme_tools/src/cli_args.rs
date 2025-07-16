use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "llm_planner")]
#[command(about = "Plans LLM reflection tasks by grouping code chunks based on embeddings.", long_about = None)]
pub struct Cli {
    #[arg(long, help = "Path to the Tantivy index directory.")]
    pub index_path: PathBuf,
    #[arg(long, default_value = "100", help = "Number of top chunks to retrieve for analysis.")]
    pub limit: usize,
    #[arg(long, default_value = "0.7", help = "Cosine similarity threshold for grouping chunks.")]
    pub similarity_threshold: f32,
    #[arg(long, help = "Path to the LLM providers configuration TOML file.")]
    pub llm_config_path: PathBuf,
    #[arg(long, default_value = "text", help = "Output format: text or json.")]
    pub output_format: String,
    #[arg(long, default_value = "code_reflection", help = "Type of task to generate: code_reflection, clifford_operation, or code_evolution.")]
    pub task_type: String,
    #[arg(long, help = "Code snippet for code_evolution task.")]
    pub code_snippet: Option<String>,
}
