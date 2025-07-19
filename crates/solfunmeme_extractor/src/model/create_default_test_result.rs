use solfunmeme_function_analysis::TestResult;

/// Create a default test result
pub fn create_default_test_result() -> TestResult {
    TestResult {
        passed: false,
        error_message: None,
        execution_time: None,
        output: None,
    }
}
