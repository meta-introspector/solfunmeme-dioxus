use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use std::path::PathBuf;

pub fn serialize_graph_internal(graph: &RdfGraph) -> Result<()> {
    let index_ttl_path = PathBuf::from("ontologies/index.ttl");
    graph.serialize_to_turtle(&index_ttl_path)
}
