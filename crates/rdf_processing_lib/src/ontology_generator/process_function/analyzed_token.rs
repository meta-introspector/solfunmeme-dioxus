use crate::ontology_generator::namespaces::Namespaces;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_rdf_utils::term_factory;

pub struct AnalyzedToken {
    pub token: String,
    pub count: u32,
    pub multivector_str: String,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

pub fn process_analyzed_token(
    graph: &mut RdfGraph,
    token_data: AnalyzedToken,
    ns: &Namespaces,
) -> anyhow::Result<()> {
    let token_iri = format!("{}token/{}", ns.ex.get_base_iri("ex").unwrap(), token_data.token);

    graph.add_triple(&token_iri, &ns.rdf.get_term("rdf", "type")?.to_string(), &ns.ex.get_term("ex", "Token")?.to_string())?;
    graph.add_literal_triple(&token_iri, &ns.ex.get_term("ex", "hasToken")?.to_string(), &token_data.token, &ns.xsd.get_term("xsd", "string")?.to_string())?;
    graph.add_literal_triple(&token_iri, &ns.ex.get_term("ex", "hasCount")?.to_string(), &token_data.count.to_string(), &ns.xsd.get_term("xsd", "integer")?.to_string())?;
    graph.add_literal_triple(&token_iri, &ns.ex.get_term("ex", "hasMultivector")?.to_string(), &token_data.multivector_str, &ns.xsd.get_term("xsd", "string")?.to_string())?;

    Ok(())
}
