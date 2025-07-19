use anyhow::Result;
use serde_json::Value;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn read_jsonld_from_file(path: &str) -> Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let value: Value = serde_json::from_reader(reader)?;
    Ok(value)
}

pub fn write_jsonld_to_file(path: &str, data: &Value) -> Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data)?;
    Ok(())
}

pub fn parse_jsonld_to_graph(path: &str) -> Result<RdfGraph> {
    RdfGraph::from_jsonld_file(Path::new(path))
}