use serde::{Deserialize, Serialize};
use super::code_snippet::CodeSnippet;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExtractedFile {
    pub name: String,
    pub snippets: Vec<CodeSnippet>,
    pub total_lines: usize,
}
