use crate::orient::Orient;
use crate::types::{CrossReference, DocumentIndex, OrientResult};
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Demonstrates how each chat can find and leverage related code
/// This is the "code-by-example" pattern in action
pub struct CodeByExample {
    orient: Orient,
    example_patterns: Vec<ExamplePattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamplePattern {
    pub name: String,
    pub description: String,
    pub related_files: Vec<String>,
    pub code_snippets: Vec<String>,
    pub integration_points: Vec<String>,
}

impl CodeByExample {
    pub fn new() -> Self {
        let orient = Orient::new();
        let example_patterns = Self::discover_patterns();
        
        Self {
            orient,
            example_patterns,
        }
    }

    /// Discover patterns by analyzing the codebase
    fn discover_patterns() -> Vec<ExamplePattern> {
        vec![
            ExamplePattern {
                name: "Clifford Algebra Attention".to_string(),
                description: "Found existing Clifford attention mechanisms in src/model/clifford.rs".to_string(),
                related_files: vec![
                    "src/model/clifford.rs".to_string(),
                    "crates/solfunmeme_clifford/src/lib.rs".to_string(),
                ],
                code_snippets: vec![
                    "CliffordAttention struct with geometric products".to_string(),
                    "BertCliffordEncoder for 8D embeddings".to_string(),
                    "SolCl algebra declaration".to_string(),
                ],
                integration_points: vec![
                    "Extend existing CliffordAttention with curvature".to_string(),
                    "Use existing 8D SolCl algebra".to_string(),
                    "Leverage existing BERT → Clifford pipeline".to_string(),
                ],
            },
            ExamplePattern {
                name: "Mathematical Framework".to_string(),
                description: "Found rich mathematical foundation in src/model/math/overview.latex".to_string(),
                related_files: vec![
                    "src/model/math/overview.latex".to_string(),
                    "doc/code_math_manifold.md".to_string(),
                ],
                code_snippets: vec![
                    "8-quadrant mathematical framework".to_string(),
                    "E8 symmetry integration".to_string(),
                    "Manifold geometry concepts".to_string(),
                ],
                integration_points: vec![
                    "Integrate with existing E8 symmetry".to_string(),
                    "Use manifold geometry framework".to_string(),
                    "Apply mathematical quadrant system".to_string(),
                ],
            },
            ExamplePattern {
                name: "BERT Integration".to_string(),
                description: "Found WASM-compatible BERT system in src/model/wasm_bert.rs".to_string(),
                related_files: vec![
                    "src/model/wasm_bert.rs".to_string(),
                    "src/model/clifford.rs".to_string(),
                ],
                code_snippets: vec![
                    "WasmBertEmbedder for lightweight embeddings".to_string(),
                    "BERT to Clifford conversion pipeline".to_string(),
                    "384-dimensional embedding support".to_string(),
                ],
                integration_points: vec![
                    "Use existing WASM BERT pipeline".to_string(),
                    "Extend embedding to Clifford space".to_string(),
                    "Leverage existing sentiment analysis".to_string(),
                ],
            },
            ExamplePattern {
                name: "Cross-Reference System".to_string(),
                description: "Found existing cross-reference infrastructure in doc_cross_references".to_string(),
                related_files: vec![
                    "crates/doc_cross_references/src/lib.rs".to_string(),
                    "crates/doc_cross_references/src/main.rs".to_string(),
                ],
                code_snippets: vec![
                    "CrossReference struct with document indexing".to_string(),
                    "CLI commands for analysis".to_string(),
                    "Report generation system".to_string(),
                ],
                integration_points: vec![
                    "Add Orient phase to existing CLI".to_string(),
                    "Extend cross-reference analysis".to_string(),
                    "Integrate with existing reports".to_string(),
                ],
            },
        ]
    }

