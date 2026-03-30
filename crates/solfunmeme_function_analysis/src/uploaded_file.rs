use serde::{Deserialize, Serialize};
use super::document_summary::DocumentSummary;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UploadedFile {
    pub name: String,
    pub contents: String,
    pub generated_program: String,
    pub summary: Option<DocumentSummary>,
    pub zip_url: Option<String>,
}
