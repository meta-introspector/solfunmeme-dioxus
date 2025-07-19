pub mod loader;
pub mod processor;
pub mod serializer;

use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;

pub fn load_graph() -> Result<RdfGraph> {
    loader::load_graph_internal()
}

pub fn add_crate_data(graph: &mut RdfGraph) -> Result<()> {
    processor::crate_data::add_crate_data_internal(graph)
}

pub fn add_emoji_data(graph: &mut RdfGraph) -> Result<()> {
    processor::emoji_data::add_emoji_data_internal(graph)
}

pub fn serialize_graph(graph: &RdfGraph) -> Result<()> {
    serializer::serialize_graph_internal(graph)
}