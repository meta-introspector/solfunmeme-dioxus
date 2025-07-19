use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use std::path::Path;

pub fn serialize_graph_to_file(
    graph: &RdfGraph,
    output_path: &Path,
) -> anyhow::Result<()> {
    graph.serialize_to_turtle(output_path)
}