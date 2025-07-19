use crate::rdf_graph::RdfGraph;
use sophia_api::serializer::{TripleSerializer, StreamSerializer};
use sophia_api::term::Iri;
use sophia_turtle::serializer::turtle::{TurtleConfig, TurtleSerializer};
use sophia_turtle::parser::turtle::TurtleParser;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

impl RdfGraph {
    pub fn from_jsonld_file(path: &Path) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let jsonld_data: serde_json::Value = serde_json::from_reader(reader)?;
        let mut parser = sophia_jsonld::JsonLdParser::new();
        let graph: sophia_inmem::graph::FastGraph = parser.parse_json(jsonld_data.to_string().as_bytes()).collect_triples::<sophia_inmem::graph::FastGraph>()?;
        Ok(RdfGraph {
            graph,
            namespaces: crate::namespace_manager::NamespaceManager::new(),
        })
    }

    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let graph: sophia_inmem::graph::FastGraph = TurtleParser::new(content.as_bytes(), None)
            .collect_triples::<sophia_inmem::graph::FastGraph>()?;
        Ok(RdfGraph {
            graph,
            namespaces: crate::namespace_manager::NamespaceManager::new(),
        })
    }

    pub fn serialize_to_turtle(&self, output_path: &Path) -> anyhow::Result<()> {
        let mut config = TurtleConfig::new().with_pretty(true);
        let mut prefix_map = Vec::new();

        for (prefix, iri) in self.namespaces.get_all_namespaces().iter() {
            prefix_map.push((
                sophia_api::prefix::Prefix::new_unchecked(prefix.clone().into()),
                Iri::new_unchecked(iri.as_str().into()),
            ));
        }

        config = config.with_own_prefix_map(prefix_map);

        let mut writer =
            TurtleSerializer::new_with_config(BufWriter::new(File::create(output_path)?), config);

        writer.serialize_graph(&self.graph)?;

        Ok(())
    }

    pub fn serialize_to_turtle_string(&self) -> anyhow::Result<String> {
        let mut buffer = Vec::new();
        let mut writer = BufWriter::new(&mut buffer);
        let mut config = TurtleConfig::new().with_pretty(true);

        for (prefix, iri) in self.namespaces.get_all_namespaces().iter() {
            config = config.with_prefix(sophia_api::prefix::Prefix::new_unchecked(prefix.clone().into()), Iri::new_unchecked(iri.as_str().into()))?;
        }

        let mut serializer = TurtleSerializer::new_with_config(&mut writer, config);
        serializer.serialize_graph(&self.graph)?;
        serializer.flush()?;

        Ok(String::from_utf8(buffer)?)
    }

    pub fn add_turtle_str(&mut self, turtle_data: &str) -> anyhow::Result<()> {
        TurtleParser::new(turtle_data.as_bytes(), None).in_graph(&mut self.graph)?;
        Ok(())
    }
}
