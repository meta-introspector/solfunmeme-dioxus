use sophia_api::serializer::TripleSerializer;
use sophia_turtle::serializer::turtle::{TurtleSerializer, TurtleConfig};
use std::io::{BufWriter};
use std::fs::File;
use sophia::api::graph::Graph;
use sophia_api::prelude::IriRef;
use std::path::Path;
use sophia_api::prefix::{Prefix, PrefixMap};
use sophia_iri::Iri;

pub fn serialize_graph_to_file<G>(
    graph: &G,
    output_path: &Path,
    ex_iri: &IriRef<String>,
    rdf_iri: &IriRef<String>,
    rdfs_iri: &IriRef<String>,
    em_iri: &IriRef<String>,
) -> anyhow::Result<()>
where
    G: Graph,
    <G as Graph>::Error: Send + Sync + 'static,
{
    let mut prefix_map = TurtleConfig::default_prefix_map();
    prefix_map.push((Prefix::new_unchecked("ex".into()), Iri::new_unchecked(ex_iri.as_str().into())));
    prefix_map.push((Prefix::new_unchecked("rdf".into()), Iri::new_unchecked(rdf_iri.as_str().into())));
    prefix_map.push((Prefix::new_unchecked("rdfs".into()), Iri::new_unchecked(rdfs_iri.as_str().into())));
    prefix_map.push((Prefix::new_unchecked("em".into()), Iri::new_unchecked(em_iri.as_str().into())));

    let config = TurtleConfig::new()
        .with_pretty(true)
        .with_own_prefix_map(prefix_map);

    let mut writer = TurtleSerializer::new_with_config(BufWriter::new(File::create(output_path)?), config);

    writer.serialize_graph(&graph)?;

    Ok(())
}
