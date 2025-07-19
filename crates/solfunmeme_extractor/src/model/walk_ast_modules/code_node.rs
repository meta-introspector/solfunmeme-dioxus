use crate::model::estimate_token_count::estimate_token_count;
use solfunmeme_function_analysis::{CodeChunk, TestResult};
use crate::model::content_hash::create_content_hash;

pub fn handle_code_node(code: &markdown::mdast::Code, snippets: &mut Vec<CodeChunk>) {
    if !code.value.trim().is_empty() {
        let content_hash = create_content_hash(&code.value);
        let token_count = estimate_token_count(&code.value);
        let line_count = code.value.lines().count();
        let char_count = code.value.chars().count();
        let language = code.lang.as_deref().unwrap_or("text").to_string();

        snippets.push(CodeChunk {
            content: code.value.clone(),
            content_hash,
            language,
            token_count,
            line_count,
            char_count,
            test_result: Some(TestResult::default()), // Default value
            line_start: 0, // Default value
            line_end: 0,   // Default value
            embedding: Vec::new(), // Default value
            clifford_vector: None, // Default value
            semantic_summary: None,
        });
    }
}