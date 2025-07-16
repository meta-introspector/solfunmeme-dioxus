use serde::{Deserialize, Serialize};

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
