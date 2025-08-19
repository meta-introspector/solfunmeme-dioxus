use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub function_name: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub file_path: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emoji: String,
    pub emoji_category: String,
    pub emoji_distance: f32,
}
