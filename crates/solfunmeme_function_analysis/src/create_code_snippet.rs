use md5;
use super::code_snippet::CodeSnippet;

pub fn create_code_snippet(
    language: String,
    content: String,
    line_start: usize,
    line_end: usize,
) -> CodeSnippet {
    let content_hash = format!("{:x}", md5::compute(&content));
    let token_count = content.split_whitespace().count();
    let line_count = content.lines().count();
    let char_count = content.chars().count();
    let test_result = Some("Untested".to_string());

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
