use std::path::Path;
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DocumentSummary {
    pub total_turns: usize,
    pub total_code_snippets: usize,
    pub total_tokens: usize,
    pub languages_found: Vec<String>,
    pub content_hashes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ConversationTurn {
    pub author: String,
    pub content: String,
    pub code_snippets: Vec<CodeSnippet>,
}

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub content: String,
    pub language: String,
    pub token_count: usize,
    pub content_hash: String,
}

pub fn process_file(file_path: &Path) -> Result<DocumentSummary> {
    let content = fs::read_to_string(file_path)?;

    let mut turns = Vec::new();
    let parts = content.split("---");

    for part in parts {
        let trimmed_part = part.trim();
        if trimmed_part.starts_with("### User") {
            let content = trimmed_part.strip_prefix("### User").unwrap_or("").trim();
            let code_snippets = extract_code_snippets(content)?;
            turns.push(ConversationTurn {
                author: "User".to_string(),
                content: content.to_string(),
                code_snippets,
            });
        } else if trimmed_part.starts_with("### Grok AI") {
            let content = trimmed_part.strip_prefix("### Grok AI").unwrap_or("").trim();
            let mut code_snippets = extract_code_snippets(content)?;

            // Test the code snippets
            for snippet in &mut code_snippets {
                test_code_snippet(snippet);
            }

            turns.push(ConversationTurn {
                author: "Grok AI".to_string(),
                content: content.to_string(),
                code_snippets,
            });
        }
    }

    // Create summary
    let total_turns = turns.len();
    let total_code_snippets: usize = turns.iter().map(|t| t.code_snippets.len()).sum();
    let total_tokens: usize = turns.iter()
        .flat_map(|t| &t.code_snippets)
        .map(|s| s.token_count)
        .sum();

    let mut languages_found: Vec<String> = turns.iter()
        .flat_map(|t| &t.code_snippets)
        .map(|s| s.language.clone())
        .collect();
    languages_found.sort();
    languages_found.dedup();

    let content_hashes: Vec<String> = turns.iter()
        .flat_map(|t| &t.code_snippets)
        .map(|s| s.content_hash.clone())
        .collect();

    Ok(DocumentSummary {
        total_turns,
        total_code_snippets,
        total_tokens,
        languages_found,
        content_hashes,
    })
}

fn extract_code_snippets(content: &str) -> Result<Vec<CodeSnippet>> {
    // Simple implementation - extract code blocks
    let mut snippets = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_code_block = false;
    let mut current_snippet = String::new();
    let mut current_language = String::new();

    for line in lines {
        if line.starts_with("```") {
            if in_code_block {
                // End of code block
                snippets.push(CodeSnippet {
                    content: current_snippet.clone(),
                    language: current_language.clone(),
                    token_count: current_snippet.split_whitespace().count(),
                    content_hash: format!("{:x}", md5::compute(&current_snippet)),
                });
                current_snippet.clear();
                current_language.clear();
                in_code_block = false;
            } else {
                // Start of code block
                in_code_block = true;
                current_language = line.strip_prefix("```").unwrap_or("").to_string();
            }
        } else if in_code_block {
            current_snippet.push_str(line);
            current_snippet.push('\n');
        }
    }

    Ok(snippets)
}

fn test_code_snippet(snippet: &mut CodeSnippet) {
    // Simple validation - check if it looks like valid code
    let has_brackets = snippet.content.contains('{') && snippet.content.contains('}');
    let has_semicolons = snippet.content.contains(';');
    let has_keywords = snippet.content.contains("fn") || snippet.content.contains("let") || snippet.content.contains("pub");
    
    if has_brackets || has_semicolons || has_keywords {
        snippet.content_hash = format!("valid_{:x}", md5::compute(&snippet.content));
    } else {
        snippet.content_hash = format!("invalid_{:x}", md5::compute(&snippet.content));
    }
} 