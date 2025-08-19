use sophia_api::graph::MutableGraph;
use sophia_api::term::{SimpleTerm, TTerm};
use sophia_inmem::graph::FastGraph;
use sophia_iri::Iri;
use anyhow::Result;
use shared_analysis_types::CodeChunk;

pub fn code_chunks_to_rdf(chunks: Vec<CodeChunk>) -> Result<FastGraph> {
    let mut graph = FastGraph::new();

    for chunk in chunks {
        let chunk_iri = Iri::new_unchecked(format!("http://solfunmeme.com/code_chunk/{}", chunk.path.replace(".", "_").replace("/", "_")));
        let content_literal = SimpleTerm::new_literal_dt(chunk.content, Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#string"));
        let emoji_literal = SimpleTerm::new_literal_dt(chunk.emoji, Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#string"));
        let line_start_literal = SimpleTerm::new_literal_dt(chunk.line_start.to_string(), Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"));
        let line_end_literal = SimpleTerm::new_literal_dt(chunk.line_end.to_string(), Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"));
        let chunk_type_literal = SimpleTerm::new_literal_dt(chunk.chunk_type, Iri::new_unchecked("http://www.w3.org/2001/XMLSchema#string"));

        graph.insert(&chunk_iri, &Iri::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"), &Iri::new_unchecked("http://solfunmeme.com/CodeChunk"))?;
        graph.insert(&chunk_iri, &Iri::new_unchecked("http://solfunmeme.com/hasContent"), &content_literal)?;
        graph.insert(&chunk_iri, &Iri::new_unchecked("http://solfunmeme.com/hasEmoji"), &emoji_literal)?;
        graph.insert(&chunk_iri, &Iri::new_unchecked("http://solfunmeme.com/hasLineStart"), &line_start_literal)?;
        graph.insert(&chunk_iri, &Iri::new_unchecked("http://solfunmeme.com/hasLineEnd"), &line_end_literal)?;
        graph.insert(&chunk_iri, &Iri::new_unchecked("http://solfunmeme.com/hasChunkType"), &chunk_type_literal)?;
    }

    Ok(graph)
}
