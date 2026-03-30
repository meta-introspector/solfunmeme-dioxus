use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub path: PathBuf,
    pub content: String,
    pub title: Option<String>,
    pub references: Vec<String>,
}

impl Document {
    pub fn hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.path.hash(&mut hasher);
        hasher.finish()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReference {
    pub referencing_doc: Document,
    pub referenced_doc: Document,
    pub reference_type: ReferenceType,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceType {
    Link,
    Include,
    Import,
    Cite,
    SeeAlso,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIndex {
    pub documents: Vec<Document>,
}

impl DocumentIndex {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let index: DocumentIndex = serde_json::from_str(&content)?;
        Ok(index)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrientResult {
    pub original_vector: Vec<f32>,
    pub oriented_vector: Vec<f32>, // Simplified representation
    pub curvature: f32,
    pub sieve_address: String,
    pub orientation_confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReferenceAnalysis {
    pub cross_references: Vec<CrossReference>,
}

impl CrossReferenceAnalysis {
    pub fn new() -> Self {
        Self {
            cross_references: Vec::new(),
        }
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let analysis: CrossReferenceAnalysis = serde_json::from_str(&content)?;
        Ok(analysis)
    }

    pub fn get_references_to(&self, document: &str) -> Option<&[CrossReference]> {
        let refs: Vec<&CrossReference> = self.cross_references
            .iter()
            .filter(|r| r.referenced_doc.path.to_string_lossy().contains(document))
            .collect();
        
        if refs.is_empty() {
            None
        } else {
            // This is a simplified version - in practice you'd want to return owned data
            None
        }
    }

    pub fn find_broken_references(&self) -> Vec<&CrossReference> {
        self.cross_references
            .iter()
            .filter(|r| !r.referenced_doc.path.exists())
            .collect()
    }
} 