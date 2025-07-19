use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexedDocument {
    pub path: Option<String>,
    pub content: Option<String>,
    pub emoji: Option<String>,
    pub line_start: Option<u64>,
    pub line_end: Option<u64>,
    pub chunk_type: Option<String>,
    pub language: Option<String>,
    pub content_hash: Option<String>,
    pub token_count: Option<u64>,
    pub line_count: Option<u64>,
    pub char_count: Option<u64>,
    pub test_result: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStats {
    pub total_documents: usize,
    pub fields: Vec<FieldStats>,
    pub emoji_distribution: std::collections::HashMap<String, usize>,
    pub language_distribution: std::collections::HashMap<String, usize>,
    pub chunk_type_distribution: std::collections::HashMap<String, usize>,
    pub path_distribution: std::collections::HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldStats {
    pub field_name: String,
    pub non_null_count: usize,
    pub unique_values: usize,
}
