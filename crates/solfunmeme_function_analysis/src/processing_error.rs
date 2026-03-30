use serde::{Deserialize, Serialize};

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
