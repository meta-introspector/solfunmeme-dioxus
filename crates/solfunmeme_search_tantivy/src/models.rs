use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub language: String,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
    pub content_hash: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<String>,
    pub embedding: Option<Vec<f32>>,
    pub clifford_vector: Option<Vec<f32>>,
    pub semantic_summary: Option<String>,
    pub score: f32,
}
