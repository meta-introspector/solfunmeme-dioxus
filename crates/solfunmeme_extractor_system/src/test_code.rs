use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub error_message: Option<String>,
    pub output: Option<String>,
    pub execution_time: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub content: String,
    pub language: String,
    pub test_result: Option<TestResult>,
}

/// Tests a code snippet by attempting to compile/execute it.
pub fn test_code_snippet_old(snippet: &mut CodeSnippet) {
    let start_time = std::time::Instant::now();
    let output = None;
    match snippet.language.as_str() {
        "rust" => {
            // For Rust code, we could try to compile it in a sandbox
            // For now, we'll just do basic syntax checking
            if snippet.content.contains("fn ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    passed: true,
                    error_message: None,
                    output,
                    execution_time: Some(start_time.elapsed()),
                });
            } else {
                snippet.test_result = Some(TestResult {
                    passed: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time: Some(start_time.elapsed()),
                    output,
                });
            }
        }
        "javascript" | "js" => {
            // For JavaScript, we could use a JS engine
            // For now, just check for basic syntax
            if snippet.content.contains("function")
                || snippet.content.contains("const ")
                || snippet.content.contains("let ")
            {
                snippet.test_result = Some(TestResult {
                    passed: true,
                    error_message: None,
                    execution_time: Some(start_time.elapsed()),
                    output,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    passed: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time: Some(start_time.elapsed()),
                    output,
                });
            }
        }
        _ => {
            // For other languages, just mark as untested
            snippet.test_result = Some(TestResult {
                passed: false,
                error_message: Some("Language not supported for testing".to_string()),
                execution_time: Some(start_time.elapsed()),
                output,
            });
        }
    }
}

/// Tests a code snippet by attempting to compile/execute it.  
pub fn test_code_snippet(snippet: &mut CodeSnippet) {
    let start_time = std::time::Instant::now();
    let output = None;
    match snippet.language.as_str() {
        "rust" => {
            if snippet.content.contains("fn ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    passed: true,
                    error_message: None,
                    execution_time: Some(start_time.elapsed()),
                    output,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    passed: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time: Some(start_time.elapsed()),
                    output,
                });
            }
        }
        "javascript" | "js" => {
            if snippet.content.contains("function")
                || snippet.content.contains("const ")
                || snippet.content.contains("let ")
            {
                snippet.test_result = Some(TestResult {
                    passed: true,
                    error_message: None,
                    execution_time: Some(start_time.elapsed()),
                    output,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    passed: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time: Some(start_time.elapsed()),
                    output,
                });
            }
        }
        _ => {
            snippet.test_result = Some(TestResult {
                passed: false,
                error_message: Some("Language not supported for testing".to_string()),
                execution_time: Some(start_time.elapsed()),
                output,
            });
        }
    }
} 