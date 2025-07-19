use crate::ontology_generator::namespaces::Namespaces;
use crate::ontology_generator::closest_emoji_info::ClosestEmojiInfo;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_rdf_utils::term_factory;

pub struct AnalyzedFunction {
    pub function_name: String,
    pub file_path: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emojis: Vec<ClosestEmojiInfo>,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

pub fn process_analyzed_function(
    graph: &mut RdfGraph,
    func: AnalyzedFunction,
    ns: &Namespaces,
) -> anyhow::Result<()> {
    let func_iri = format!("{}function/{}", ns.ex.get_base_iri("ex").unwrap(), func.function_name);

    graph.add_triple(&func_iri, &ns.rdf.get_term("rdf", "type")?.to_string(), &ns.ex.get_term("ex", "Function")?.to_string())?;
    graph.add_literal_triple(&func_iri, &ns.ex.get_term("ex", "hasName")?.to_string(), &func.function_name, &ns.xsd.get_term("xsd", "string")?.to_string())?;
    graph.add_literal_triple(&func_iri, &ns.ex.get_term("ex", "hasFilePath")?.to_string(), &func.file_path, &ns.xsd.get_term("xsd", "string")?.to_string())?;
    graph.add_literal_triple(&func_iri, &ns.ex.get_term("ex", "hasCodeSnippet")?.to_string(), &func.code_snippet, &ns.xsd.get_term("xsd", "string")?.to_string())?;
    graph.add_literal_triple(&func_iri, &ns.ex.get_term("ex", "hasSemanticSummary")?.to_string(), &func.semantic_summary, &ns.xsd.get_term("xsd", "string")?.to_string())?;
    graph.add_literal_triple(&func_iri, &ns.ex.get_term("ex", "hasMultivector")?.to_string(), &func.multivector_str, &ns.xsd.get_term("xsd", "string")?.to_string())?;
    graph.add_literal_triple(&func_iri, &ns.ex.get_term("ex", "hasSieveAddress")?.to_string(), &func.sieve_address, &ns.xsd.get_term("xsd", "string")?.to_string())?;

    for emoji_info in func.closest_emojis {
        let emoji_iri = format!("{}emoji/{}", ns.em.get_base_iri("em").unwrap(), emoji_info.emoji.replace(" ", "_"));
        graph.add_triple(&func_iri, &ns.ex.get_term("ex", "hasClosestEmoji")?.to_string(), &emoji_iri)?;
        graph.add_literal_triple(&emoji_iri, &ns.ex.get_term("ex", "hasEmoji")?.to_string(), &emoji_info.emoji, &ns.xsd.get_term("xsd", "string")?.to_string())?;
        graph.add_literal_triple(&emoji_iri, &ns.ex.get_term("ex", "hasDistance")?.to_string(), &emoji_info.distance.to_string(), &ns.xsd.get_term("xsd", "double")?.to_string())?;
    }

    Ok(())
}
