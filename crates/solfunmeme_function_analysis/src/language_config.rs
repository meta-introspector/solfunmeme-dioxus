use serde::{Deserialize, Serialize};

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
