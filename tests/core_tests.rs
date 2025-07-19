use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_function_analysis::{ExtractedFile, TestResult, DocumentSummary, ProcessingStats, ProcessingError, LanguageConfig};

#[test]
fn test_code_snippet_creation() {
    let snippet = CodeChunk {
        language: "rust".to_string(),
        content: "fn test() { println!(\"hello\"); }".to_string(),
        line_start: 1,
        line_end: 1,
        content_hash: "abc123".to_string(),
        token_count: 8,
        line_count: 1,
        char_count: 32,
        test_result: Some(TestResult::default()),
        embedding: vec![],
        clifford_vector: Some(solfunmeme_clifford::SerializableMultivector(tclifford::Multivector::default())),
    };
    
    assert_eq!(snippet.language, "rust");
    assert_eq!(snippet.content, "fn test() { println!(\"hello\"); }");
    assert_eq!(snippet.line_start, 1);
    assert_eq!(snippet.line_end, 1);
    assert_eq!(snippet.token_count, 8);
    assert_eq!(snippet.line_count, 1);
    assert_eq!(snippet.char_count, 32);
    assert_eq!(snippet.test_result, Some(TestResult { passed: true, error_message: None, execution_time: None, output: Some("Passed".to_string()) }));
}

#[test]
fn test_extracted_file_creation() {
    let snippet = CodeChunk {
        language: "rust".to_string(),
        content: "fn main() {}".to_string(),
        line_start: 1,
        line_end: 1,
        content_hash: "def456".to_string(),
        token_count: 3,
        line_count: 1,
        char_count: 12,
        test_result: None,
        clifford_vector: Some(solfunmeme_clifford::SerializableMultivector(tclifford::Multivector::default())),
        embedding: vec![],
    };
    
    let file = ExtractedFile {
        name: "test.rs".to_string(),
        snippets: vec![snippet],
        total_lines: 1,
    };
    
    assert_eq!(file.name, "test.rs");
    assert_eq!(file.snippets.len(), 1);
    assert_eq!(file.total_lines, 1);
    assert_eq!(file.snippets[0].language, "rust");
}

#[test]
fn test_multivector_creation() {
    let mv = solfunmeme_clifford::SerializableMultivector(tclifford::Multivector::from_scalar(1.0));
    
    assert_eq!(mv.0.scalar_part(), 1.0);
    assert_eq!(mv.0.extract_vector().as_slice(), Some(&[0.0f32, 0.0f32, 0.0f32][..]));
}

#[test]
fn test_processing_stats_creation() {
    let stats = ProcessingStats {
        files_processed: 5,
        total_snippets_extracted: 10,
        total_lines_processed: 100,
        processing_time_ms: 5000,
        languages_detected: vec!["rust".to_string(), "python".to_string()],
    };
    
    assert_eq!(stats.files_processed, 5);
    assert_eq!(stats.total_snippets_extracted, 10);
    assert_eq!(stats.total_lines_processed, 100);
    assert_eq!(stats.processing_time_ms, 5000);
    assert_eq!(stats.languages_detected.len(), 2);
    assert!(stats.languages_detected.contains(&"rust".to_string()));
    assert!(stats.languages_detected.contains(&"python".to_string()));
}

#[test]
fn test_language_config_default() {
    let config = LanguageConfig::default();
    
    assert_eq!(config.name, "text");
    assert_eq!(config.file_extension, "txt");
    assert_eq!(config.comment_prefix, "#");
    assert_eq!(config.supports_testing, false);
    assert_eq!(config.syntax_highlighter, None);
}

#[test]
fn test_processing_error_display() {
    let error = ProcessingError::FileReadError("file not found".to_string());
    let error_string = error.to_string();
    
    assert!(error_string.contains("file not found"));
}

#[test]
fn test_document_summary_creation() {
    let summary = DocumentSummary {
        total_turns: 3,
        total_code_snippets: 5,
        total_tokens: 100,
        languages_found: vec!["rust".to_string()],
        content_hashes: vec!["hash1".to_string(), "hash2".to_string()],
    };
    
    assert_eq!(summary.total_turns, 3);
    assert_eq!(summary.total_code_snippets, 5);
    assert_eq!(summary.total_tokens, 100);
    assert_eq!(summary.languages_found.len(), 1);
    assert_eq!(summary.content_hashes.len(), 2);
}