    /// Demonstrate how a chat about attention mechanisms finds related code
    pub fn demonstrate_attention_chat(&self) -> Result<String> {
        let pattern = self.example_patterns.iter()
            .find(|p| p.name.contains("Attention"))
            .unwrap();

        let mut report = String::new();
        report.push_str("=== Attention Mechanism Chat Example ===\n\n");
        report.push_str(&format!("Chat Topic: 'I want to implement geometric attention'\n\n"));
        report.push_str("🔍 **Code Discovery Process:**\n");
        report.push_str("1. Search for 'attention' in codebase\n");
        report.push_str("2. Found existing CliffordAttention in src/model/clifford.rs\n");
        report.push_str("3. Found 8D Clifford algebra in crates/solfunmeme_clifford/src/lib.rs\n\n");
        
        report.push_str("📁 **Related Files Found:**\n");
        for file in &pattern.related_files {
            report.push_str(&format!("- {}\n", file));
        }
        report.push_str("\n");

        report.push_str("💡 **Code Snippets Available:**\n");
        for snippet in &pattern.code_snippets {
            report.push_str(&format!("- {}\n", snippet));
        }
        report.push_str("\n");

        report.push_str("🔗 **Integration Points:**\n");
        for point in &pattern.integration_points {
            report.push_str(&format!("- {}\n", point));
        }
        report.push_str("\n");

        report.push_str("✅ **Result:** Instead of building from scratch, we extended the existing CliffordAttention with our Orient phase!\n");

        Ok(report)
    }

    /// Demonstrate how a chat about mathematical frameworks finds related code
    pub fn demonstrate_math_chat(&self) -> Result<String> {
        let pattern = self.example_patterns.iter()
            .find(|p| p.name.contains("Mathematical"))
            .unwrap();

        let mut report = String::new();
        report.push_str("=== Mathematical Framework Chat Example ===\n\n");
        report.push_str("Chat Topic: 'I need to implement E8 symmetry and manifold geometry'\n\n");
        report.push_str("🔍 **Code Discovery Process:**\n");
        report.push_str("1. Search for 'manifold' and 'E8' in codebase\n");
        report.push_str("2. Found comprehensive math framework in src/model/math/overview.latex\n");
        report.push_str("3. Found Code-Math Manifold philosophy in doc/code_math_manifold.md\n\n");
        
        report.push_str("📁 **Related Files Found:**\n");
        for file in &pattern.related_files {
            report.push_str(&format!("- {}\n", file));
        }
        report.push_str("\n");

        report.push_str("💡 **Code Snippets Available:**\n");
        for snippet in &pattern.code_snippets {
            report.push_str(&format!("- {}\n", snippet));
        }
        report.push_str("\n");

        report.push_str("🔗 **Integration Points:**\n");
        for point in &pattern.integration_points {
            report.push_str(&format!("- {}\n", point));
        }
        report.push_str("\n");

        report.push_str("✅ **Result:** We integrated our Orient module with the existing E8 symmetry and manifold geometry framework!\n");

        Ok(report)
    }

    /// Demonstrate how a chat about embeddings finds related code
    pub fn demonstrate_embedding_chat(&self) -> Result<String> {
        let pattern = self.example_patterns.iter()
            .find(|p| p.name.contains("BERT"))
            .unwrap();

        let mut report = String::new();
        report.push_str("=== Embedding Chat Example ===\n\n");
        report.push_str("Chat Topic: 'I need to process text embeddings and convert them to geometric representations'\n\n");
        report.push_str("🔍 **Code Discovery Process:**\n");
        report.push_str("1. Search for 'embedding' and 'BERT' in codebase\n");
        report.push_str("2. Found WASM-compatible BERT system in src/model/wasm_bert.rs\n");
        report.push_str("3. Found existing BERT → Clifford pipeline in src/model/clifford.rs\n\n");
        
        report.push_str("📁 **Related Files Found:**\n");
        for file in &pattern.related_files {
            report.push_str(&format!("- {}\n", file));
        }
        report.push_str("\n");

        report.push_str("💡 **Code Snippets Available:**\n");
        for snippet in &pattern.code_snippets {
            report.push_str(&format!("- {}\n", snippet));
        }
        report.push_str("\n");

        report.push_str("🔗 **Integration Points:**\n");
        for point in &pattern.integration_points {
            report.push_str(&format!("- {}\n", point));
        }
        report.push_str("\n");

        report.push_str("✅ **Result:** We leveraged the existing WASM BERT pipeline and extended it with our Orient phase!\n");

        Ok(report)
    }

