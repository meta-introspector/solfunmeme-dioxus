use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use std::path::PathBuf;

pub fn load_graph_internal() -> Result<RdfGraph> {
    let index_ttl_path = PathBuf::from("ontologies/index.ttl");
    let zos_v1_ttl_path = PathBuf::from("ontologies/zos/v1.ttl");

    let mut graph = RdfGraph::from_file(&index_ttl_path)?;
    let graph2 = RdfGraph::from_file(&zos_v1_ttl_path)?;

    graph.merge_graph(graph2)?;

    Ok(graph)
}
