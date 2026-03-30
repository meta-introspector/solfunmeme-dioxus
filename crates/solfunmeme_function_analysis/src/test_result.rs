use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TestResult {
    pub passed: bool,
    pub error_message: Option<String>,
    pub execution_time: Option<std::time::Duration>,
    pub output: Option<String>,
}
