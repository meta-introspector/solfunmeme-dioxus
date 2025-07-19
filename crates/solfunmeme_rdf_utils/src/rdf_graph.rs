use crate::namespace_manager::NamespaceManager;
use sophia_api::graph::{MutableGraph, Graph};
use sophia_inmem::graph::FastGraph;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use sophia_turtle::parser::turtle::TurtleParser;
use sophia_api::parser::TripleParser;
use sophia_api::prelude::{TripleSource, Triple, TermKind, Term};
use sophia_api::term::SimpleTerm;
use std::path::Path;
use crate::term_factory; // Import the new term_factory
use sophia_api::serializer::TripleSerializer;
use uuid;


#[derive(Clone, Debug)]
pub struct RdfGraph {
    pub graph: FastGraph,
    pub namespaces: NamespaceManager,
}

impl RdfGraph {
    pub fn from_turtle_str(turtle_data: &str) -> anyhow::Result<Self> {
        let parser = TurtleParser { base: None };
        let graph: FastGraph = parser.parse_str(turtle_data).collect_triples()?;
        Ok(RdfGraph {
            graph,
            namespaces: NamespaceManager::new(),
        })
    }

    pub fn new() -> Self {
        RdfGraph {
            graph: FastGraph::new(),
            namespaces: NamespaceManager::new(),
        }
    }

    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let parser = TurtleParser { base: None };
        let graph: FastGraph = parser.parse_str(&content).collect_triples()?;
        Ok(RdfGraph {
            graph,
            namespaces: NamespaceManager::new(),
        })
    }

    pub fn add_triple(
        &mut self,
        subject_iri: &str,
        predicate_iri: &str,
        object_iri: &str,
    ) -> anyhow::Result<()> {
        let subject = term_factory::iri_term(subject_iri.to_string())?;
        let predicate = term_factory::iri_term(predicate_iri.to_string())?;
        let object = term_factory::iri_term(object_iri.to_string())?;
        self.graph.insert(&subject, &predicate, &object)?;
        Ok(())
    }

    pub fn add_literal_triple(
        &mut self,
        subject_iri: &str,
        predicate_iri: &str,
        literal_value: &str,
        literal_type_iri: &str,
    ) -> anyhow::Result<()> {
        let subject = term_factory::iri_term(subject_iri.to_string())?;
        let predicate = term_factory::iri_term(predicate_iri.to_string())?;
        let literal = term_factory::literal_term_typed(literal_value, literal_type_iri)?;
        self.graph.insert(&subject, &predicate, &literal)?;
        Ok(())
    }

    pub fn merge_graph(&mut self, other: RdfGraph) -> anyhow::Result<()> {
        self.graph.insert_all(other.graph.triples())?;
        Ok(())
    }

    pub fn get_all_triples_as_strings(&self) -> Vec<(String, String, String)> {
        let mut results = Vec::new();
        for t in self.graph.triples() {
            let t = t.unwrap();
            results.push((term_to_string(t.s()), term_to_string(t.p()), term_to_string(t.o())));
        }
        results
    }

    pub fn query_triples_as_strings(
        &self,
        subj: Option<&str>,
        pred: Option<&str>,
        obj: Option<&str>,
    ) -> Vec<(String, String, String)> {
        let mut results = Vec::new();
        for t in self.graph.triples() {
            let t = t.unwrap();
            let s_str = term_to_string(t.s());
            let p_str = term_to_string(t.p());
            let o_str = term_to_string(t.o());

            let s_match = subj.map_or(true, |s_| s_str == s_);
            let p_match = pred.map_or(true, |p_| p_str == p_);
            let o_match = obj.map_or(true, |o_| o_str == o_);

            if s_match && p_match && o_match {
                results.push((s_str, p_str, o_str));
            }
        }
        results
    }

    pub fn get_subjects_with_property(
        &self,
        predicate_iri: &str,
        object_iri: &str,
    ) -> anyhow::Result<Vec<String>> {
        let predicate = term_factory::iri_term(predicate_iri.to_string())?;
        let object = term_factory::iri_term(object_iri.to_string())?;
        let mut subjects = Vec::new();
        for t in self.graph.triples_matching(None::<&SimpleTerm>, Some(&predicate), Some(&object)) {
            subjects.push(term_to_string(t.unwrap().s()));
        }
        Ok(subjects)
    }

    pub fn get_property_value(
        &self,
        subject_iri: &str,
        predicate_iri: &str,
    ) -> anyhow::Result<Option<String>> {
        let subject = term_factory::iri_term(subject_iri.to_string())?;
        let predicate = term_factory::iri_term(predicate_iri.to_string())?;
        for t in self.graph.triples_matching(Some(&subject), Some(&predicate), None::<&SimpleTerm>) {
            return Ok(Some(term_to_string(t.unwrap().o())));
        }
        Ok(None)
    }

    pub fn new_bnode(&mut self) -> anyhow::Result<SimpleTerm<'static>> {
        let id = uuid::Uuid::new_v4().to_string();
        term_factory::bnode_term(id)
    }

    pub fn query_graph_triples(
        &self,
        subj_iri: Option<&str>,
        pred_iri: Option<&str>,
        obj_iri: Option<&str>,
    ) -> Vec<(String, String, String)> {
        let s_term = subj_iri.and_then(|s| term_factory::iri_term(s.to_string()).ok());
        let p_term = pred_iri.and_then(|p| term_factory::iri_term(p.to_string()).ok());
        let o_term = obj_iri.and_then(|o| term_factory::iri_term(o.to_string()).ok());

        let mut results = Vec::new();
        for t in self.graph.triples_matching(s_term.as_ref(), p_term.as_ref(), o_term.as_ref()) {
            let t = t.unwrap();
            results.push((term_to_string(t.s()), term_to_string(t.p()), term_to_string(t.o())));
        }
        results
    }

    pub fn serialize_to_turtle(&self, path: &Path) -> anyhow::Result<()> {
        let mut buffer = Vec::new();
        let mut serializer = TurtleSerializer::new(&mut buffer);
        serializer.serialize_graph(&self.graph)?;
        std::fs::write(path, buffer)?;
        Ok(())
    }

    pub fn serialize_to_turtle_string(&self) -> anyhow::Result<String> {
        let mut buffer = Vec::new();
        let mut serializer = TurtleSerializer::new(&mut buffer);
        serializer.serialize_graph(&self.graph)?;
        Ok(String::from_utf8(buffer)?)
    }
}

