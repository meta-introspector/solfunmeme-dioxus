use crate::rdf_graph::RdfGraph;
use sophia_api::term::SimpleTerm;
use sophia_api::graph::Graph;

impl RdfGraph {
    pub fn get_object_literal(
        &self,
        subject: &SimpleTerm,
        predicate: &SimpleTerm,
    ) -> anyhow::Result<Option<String>> {
        if let Some(t) = self.graph.triples_with_sp(subject, predicate).next() {
            let t = t?;
            if let Some(literal) = t.o().as_literal() {
                return Ok(Some(literal.value().to_string()));
            }
        }
        Ok(None)
    }

    pub fn get_subjects_with_property(
        &self,
        predicate: &SimpleTerm,
        object: &SimpleTerm,
    ) -> anyhow::Result<Vec<SimpleTerm>> {
        Ok(self
            .graph
            .triples_with_po(predicate, object)
            .map(|t| t.map(|t| t.s().clone()))
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub fn get_property_value(
        &self,
        subject: &SimpleTerm,
        predicate: &SimpleTerm,
    ) -> anyhow::Result<Option<String>> {
        if let Some(t) = self.graph.triples_with_sp(subject, predicate).next() {
            let t = t?;
            let o = t.o();
            if let Some(iri) = o.as_iri() {
                return Ok(Some(iri.as_str().to_string()));
            } else if let Some(literal) = o.as_literal() {
                return Ok(Some(literal.value().to_string()));
            }
        }
        Ok(None)
    }
}
