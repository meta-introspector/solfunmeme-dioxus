use crate::types::{CrossReference, CrossReferenceAnalysis, Document, DocumentIndex, ReferenceType};
use anyhow::Result;

pub struct CrossReferenceAnalyzer;

impl CrossReferenceAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, index: &DocumentIndex) -> Result<CrossReferenceAnalysis> {
        let mut analysis = CrossReferenceAnalysis::new();
        
        // Simple analysis - in practice this would be more sophisticated
        for doc in &index.documents {
            for reference in &doc.references {
                if let Some(referenced_doc) = self.find_document_by_path(index, reference) {
                    let cross_ref = CrossReference {
                        referencing_doc: doc.clone(),
                        referenced_doc: referenced_doc.clone(),
                        reference_type: ReferenceType::Link,
                        context: "Found in references".to_string(),
                    };
                    analysis.cross_references.push(cross_ref);
                }
            }
        }
        
        Ok(analysis)
    }

    pub fn load(path: &str) -> Result<CrossReferenceAnalysis> {
        match CrossReferenceAnalysis::load(path) {
            Ok(result) => Ok(result),
            Err(e) => Err(anyhow::anyhow!("Failed to load analysis: {}", e)),
        }
    }

    fn find_document_by_path<'a>(&self, index: &'a DocumentIndex, path: &str) -> Option<&'a Document> {
        index.documents.iter().find(|d| d.path.to_string_lossy() == path)
    }
} 