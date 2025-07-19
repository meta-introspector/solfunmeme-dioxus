use crate::model::estimate_token_count::estimate_token_count;
use solfunmeme_function_analysis::{CodeChunk, TestResult};
use crate::model::content_hash::create_content_hash;

pub fn handle_inline_code_node(inline_code: &markdown::mdast::InlineCode, snippets: &mut Vec<CodeChunk>) {
    if inline_code.value.len() > 10 {
        // Only consider substantial inline code
        let content_hash = create_content_hash(&inline_code.value);
        let token_count = estimate_token_count(&inline_code.value);
        let line_count = inline_code.value.lines().count();
        let char_count = inline_code.value.chars().count();

        snippets.push(CodeChunk {
            content: inline_code.value.clone(),
            content_hash,
            language: "inline".to_string(),
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