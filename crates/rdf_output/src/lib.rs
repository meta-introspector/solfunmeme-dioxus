use anyhow::Result;
use shared_analysis_types::CodeChunk;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;

pub fn code_chunks_to_rdf(chunks: Vec<CodeChunk>) -> Result<RdfGraph<'static>> {
    let mut graph = RdfGraph::new();
    graph.namespaces.add_namespace("sol", "http://solfunmeme.com/")?;
    graph.namespaces.add_namespace("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")?;
    graph.namespaces.add_namespace("xsd", "http://www.w3.org/2001/XMLSchema#")?;

    for chunk in chunks {
        let chunk_iri = graph.namespaces.get_term("sol", &format!("code_chunk/{}", chunk.path.replace(".", "_").replace("/", "_")))?;
        let type_prop = graph.namespaces.get_term("rdf", "type")?;
        let chunk_class = graph.namespaces.get_term("sol", "CodeChunk")?;
        graph.add_triple(&chunk_iri, &type_prop, &chunk_class)?;

        let content_prop = graph.namespaces.get_term("sol", "hasContent")?;
        graph.add_literal_triple(&chunk_iri, &content_prop, &chunk.content, graph.namespaces.get_base_iri("xsd").unwrap())?;

        let emoji_prop = graph.namespaces.get_term("sol", "hasEmoji")?;
        graph.add_literal_triple(&chunk_iri, &emoji_prop, &chunk.emoji, graph.namespaces.get_base_iri("xsd").unwrap())?;

        let line_start_prop = graph.namespaces.get_term("sol", "hasLineStart")?;
        graph.add_literal_triple(&chunk_iri, &line_start_prop, &chunk.line_start.to_string(), graph.namespaces.get_base_iri("xsd").unwrap())?;

        let line_end_prop = graph.namespaces.get_term("sol", "hasLineEnd")?;
        graph.add_literal_triple(&chunk_iri, &line_end_prop, &chunk.line_end.to_string(), graph.namespaces.get_base_iri("xsd").unwrap())?;

        let chunk_type_prop = graph.namespaces.get_term("sol", "hasChunkType")?;
        graph.add_literal_triple(&chunk_iri, &chunk_type_prop, &chunk.chunk_type, graph.namespaces.get_base_iri("xsd").unwrap())?;
    }

    Ok(graph)
}