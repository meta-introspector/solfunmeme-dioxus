pub mod process_file;
pub mod project_analyzer;

pub use solfunmeme_function_analysis::{CodeChunk, AnalyzedDocument, AnalyzedFunction, AnalyzedToken};
pub use project_analyzer::{
    rust_analyzer::analyze_project,
    markdown_analyzer::analyze_markdown_files,
    token_analyzer::analyze_project_tokens,
};
