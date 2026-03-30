use crate::types::CrossReferenceAnalysis;
use anyhow::Result;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_diagrams(&self, analysis: &CrossReferenceAnalysis) -> Result<()> {
        // Simple report generation
        println!("Generated cross-reference diagrams for {} references", analysis.cross_references.len());
        Ok(())
    }

    pub fn generate_summary(&self, analysis: &CrossReferenceAnalysis) -> Result<String> {
        let mut summary = String::new();
        summary.push_str(&format!("Cross-Reference Analysis Summary\n"));
        summary.push_str(&format!("Total References: {}\n", analysis.cross_references.len()));
        summary.push_str(&format!("Unique Documents: {}\n", self.count_unique_documents(analysis)));
        Ok(summary)
    }

    fn count_unique_documents(&self, analysis: &CrossReferenceAnalysis) -> usize {
        use std::collections::HashSet;
        let mut unique_paths = HashSet::new();
        
        for cross_ref in &analysis.cross_references {
            unique_paths.insert(cross_ref.referencing_doc.path.clone());
            unique_paths.insert(cross_ref.referenced_doc.path.clone());
        }
        
        unique_paths.len()
    }
} 