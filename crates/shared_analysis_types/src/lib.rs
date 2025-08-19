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
pub struct AnalyzedDocument {
    pub file_path: String,
    pub code_snippets: Vec<CodeSnippet>,
    pub text_chunks: Vec<String>,
    pub analyzed_snippets: Vec<AnalyzedFunction>, // Reusing AnalyzedFunction for snippets
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedToken {
    pub token: String,
    pub count: usize,
    pub multivector_str: String,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
    pub content_hash: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Multivector {
    pub scalar: f32,
    pub vector: [f32; 3],
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExtractedFile {
    pub name: String,
    pub snippets: Vec<CodeSnippet>,
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
    pub code_snippets: Vec<CodeSnippet>,
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