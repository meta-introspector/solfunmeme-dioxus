use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub files_processed: usize,
    pub total_snippets_extracted: usize,
    pub total_lines_processed: usize,
    pub processing_time_ms: u64,
    pub languages_detected: Vec<String>,
}
