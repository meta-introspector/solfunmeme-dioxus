pub mod rust_processor;
pub mod markdown_processor;

use anyhow::Result;
use solfunmeme_function_analysis::CodeChunk;

pub trait LanguageProcessor {
    fn process_code(&self, content: &str, file_path: &str) -> Result<Vec<CodeChunk>>;
}