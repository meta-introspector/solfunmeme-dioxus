use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use std::path::Path;
use std::collections::HashMap;

mod namespaces;
pub mod process_function;
mod serialize;

pub fn generate_ontology(
    analyzed_functions: Vec<process_function::AnalyzedFunction>,
    output_path: &Path,
) -> Result<()> {
    let mut graph = RdfGraph::new();
    graph.namespaces = namespaces::define_namespaces();

    for func in analyzed_functions {
        let ns_clone = graph.namespaces.clone();
        process_function::process_analyzed_function(&mut graph, func, &ns_clone)?;
    }

    serialize::serialize_graph_to_file(&graph, output_path)?;

    Ok(())
}

pub fn generate_token_ontology(
    analyzed_tokens: HashMap<String, process_function::AnalyzedToken>,
    output_path: &Path,
) -> Result<()> {
    let mut graph = RdfGraph::new();
    graph.namespaces = namespaces::define_namespaces();

    for (_token_str, token_data) in analyzed_tokens {
        let ns_clone = graph.namespaces.clone();
        process_function::process_analyzed_token(&mut graph, token_data, &ns_clone)?;
    }

    serialize::serialize_graph_to_file(&graph, output_path)?;

    Ok(())
}
