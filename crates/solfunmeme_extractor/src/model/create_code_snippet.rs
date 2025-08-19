use crate::types::CodeSnippet;
use super::generate_content_hash::generate_content_hash;
use super::estimate_token_count::estimate_token_count;
use super::create_default_test_result::create_default_test_result;

/// Create a complete CodeSnippet with all fields populated
pub fn create_code_snippet(
    language: String,
    content: String,
    line_start: usize,
    line_end: usize,
) -> CodeSnippet {
    let content_hash = generate_content_hash(&content);
    let token_count = estimate_token_count(&content);
    let line_count = content.lines().count();
    let char_count = content.chars().count();
    let test_result = Some(create_default_test_result());

    CodeSnippet {
        language,
        content,
        line_start,
        line_end,
        content_hash,
        token_count,
        line_count,
        char_count,
        test_result,
    }
}