pub fn term_to_string(term: &SimpleTerm) -> String {
    match term.kind() {
        TermKind::Iri => term.iri().unwrap().to_string(),
        TermKind::Literal => term.lexical_form().unwrap().to_string(),
        TermKind::BlankNode => term.bnode_id().unwrap().to_string(),
        _ => format!("{:?}", term), // Fallback for other term kinds
    }
}

pub struct GraphBuilder {
    graph: RdfGraph,
}

impl GraphBuilder {
    pub fn new() -> Self {
        GraphBuilder {
            graph: RdfGraph::new(),
        }
    }

    pub fn with_namespace(mut self, prefix: &str, iri: &str) -> anyhow::Result<Self> {
        self.graph.namespaces.add_namespace(prefix, iri)?;
        Ok(self)
    }

    pub fn add_triple(mut self, subject_iri: &str, predicate_iri: &str, object_iri: &str) -> anyhow::Result<Self> {
        self.graph.add_triple(subject_iri, predicate_iri, object_iri)?;
        Ok(self)
    }

    pub fn add_literal_triple(
        mut self,
        subject_iri: &str,
        predicate_iri: &str,
        literal_value: &str,
        literal_type_iri: &str,
    ) -> anyhow::Result<Self> {
        self.graph.add_literal_triple(subject_iri, predicate_iri, literal_value, literal_type_iri)?;
        Ok(self)
    }

    pub fn build(self) -> RdfGraph {
        self.graph
    }
}