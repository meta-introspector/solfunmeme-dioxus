use serde::{Deserialize, Serialize};
use super::code_snippet::CodeSnippet;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub author: String,
    pub content: String,
    pub code_snippets: Vec<CodeSnippet>,
}
