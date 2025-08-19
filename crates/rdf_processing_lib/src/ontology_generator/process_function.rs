use sophia::api::graph::MutableGraph;
use sophia_api::prelude::{IriRef, Term};
use sophia_api::term::SimpleTerm;

use shared_analysis_types::{AnalyzedFunction, ClosestEmojiInfo, AnalyzedToken};
use super::namespaces::Namespaces;

pub fn process_analyzed_function<G>(
    graph: &mut G,
    func: AnalyzedFunction,
    ns: &Namespaces,
) -> anyhow::Result<()>
where
    G: MutableGraph,
    <G as MutableGraph>::MutationError: Send + Sync + 'static,
{
    let func_iri = SimpleTerm::Iri(IriRef::new_unchecked(format!("{}function/{}", ns.ex_iri.as_str(), func.function_name).into()));
    let file_iri = SimpleTerm::Iri(IriRef::new_unchecked(format!("{}file/{}", ns.ex_iri.as_str(), func.file_path.replace(".", "_").replace("/", "_").replace("-", "_")).into()));

    // Function triples
    graph.insert(&func_iri, &ns.rdf.iri().unwrap(), &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}Function", ns.ex_iri.as_str()).into())))?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}label", ns.rdfs_iri.as_str()).into())), func.function_name.as_str())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasCodeSnippet", ns.ex_iri.as_str()).into())), func.code_snippet.as_str())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasSemanticSummary", ns.ex_iri.as_str()).into())), func.semantic_summary.as_str())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasMultivectorEmbedding", ns.ex_iri.as_str()).into())), func.multivector_str.as_str())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasSieveAddress", ns.ex_iri.as_str()).into())), func.sieve_address.as_str())?;
    
    // Add closest emojis
    for (i, emoji_info) in func.closest_emojis.iter().enumerate() {
        let emoji_iri = SimpleTerm::Iri(IriRef::new_unchecked(format!("{}closestEmoji/{}", ns.ex_iri.as_str(), i).into()));
        graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasClosestEmojiInfo", ns.ex_iri.as_str()).into())), &emoji_iri)?;
        graph.insert(&emoji_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}emojiChar", ns.ex_iri.as_str()).into())), emoji_info.emoji.as_str())?;
        graph.insert(&emoji_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}emojiCategory", ns.ex_iri.as_str()).into())), emoji_info.category.as_str())?;
        graph.insert(&emoji_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}emojiDistance", ns.ex_iri.as_str()).into())), &(emoji_info.distance as f64).into_term::<SimpleTerm>())?;
    }

    if let Some(orbital_path) = func.orbital_path {
        let orbital_path_str = orbital_path.iter().map(|(x, y)| format!("{:.4},{:.4}", x, y)).collect::<Vec<String>>().join(";");
        graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasOrbitalPath", ns.ex_iri.as_str()).into())), orbital_path_str.as_str())?;
    }

    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}isInFile", ns.ex_iri.as_str()).into())), &file_iri)?;

    // File triples
    graph.insert(&file_iri, &ns.rdf.iri().unwrap(), &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}CodeFile", ns.ex_iri.as_str()).into())))?;
    graph.insert(&file_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}label", ns.rdfs_iri.as_str()).into())), func.file_path.as_str())?;

    Ok(())
}

pub fn process_analyzed_token<G>(
    graph: &mut G,
    token_data: AnalyzedToken,
    ns: &Namespaces,
) -> anyhow::Result<()>
where
    G: MutableGraph,
    <G as MutableGraph>::MutationError: Send + Sync + 'static,
{
    let token_iri = SimpleTerm::Iri(IriRef::new_unchecked(format!("{}token/{}", ns.ex_iri.as_str(), token_data.token).into()));

    graph.insert(&token_iri, &ns.rdf.iri().unwrap(), &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}Token", ns.ex_iri.as_str()).into())))?;
    graph.insert(&token_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}label", ns.rdfs_iri.as_str()).into())), token_data.token.as_str())?;
    graph.insert(&token_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}count", ns.ex_iri.as_str()).into())), &(token_data.count as f64).into_term::<SimpleTerm>())?;
    graph.insert(&token_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}multivectorEmbedding", ns.ex_iri.as_str()).into())), token_data.multivector_str.as_str())?;

    if let Some(orbital_path) = token_data.orbital_path {
        let orbital_path_str = orbital_path.iter().map(|(x, y)| format!("{:.4},{:.4}", x, y)).collect::<Vec<String>>().join(";");
        graph.insert(&token_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasOrbitalPath", ns.ex_iri.as_str()).into())), orbital_path_str.as_str())?;
    }

    Ok(())
}