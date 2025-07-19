use crate::rdf_graph::RdfGraph;
use crate::term_factory;
use sophia_api::term::SimpleTerm;
use sophia_api::graph::MutableGraph;

impl RdfGraph {
    pub fn add_triple(
        &mut self,
        subject_iri: &str,
        predicate_iri: &str,
        object_iri: &str,
    ) -> anyhow::Result<()> {
        let subject = term_factory::iri_term(subject_iri)?;
        let predicate = term_factory::iri_term(predicate_iri)?;
        let object = term_factory::iri_term(object_iri)?;
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
        let subject = term_factory::iri_term(subject_iri)?;
        let predicate = term_factory::iri_term(predicate_iri)?;
        let literal = term_factory::literal_term_typed(literal_value, literal_type_iri)?;
        self.graph.insert(&subject, &predicate, &literal)?;
        Ok(())
    }

    pub fn new_bnode(&mut self) -> anyhow::Result<SimpleTerm> {
        Ok(self.graph.new_bnode())
    }
}