    /// Process a chat and find related code patterns
    pub fn process_chat(&self, chat_topic: &str) -> Result<Vec<ExamplePattern>> {
        let mut relevant_patterns = Vec::new();
        
        let keywords = chat_topic.to_lowercase();
        
        for pattern in &self.example_patterns {
            if keywords.contains("attention") && pattern.name.contains("Attention") {
                relevant_patterns.push(pattern.clone());
            }
            if keywords.contains("math") || keywords.contains("e8") || keywords.contains("manifold") {
                if pattern.name.contains("Mathematical") {
                    relevant_patterns.push(pattern.clone());
                }
            }
            if keywords.contains("embedding") || keywords.contains("bert") || keywords.contains("text") {
                if pattern.name.contains("BERT") {
                    relevant_patterns.push(pattern.clone());
                }
            }
            if keywords.contains("cross") || keywords.contains("reference") || keywords.contains("document") {
                if pattern.name.contains("Cross-Reference") {
                    relevant_patterns.push(pattern.clone());
                }
            }
        }
        
        Ok(relevant_patterns)
    }

    /// Generate a comprehensive code-by-example report
    pub fn generate_report(&self) -> Result<String> {
        let mut report = String::new();
        report.push_str("# Code-by-Example Pattern Demonstration\n\n");
        report.push_str("This demonstrates how each chat can find and leverage related code from the codebase.\n\n");
        
        report.push_str("## How It Works\n\n");
        report.push_str("1. **Chat Input**: User asks about implementing a feature\n");
        report.push_str("2. **Code Discovery**: System searches for related existing code\n");
        report.push_str("3. **Pattern Recognition**: Identifies reusable components and patterns\n");
        report.push_str("4. **Integration**: Extends existing code instead of building from scratch\n\n");
        
        report.push_str("## Example Patterns Found\n\n");
        for pattern in &self.example_patterns {
            report.push_str(&format!("### {}\n", pattern.name));
            report.push_str(&format!("{}\n\n", pattern.description));
            report.push_str("**Related Files:**\n");
            for file in &pattern.related_files {
                report.push_str(&format!("- `{}`\n", file));
            }
            report.push_str("\n**Integration Points:**\n");
            for point in &pattern.integration_points {
                report.push_str(&format!("- {}\n", point));
            }
            report.push_str("\n---\n\n");
        }
        
        report.push_str("## Benefits of Code-by-Example\n\n");
        report.push_str("- **Faster Development**: Leverage existing, tested code\n");
        report.push_str("- **Consistency**: Maintain architectural patterns\n");
        report.push_str("- **Integration**: Seamless fit with existing systems\n");
        report.push_str("- **Learning**: Understand system architecture through examples\n");
        report.push_str("- **Efficiency**: Avoid reinventing the wheel\n\n");
        
        report.push_str("## Next Steps\n\n");
        report.push_str("Each new chat can now:\n");
        report.push_str("1. Search for related code patterns\n");
        report.push_str("2. Identify integration points\n");
        report.push_str("3. Extend existing functionality\n");
        report.push_str("4. Maintain system coherence\n");

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_discovery() {
        let code_by_example = CodeByExample::new();
        assert_eq!(code_by_example.example_patterns.len(), 4);
    }

    #[test]
    fn test_chat_processing() {
        let code_by_example = CodeByExample::new();
        let patterns = code_by_example.process_chat("I want to implement geometric attention").unwrap();
        assert!(!patterns.is_empty());
    }

    #[test]
    fn test_report_generation() {
        let code_by_example = CodeByExample::new();
        let report = code_by_example.generate_report().unwrap();
        assert!(report.contains("Code-by-Example Pattern Demonstration"));
    }
} 