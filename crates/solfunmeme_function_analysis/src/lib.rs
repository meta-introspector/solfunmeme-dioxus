use syn::ReturnType;
use std::path::Path;
use walkdir::WalkDir;
use md5;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosestEmojiInfo {
    pub emoji: String,
    pub category: String,
    pub distance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedFunction {
    pub function_name: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub file_path: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emojis: Vec<ClosestEmojiInfo>,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub function_name: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub file_path: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emoji: String,
    pub emoji_category: String,
    pub emoji_distance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedDocument {
    pub file_path: String,
    pub code_snippets: Vec<CodeChunk>,
    pub text_chunks: Vec<String>,
    pub analyzed_snippets: Vec<AnalyzedFunction>, // Changed from FunctionInfo to AnalyzedFunction
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedToken {
    pub token: String,
    pub count: usize,
    pub multivector_str: String,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeChunk {
    pub language: String,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
    pub content_hash: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: String,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Multivector {
    pub scalar: f32,
    pub vector: [f32; 3],
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExtractedFile {
    pub name: String,
    pub snippets: Vec<CodeChunk>,
    pub total_lines: usize,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessingFile {
    pub name: String,
    pub progress: usize,
    pub total_lines: usize,
    pub current_content: String,
    pub summary: Option<DocumentSummary>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TestResult {
    pub passed: bool,
    pub error_message: Option<String>,
    pub execution_time: Option<std::time::Duration>,
    pub output: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentSummary {
    pub total_turns: usize,
    pub total_code_snippets: usize,
    pub total_tokens: usize,
    pub languages_found: Vec<String>,
    pub content_hashes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub author: String,
    pub content: String,
    pub code_snippets: Vec<CodeChunk>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UploadedFile {
    pub name: String,
    pub contents: String,
    pub generated_program: String,
    pub summary: Option<DocumentSummary>,
    pub zip_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AnnotatedWord {
    pub word: String,
    pub primary_emoji: String,
    pub secondary_emoji: String,
    pub wikidata: Option<String>,
    pub embedding: Vec<f32>,
    pub multivector: Multivector,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub files_processed: usize,
    pub total_snippets_extracted: usize,
    pub total_lines_processed: usize,
    pub processing_time_ms: u64,
    pub languages_detected: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProcessingError {
    FileReadError(String),
    ParseError(String),
    TestExecutionError(String),
    InvalidFormat(String),
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingError::FileReadError(msg) => write!(f, "File read error: {}", msg),
            ProcessingError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProcessingError::TestExecutionError(msg) => write!(f, "Test execution error: {}", msg),
            ProcessingError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for ProcessingError {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub name: String,
    pub file_extension: String,
    pub comment_prefix: String,
    pub supports_testing: bool,
    pub syntax_highlighter: Option<String>,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            name: "text".to_string(),
            file_extension: "txt".to_string(),
            comment_prefix: "#".to_string(),
            supports_testing: false,
            syntax_highlighter: None,
        }
    }
}

pub fn extract_code_snippets(content: &str) -> Vec<CodeChunk> {
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

pub fn create_code_snippet(
    language: String,
    content: String,
    line_start: usize,
    line_end: usize,
) -> CodeChunk {
    let content_hash = format!("{:x}", md5::compute(&content));
    let token_count = content.split_whitespace().count();
    let line_count = content.lines().count();
    let char_count = content.chars().count();
    let test_result = "Untested".to_string();

    CodeChunk {
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

pub fn analyze_rust_file(file_path: &Path) -> Vec<FunctionInfo> {
    let mut functions_info = Vec::new();
    let code = match std::fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", file_path.display(), e);
            return functions_info;
        }
    };

    let syntax = match syn::parse_file(&code) {
        Ok(syntax) => syntax,
        Err(e) => {
            eprintln!("Failed to parse file {}: {}", file_path.display(), e);
            return functions_info;
        }
    };

    for item in syntax.items {
        if let syn::Item::Fn(item_fn) = item {
            let function_name = item_fn.sig.ident.to_string();
            let code_snippet = quote::quote! { #item_fn }.to_string();
            let semantic_summary = extract_semantic_summary(&item_fn);
            functions_info.push(FunctionInfo {
                function_name,
                code_snippet,
                semantic_summary,
                file_path: file_path.to_string_lossy().replace("\\", "/").to_owned(),
                multivector_str: String::new(), // Placeholder
                sieve_address: String::new(),   // Placeholder
                closest_emoji: String::new(),   // Placeholder
                emoji_category: String::new(),  // Placeholder
                emoji_distance: 0.0,            // Placeholder
            });
        }
    }
    functions_info
}

fn extract_semantic_summary(item_fn: &syn::ItemFn) -> String {
    let mut summary = String::new();
    // Extract identifiers and literals from the function to form a semantic summary
    // This is a simplified example; a real implementation would traverse the AST more thoroughly.
    summary.push_str(&item_fn.sig.ident.to_string());
    for input in &item_fn.sig.inputs {
        summary.push_str(&format!("{:?}", input));
    }
    if let ReturnType::Type(_, ty) = &item_fn.sig.output {
        summary.push_str(&format!("{:?}", ty));
    }
    // Add more AST traversal logic here to extract meaningful information
    summary
}

pub fn find_rust_files(project_root: &Path) -> Vec<String> {
    let mut rust_files = Vec::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "rs" {
                    rust_files.push(entry.path().to_string_lossy().into_owned());
                }
            }
        }
    }
    rust_files
}

// FIXME: This file had an "unexpected closing delimiter" error previously.
// The current content is a consolidation of data models and utility functions.
// If the error reappears, carefully check for mismatched braces, parentheses, or brackets,
// especially around macro expansions or complex type definitions.

