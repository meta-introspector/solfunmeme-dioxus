use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_rdf_utils::term_factory;
use solfunmeme_rdf_utils::sophia_api::term::TTerm;
use solfunmeme_rdf_utils::sophia_api::prelude::Triple;

pub struct OntologyResolver {
    graph: RdfGraph,
    uri_to_emoji: HashMap<String, String>,
}

impl OntologyResolver {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let graph = RdfGraph::from_file(path)?;

        let mut uri_to_emoji = HashMap::new();
        let emoji_prop_uri = term_factory::iri_term("https://rdf.solfunmeme.com/spec/2025/07/17/emoji.ttl#emoji")?;

        for triple in graph.graph.triples() {
            let triple = triple?;
            if triple.p() == &emoji_prop_uri {
                let subject_str = if let Some(iri) = triple.s().as_iri() {
                    iri.as_str().to_string()
                } else if let Some(lit) = triple.s().as_literal() {
                    lit.value().to_string()
                } else {
                    continue; // Skip other term types
                };
                let object_str = if let Some(iri) = triple.o().as_iri() {
                    iri.as_str().to_string()
                } else if let Some(lit) = triple.o().as_literal() {
                    lit.value().to_string()
                } else {
                    continue; // Skip other term types
                };
                uri_to_emoji.insert(subject_str, object_str);
            }
        }

        Ok(OntologyResolver { graph, uri_to_emoji })
    }

    pub fn resolve_emoji(&self, concept_uri: &str) -> Option<&String> {
        self.uri_to_emoji.get(concept_uri)
    }
}
