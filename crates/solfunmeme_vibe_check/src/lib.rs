use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_clifford::encoder::BertCliffordEncoder;
use solfunmeme_indexer::indexer::SearchIndex;
use clap::Parser;
use std::path::PathBuf;
use std::collections::HashMap;
use log::{info, error};
use tclifford::Multivector;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct VibeCheckArgs {
    /// The root directory of the codebase to analyze.
    #[clap(long, default_value = ".")]
    pub path: PathBuf,

    /// Path to the semantic ontology file.
    #[clap(long, default_value = "ontologies/zos/v1.ttl")]
    pub ontology: PathBuf,

    /// The maximum number of top terms to extract.
    #[clap(long, default_value = "100")]
    pub limit: usize,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct VibeCheckReport {
    pub terms: Vec<VibeCheckTerm>,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct VibeCheckTerm {
    pub term: String,
    pub emoji: Option<String>,
    pub clifford_vector: Option<Vec<f32>>,
    pub vibe_consistency_score: f32,
    pub suggestions: Vec<String>,
}

pub fn vibe_check(args: VibeCheckArgs) -> Result<VibeCheckReport, Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting vibe check with args: {:?}", args);

    let mut report = VibeCheckReport::default();

    // 1. Extract Top Terms (Mock for now, will integrate with solfunmeme_indexer)
    let mock_terms = vec!["code", "math", "manifold", "vibe", "rust", "ontology", "emoji"];

    // Load ontology
    let ontology_path = args.ontology;
    let mut ontology_graph = RdfGraph::from_file(&ontology_path)?;
    ontology_graph.namespaces.add_namespace("term", "http://solfunmeme.com/terms#")?;
    ontology_graph.namespaces.add_namespace("emoji", "https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#emoji")?;


    // Initialize Clifford encoder (mock for now)
    let encoder = BertCliffordEncoder::new();

    for term_str in mock_terms {
        let mut term_info = VibeCheckTerm {
            term: term_str.to_string(),
            ..Default::default()
        };

        // 2. Emoji Mapping
        let term_iri = ontology_graph.namespaces.get_term("term", term_str)?;
        let emoji_prop = ontology_graph.namespaces.get_term("emoji", "emoji")?;
        term_info.emoji = ontology_graph.get_object_literal(&term_iri, &emoji_prop)?;


        // 3. Hyperspace Location (Mock embedding for now)
        let mock_embedding = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]; // Dummy 8D vector
        let clifford_multivector = encoder.encode(&mock_embedding);
        term_info.clifford_vector = Some(clifford_multivector.into_iter().collect());

        // 4. Vibe Validation (Placeholder)
        term_info.vibe_consistency_score = 0.5; // Dummy score
        term_info.suggestions.push("Consider refining ontology mapping.".to_string());

        report.terms.push(term_info);
    }

    info!("Vibe check completed.");
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, TempDir};
    use std::fs;

    fn setup_mock_codebase(dir: &TempDir) {
        fs::create_dir_all(dir.path().join("src")).unwrap();
        fs::write(
            dir.path().join("src/main.rs"),
            r#"
fn main() {
    // This is some code about math and vibes.
    let x = 1 + 2; // code
    let y = x * 3; // math
    // A manifold of vibes.
}
            "#,
        )
        .unwrap();
    }

    fn setup_mock_ontology(dir: &TempDir) {
        fs::create_dir_all(dir.path().join("ontologies/zos")).unwrap();
        fs::write(
            dir.path().join("ontologies/zos/v1.ttl"),
            r#"
@prefix vibe: <https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#vibe> .
@prefix em: <https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#emoji> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix term: <http://solfunmeme.com/terms#> .

term:code em:emoji "U+1F4BB" .
term:math em:emoji "U+1F4C8" .
term:vibe em:emoji "U+1F4A3" .
            "#,
        )
        .unwrap();
    }

    #[test]
    fn test_vibe_check_basic_functionality() {
        let temp_dir = tempdir().unwrap();
        setup_mock_codebase(&temp_dir);
        setup_mock_ontology(&temp_dir);

        let args = VibeCheckArgs {
            path: temp_dir.path().to_path_buf(),
            ontology: temp_dir.path().join("ontologies/zos/v1.ttl"),
            limit: 10,
        };

        let report = vibe_check(args).unwrap();

        assert!(!report.terms.is_empty());

        // Check for specific terms and their properties
        let code_term = report.terms.iter().find(|t| t.term == "code").unwrap();
        assert_eq!(code_term.emoji, Some("U+1F4BB".to_string()));
        assert!(code_term.clifford_vector.is_some());

        let math_term = report.terms.iter().find(|t| t.term == "math").unwrap();
        assert_eq!(math_term.emoji, Some("U+1F4C8".to_string()));
        assert!(math_term.clifford_vector.is_some());

        let vibe_term = report.terms.iter().find(|t| t.term == "vibe").unwrap();
        assert_eq!(vibe_term.emoji, Some("U+1F4A3".to_string()));
        assert!(vibe_term.clifford_vector.is_some());

        // Ensure terms without direct emoji mappings are still included
        let manifold_term = report.terms.iter().find(|t| t.term == "manifold").unwrap();
        assert!(manifold_term.emoji.is_none());
        assert!(manifold_term.clifford_vector.is_some());
    }
}