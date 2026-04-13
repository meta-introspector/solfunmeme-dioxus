use crate::extractor::{
    model::extract_html::extract_code_snippets_from_html,
    types::{CodeSnippet, DocumentSummary, ExtractedFile, ProcessingFile, TestResult},
};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};

// /// Extract code snippets from markdown content
// src/extractor/model/extract.rs
//  11:pub fn extract_code_snippets(content: &str) -> Vec<CodeSnippet> {
// 174:            let snippets = extract_code_snippets(&content);
// src/extractor/system/files.rs
//   7:use crate::extractor::{model::extract::extract_code_snippets, types::{ExtractedFile, ProcessingFile}};
//  37:            let snippets = extract_code_snippets(&content);
// 100:            let snippets = extract_code_snippets(&content);
// 152:                let snippets = extract_code_snippets(&content);
// src/extractor/system/process_file.rs
// 6:use crate::extractor::model::extract::extract_code_snippets;

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
                        start_line,
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
        let snippet =
            create_code_snippet(current_language, current_content, start_line, lines.len());
        snippets.push(snippet);
    }

    snippets
}

/// Create a complete CodeSnippet with all fields populated
fn create_code_snippet(
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

/// Generate a simple hash for content deduplication
fn generate_content_hash(content: &str) -> String {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Estimate token count (rough approximation)
pub fn estimate_token_count(content: &str) -> usize {
    // Simple estimation: split by whitespace and common delimiters
    content
        .split_whitespace()
        .map(|word| {
            // Count punctuation and operators as separate tokens
            let punctuation_count = word.chars().filter(|c| c.is_ascii_punctuation()).count();
            1 + punctuation_count
        })
        .sum()
}

/// Create a default test result
fn create_default_test_result() -> TestResult {
    TestResult {
        passed: false,
        error_message: None,
        execution_time: None,
        output: None,
    }
}

/// Process files from FileEngine and extract code snippets
pub async fn process_file_engine(
    file_engine: Arc<dyn FileEngine>,
    mut files: Signal<Vec<ExtractedFile>>,
    mut processing_file: Signal<Option<ProcessingFile>>,
) {
    let file_names = file_engine.files();

    for file_name in &file_names {
        let summary = Some(DocumentSummary {
            total_turns: 9,
            total_code_snippets: 0,
            total_tokens: 0,
            languages_found: [].to_vec(),
            content_hashes: [].to_vec(),
        });
        // Start processing this file
        processing_file.set(Some(ProcessingFile {
            name: file_name.clone(),
            progress: 0,
            total_lines: 0,
            current_content: String::new(),
            summary,
        }));

        // Small delay for UI responsiveness
        TimeoutFuture::new(50).await;

        if let Some(content) = file_engine.read_file_to_string(file_name).await {
            let lines: Vec<&str> = content.lines().collect();
            let total_lines = lines.len();

            // Update processing status
            if let Some(pf) = processing_file.write().as_mut() {
                pf.total_lines = total_lines;
                pf.current_content = content.clone();
            }

            // Simulate progress for visual feedback
            let progress_steps = (total_lines / 100).max(1);
            for i in (0..=total_lines).step_by(progress_steps) {
                if let Some(pf) = processing_file.write().as_mut() {
                    pf.progress = i.min(total_lines);
                }
                TimeoutFuture::new(10).await;
            }

            // Extract code snippets
            // FIXME why are we doing the same thing twice?
            let snippets = extract_code_snippets(&content);
            // …then append any HTML‐based snippets
            let mut all_snippets = snippets;
            all_snippets.extend(extract_code_snippets_from_html(&content));
            // Finally push one combined ExtractedFile
            files.write().push(ExtractedFile {
                name: file_name.clone(),
                snippets: all_snippets,
                total_lines,
            });
        }
    }

    // Clear processing state
    processing_file.set(None);
}

/// Generate unique identifier for a code snippet
pub fn generate_snippet_id(file_name: &str, snippet_idx: usize) -> String {
    format!("{}_{}", file_name, snippet_idx)
}

/// Validate if content appears to be a markdown file
pub fn is_markdown_file(file_name: &str) -> bool {
    let lower_name = file_name.to_lowercase();
    lower_name.ends_with(".md")
        || lower_name.ends_with(".markdown")
        || lower_name.ends_with(".mdown")
}

/// Get language-specific file extension for downloaded snippets
pub fn get_file_extension(language: &str) -> &'static str {
    match language.to_lowercase().as_str() {
        "rust" | "rs" => "rs",
        "javascript" | "js" => "js",
        "typescript" | "ts" => "ts",
        "python" | "py" => "py",
        "java" => "java",
        "cpp" | "c++" => "cpp",
        "c" => "c",
        "html" => "html",
        "css" => "css",
        "json" => "json",
        "xml" => "xml",
        "yaml" | "yml" => "yml",
        "toml" => "toml",
        "sql" => "sql",
        "bash" | "sh" => "sh",
        "powershell" | "ps1" => "ps1",
        _ => "txt",
    }
}
