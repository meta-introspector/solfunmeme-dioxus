use crate::types::{Document, DocumentIndex};
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

pub struct DocumentationIndexer;

impl DocumentationIndexer {
    pub fn new() -> Self {
        Self
    }

    pub fn index_directory(&self, path: &Path) -> Result<DocumentIndex> {
        let mut index = DocumentIndex::new();
        
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension() {
                if ext == "md" || ext == "txt" || ext == "rs" {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        let doc = Document {
                            path: entry.path().to_path_buf(),
                            content,
                            title: None,
                            references: Vec::new(),
                        };
                        index.documents.push(doc);
                    }
                }
            }
        }
        
        Ok(index)
    }
} 