use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use std::path::Path;

pub fn read_and_parse_triples(ontology_path: &str) -> anyhow::Result<RdfGraph> {
    RdfGraph::from_file(Path::new(ontology_path))
}