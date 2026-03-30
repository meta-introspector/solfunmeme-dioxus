use md5;
use super::code_snippet::CodeSnippet;
use super::create_code_snippet::create_code_snippet;

pub fn extract_code_snippets(content: &str) -> Vec<CodeSnippet> {
    let mut snippets = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_code_block = false;
    let mut current_language = String::new();
    let mut current_content = String::new();
    let mut start_line = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("```") {
            if in_code_block {
                // End of code block
                if !current_content.trim().is_empty() {
                    let snippet = create_code_snippet(
                        current_language.clone(),
                        current_content.clone(),
                        i - current_content.lines().count(),
                        i,
                    );
                    snippets.push(snippet);
                }
                current_content.clear();
                in_code_block = false;
            } else {
                // Start of code block
                current_language = line[3..].trim().to_string();
                if current_language.is_empty() {
                    current_language = "text".to_string();
                }
                current_content.clear();
                start_line = i + 1;
                in_code_block = true;
            }
        } else if in_code_block {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }

    // Handle unclosed code block
    if in_code_block && !current_content.trim().is_empty() {
        let snippet = create_code_snippet(
            current_language,
            current_content.clone(),
            start_line,
            lines.len(),
        );
        snippets.push(snippet);
    }

    snippets
}
