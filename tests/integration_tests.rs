use shared_analysis_types::{CodeSnippet, ExtractedFile, ProcessingFile, TestResult, DocumentSummary, ConversationTurn, UploadedFile, AnnotatedWord, Multivector, ProcessingStats, ProcessingError, LanguageConfig};

const SAMPLE_RUST_CODE: &str = r#"
use std::collections::HashMap;

/// A sample function for testing
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// A sample struct
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}
"#;

#[test]
fn test_code_snippet_extraction() {
    let snippets = shared_analysis_types::extract_code_snippets(SAMPLE_RUST_CODE);
    
    // Should extract at least one snippet
    assert!(!snippets.is_empty());
    
    // Check that the first snippet has the expected content
    let first_snippet = &snippets[0];
    assert_eq!(first_snippet.language, "rust");
    assert!(first_snippet.content.contains("fn fibonacci"));
    assert!(first_snippet.content.contains("struct Point"));
    assert!(first_snippet.line_start > 0);
    assert!(first_snippet.line_end > first_snippet.line_start);
    assert!(first_snippet.token_count > 0);
    assert!(first_snippet.line_count > 0);
    assert!(first_snippet.char_count > 0);
}

#[test]
fn test_extracted_file_processing() {
    let snippet = CodeSnippet {
        language: "rust".to_string(),
        content: SAMPLE_RUST_CODE.to_string(),
        line_start: 1,
        line_end: 25,
        content_hash: shared_analysis_types::generate_content_hash(SAMPLE_RUST_CODE),
        token_count: shared_analysis_types::estimate_token_count(SAMPLE_RUST_CODE),
        line_count: SAMPLE_RUST_CODE.lines().count(),
        char_count: SAMPLE_RUST_CODE.chars().count(),
        test_result: Some(shared_analysis_types::create_default_test_result()),
    };
    
    let file = ExtractedFile {
        name: "sample.rs".to_string(),
        snippets: vec![snippet],
        total_lines: SAMPLE_RUST_CODE.lines().count(),
    };
    
    assert_eq!(file.name, "sample.rs");
    assert_eq!(file.snippets.len(), 1);
    assert_eq!(file.total_lines, SAMPLE_RUST_CODE.lines().count());
    
    // Verify the snippet properties
    let file_snippet = &file.snippets[0];
    assert_eq!(file_snippet.language, "rust");
    assert!(file_snippet.content.contains("fn fibonacci"));
    assert!(file_snippet.test_result.is_some());
}

#[test]
fn test_processing_file_workflow() {
    let processing_file = ProcessingFile {
        name: "test.rs".to_string(),
        progress: 50,
        total_lines: 100,
        current_content: SAMPLE_RUST_CODE.to_string(),
        summary: Some(DocumentSummary {
            total_turns: 1,
            total_code_snippets: 1,
            total_tokens: shared_analysis_types::estimate_token_count(SAMPLE_RUST_CODE),
            languages_found: vec!["rust".to_string()],
            content_hashes: vec![shared_analysis_types::generate_content_hash(SAMPLE_RUST_CODE)],
        }),
    };
    
    assert_eq!(processing_file.name, "test.rs");
    assert_eq!(processing_file.progress, 50);
    assert_eq!(processing_file.total_lines, 100);
    assert!(processing_file.summary.is_some());
    
    let summary = processing_file.summary.unwrap();
    assert_eq!(summary.total_turns, 1);
    assert_eq!(summary.total_code_snippets, 1);
    assert_eq!(summary.languages_found.len(), 1);
    assert_eq!(summary.languages_found[0], "rust");
}

#[test]
fn test_conversation_turn_processing() {
    let snippet = CodeSnippet {
        language: "rust".to_string(),
        content: "fn hello() { println!(\"world\"); }".to_string(),
        line_start: 1,
        line_end: 1,
        content_hash: "abc123".to_string(),
        token_count: 6,
        line_count: 1,
        char_count: 32,
        test_result: None,
    };
    
    let turn = ConversationTurn {
        author: "developer".to_string(),
        content: "Here's a simple function:".to_string(),
        code_snippets: vec![snippet],
    };
    
    assert_eq!(turn.author, "developer");
    assert_eq!(turn.content, "Here's a simple function:");
    assert_eq!(turn.code_snippets.len(), 1);
    assert_eq!(turn.code_snippets[0].language, "rust");
}

#[test]
fn test_uploaded_file_processing() {
    let uploaded_file = UploadedFile {
        name: "project.zip".to_string(),
        contents: SAMPLE_RUST_CODE.to_string(),
        generated_program: "fn main() { fibonacci(10); }".to_string(),
        summary: Some(DocumentSummary {
            total_turns: 1,
            total_code_snippets: 1,
            total_tokens: 50,
            languages_found: vec!["rust".to_string()],
            content_hashes: vec!["hash123".to_string()],
        }),
        zip_url: Some("https://example.com/project.zip".to_string()),
    };
    
    assert_eq!(uploaded_file.name, "project.zip");
    assert!(uploaded_file.contents.contains("fn fibonacci"));
    assert!(uploaded_file.generated_program.contains("fn main"));
    assert!(uploaded_file.summary.is_some());
    assert!(uploaded_file.zip_url.is_some());
}

#[test]
fn test_multivector_operations() {
    let mv1 = Multivector {
        scalar: 1.0,
        vector: [0.1, 0.2, 0.3],
    };
    
    let mv2 = Multivector {
        scalar: 2.0,
        vector: [0.4, 0.5, 0.6],
    };
    
    // Test default implementation
    let default_mv = Multivector::default();
    assert_eq!(default_mv.scalar, 0.0);
    assert_eq!(default_mv.vector, [0.0, 0.0, 0.0]);
    
    // Test partial equality
    assert_ne!(mv1, mv2);
    assert_eq!(mv1, mv1.clone());
}

#[test]
fn test_annotated_word_creation() {
    let word = AnnotatedWord {
        word: "function".to_string(),
        primary_emoji: "🔧".to_string(),
        secondary_emoji: "⚙️".to_string(),
        wikidata: Some("Q12345".to_string()),
        embedding: vec![0.1, 0.2, 0.3, 0.4],
        multivector: Multivector {
            scalar: 1.0,
            vector: [0.1, 0.2, 0.3],
        },
    };
    
    assert_eq!(word.word, "function");
    assert_eq!(word.primary_emoji, "🔧");
    assert_eq!(word.secondary_emoji, "⚙️");
    assert_eq!(word.wikidata, Some("Q12345".to_string()));
    assert_eq!(word.embedding.len(), 4);
    assert_eq!(word.multivector.scalar, 1.0);
}

#[test]
fn test_processing_error_handling() {
    let errors = vec![
        ProcessingError::FileReadError("file not found".to_string()),
        ProcessingError::ParseError("invalid syntax".to_string()),
        ProcessingError::TestExecutionError("test failed".to_string()),
        ProcessingError::InvalidFormat("wrong format".to_string()),
    ];
    
    for error in errors {
        let error_string = error.to_string();
        assert!(!error_string.is_empty());
        
        // Test that the error can be converted to a string
        match error {
            ProcessingError::FileReadError(msg) => assert!(error_string.contains(&msg)),
            ProcessingError::ParseError(msg) => assert!(error_string.contains(&msg)),
            ProcessingError::TestExecutionError(msg) => assert!(error_string.contains(&msg)),
            ProcessingError::InvalidFormat(msg) => assert!(error_string.contains(&msg)),
        }
    }
}