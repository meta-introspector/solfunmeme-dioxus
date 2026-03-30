/// Generate unique identifier for a code snippet
pub fn generate_snippet_id(file_name: &str, snippet_idx: usize) -> String {
    format!("{}_{}", file_name, snippet_idx)
}
