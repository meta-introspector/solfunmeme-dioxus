use serde::{Deserialize, Serialize};

pub mod artifact;

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiState {
    pub last_run: String,
    pub conversation_history: Vec<String>,
}

pub fn export_state() -> GeminiState {
    GeminiState {
        last_run: chrono::Utc::now().to_rfc3339(),
        conversation_history: vec!["User: Hello".to_string(), "Gemini: Hi there!".to_string()],
    }
}
