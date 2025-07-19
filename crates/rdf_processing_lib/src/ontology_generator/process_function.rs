use solfunmeme_rdf_utils::rdf_graph::{RdfGraph, term_to_string};
use solfunmeme_rdf_utils::namespace_manager::NamespaceManager;

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

pub struct ClosestEmojiInfo {
    pub emoji: String,
    pub category: String,
    pub distance: f32,
}

pub struct AnalyzedToken {
    pub token: String,
    pub count: u32,
    pub multivector_str: String,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

pub fn process_analyzed_function(
    graph: &mut RdfGraph,
    func: AnalyzedFunction,
    ns: &NamespaceManager,
) -> anyhow::Result<()> {
    let func_iri = ns.get_term("ex", &func.function_name)?;
    let type_pred = ns.get_term("rdf", "type")?;
    let func_class = ns.get_term("ex", "Function")?;
    graph.add_triple(term_to_string(&func_iri).as_str(), term_to_string(&type_pred).as_str(), term_to_string(&func_class).as_str())?;

    let file_path_pred = ns.get_term("ex", "filePath")?;
    graph.add_literal_triple(
        term_to_string(&func_iri).as_str(),
        term_to_string(&file_path_pred).as_str(),
        &func.file_path,
        ns.get_base_iri("xsd").unwrap().as_str(),
    )?;

    let code_snippet_pred = ns.get_term("ex", "codeSnippet")?;
    graph.add_literal_triple(
        term_to_string(&func_iri).as_str(),
        term_to_string(&code_snippet_pred).as_str(),
        &func.code_snippet,
        ns.get_base_iri("xsd").unwrap().as_str(),
    )?;

    let semantic_summary_pred = ns.get_term("ex", "semanticSummary")?;
    graph.add_literal_triple(
        term_to_string(&func_iri).as_str(),
        term_to_string(&semantic_summary_pred).as_str(),
        &func.semantic_summary,
        ns.get_base_iri("xsd").unwrap().as_str(),
    )?;

    let multivector_pred = ns.get_term("ex", "multivector")?;
    graph.add_literal_triple(
        term_to_string(&func_iri).as_str(),
        term_to_string(&multivector_pred).as_str(),
        &func.multivector_str,
        ns.get_base_iri("xsd").unwrap().as_str(),
    )?;

    let sieve_address_pred = ns.get_term("ex", "sieveAddress")?;
    graph.add_literal_triple(
        term_to_string(&func_iri).as_str(),
        term_to_string(&sieve_address_pred).as_str(),
        &func.sieve_address,
        ns.get_base_iri("xsd").unwrap().as_str(),
    )?;

    for emoji_info in func.closest_emojis {
        let emoji_pred = ns.get_term("em", &emoji_info.emoji)?;
        let emoji_dist_pred = ns.get_term("ex", "emojiDistance")?;
        let emoji_cat_pred = ns.get_term("ex", "emojiCategory")?;

        let emoji_bnode = graph.new_bnode()?;
        graph.add_triple(term_to_string(&func_iri).as_str(), term_to_string(&emoji_pred).as_str(), term_to_string(&emoji_bnode).as_str())?;

        graph.add_literal_triple(
            term_to_string(&emoji_bnode).as_str(),
            term_to_string(&emoji_dist_pred).as_str(),
            &emoji_info.distance.to_string(),
            ns.get_base_iri("xsd").unwrap().as_str(),
        )?;

        graph.add_literal_triple(
            term_to_string(&emoji_bnode).as_str(),
            term_to_string(&emoji_cat_pred).as_str(),
            &emoji_info.category,
            ns.get_base_iri("xsd").unwrap().as_str(),
        )?;
    }

    Ok(())
}

pub fn process_analyzed_token(
    graph: &mut RdfGraph,
    token_data: AnalyzedToken,
    ns: &NamespaceManager,
) -> anyhow::Result<()> {
    let token_iri = ns.get_term("ex", &token_data.token)?;
    let type_pred = ns.get_term("rdf", "type")?;
    let token_class = ns.get_term("ex", "Token")?;
    graph.add_triple(term_to_string(&token_iri).as_str(), term_to_string(&type_pred).as_str(), term_to_string(&token_class).as_str())?;

    let count_pred = ns.get_term("ex", "count")?;
    graph.add_literal_triple(
        term_to_string(&token_iri).as_str(),
        term_to_string(&count_pred).as_str(),
        &token_data.count.to_string(),
        ns.get_base_iri("xsd").unwrap().as_str(),
    )?;

    let multivector_pred = ns.get_term("ex", "multivector")?;
    graph.add_literal_triple(
        term_to_string(&token_iri).as_str(),
        term_to_string(&multivector_pred).as_str(),
        &token_data.multivector_str,
        ns.get_base_iri("xsd").unwrap().as_str(),
    )?;

    Ok(())
}