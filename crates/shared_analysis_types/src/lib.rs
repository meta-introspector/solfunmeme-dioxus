use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosestEmojiInfo {
    pub emoji: String,
    pub category: String,
    pub distance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedFunction {
    pub function_name: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub file_path: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emojis: Vec<ClosestEmojiInfo>,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedDocument {
    pub file_path: String,
    pub code_snippets: Vec<CodeSnippet>,
    pub text_chunks: Vec<String>,
    pub analyzed_snippets: Vec<AnalyzedFunction>, // Reusing AnalyzedFunction for snippets
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedToken {
    pub token: String,
    pub count: usize,
    pub multivector_str: String,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
    pub content_hash: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<String>,
}

