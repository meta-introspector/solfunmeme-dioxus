use serde::{Deserialize, Serialize};
use super::code_snippet::CodeSnippet;
use super::function_info::FunctionInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedDocument {
    pub file_path: String,
    pub code_snippets: Vec<CodeSnippet>,
    pub text_chunks: Vec<String>,
    pub analyzed_snippets: Vec<FunctionInfo>, // Reusing FunctionInfo for snippets
}
