use serde_json::Value;
use sophia_api::graph::Graph;
use sophia_jsonld::loader::FsLoader;
use sophia_jsonld::JsonLdParser;
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{BufReader, BufWriter};

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

pub fn parse_jsonld_to_graph(jsonld_data: Value) -> Result<impl Graph> {
    let mut parser = JsonLdParser::new(FsLoader::new());
    let graph = parser.parse_json_ld(jsonld_data)?;
    Ok(graph)
}
