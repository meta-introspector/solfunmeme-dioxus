use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    MarkdownError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MarkdownError(msg) => write!(f, "Markdown error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct CodeSnippet {
    pub language: String,
    pub content: String,
}

pub fn process_turn(turn_content: &str) -> Result<(String, String), Error> {
    let (speaker, content) = extract_speaker_and_content(turn_content);
    let processed_message = process_message_content(content)?;
    Ok((speaker.to_string(), processed_message))
}

fn extract_speaker_and_content(turn: &str) -> (&str, &str) {
    let trimmed_turn = turn.trim();
    if trimmed_turn.starts_with("User") {
        ("User", trimmed_turn.trim_start_matches("User").trim())
    } else if trimmed_turn.starts_with("Grok AI") {
        ("Grok AI", trimmed_turn.trim_start_matches("Grok AI").trim())
    } else {
        ("Unknown", trimmed_turn)
    }
}

fn extract_markdown_snippets(content: &str) -> Result<Vec<CodeSnippet>, Error> {
    let code_block_regex = Regex::new(r"```(\w+)?\n(.*?)```").map_err(|e| Error::MarkdownError(e.to_string()))?;
    let mut snippets = Vec::new();
    
    for cap in code_block_regex.captures_iter(content) {
        let language = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_else(|| "text".to_string());
        let code_content = cap.get(2).map(|m| m.as_str().to_string()).unwrap_or_default();
        
        snippets.push(CodeSnippet {
            language,
            content: code_content,
        });
    }
    
    Ok(snippets)
}

fn process_message_content(content: &str) -> Result<String, Error> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
    // Split content into lines
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines at the start
        if processed.is_empty() && line.is_empty() {
            i += 1;
            continue;
        }

        // Handle LaTeX blocks
        if line.starts_with("\\documentclass") {
            let mut latex_block = Vec::new();
            while i < lines.len() && !lines[i].trim().is_empty() {
                latex_block.push(lines[i]);
                i += 1;
            }
            processed.push_str("```latex\n");
            processed.push_str(&latex_block.join("\n"));
            processed.push_str("\n```\n\n");
            continue;
        }

        // Handle regular text
        if !line.is_empty() {
            processed.push_str(line);
            processed.push_str("\n");
        } else if !processed.ends_with("\n\n") {
            processed.push_str("\n");
        }

        i += 1;
    }

    // Append code snippets if found
    if !code_snippets.is_empty() {
        processed.push_str("\n### Code Snippets\n\n");
        for snippet in code_snippets {
            processed.push_str(&format!("```{}\n{}\n```\n\n",
                snippet.language, snippet.content));
        }
    }

    Ok(processed)
}