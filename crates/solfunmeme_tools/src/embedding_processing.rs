use anyhow::Result;
use solfunmeme_search_tantivy::SearchIndex;
use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_clifford::{BertCliffordEncoder, SolMultivector, BertConfig as CliffordBertConfig};

pub fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    if v1.is_empty() || v2.is_empty() || v1.len() != v2.len() {
        return 0.0;
    }

    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(&a, &b)| a * b).sum();
    let magnitude1: f32 = v1.iter().map(|&a| a * a).sum::<f32>().sqrt();
    let magnitude2: f32 = v2.iter().map(|&b| b * b).sum::<f32>().sqrt();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        0.0
    } else {
        dot_product / (magnitude1 * magnitude2)
    }
}

pub fn retrieve_and_process_embeddings(search_index: &SearchIndex, limit: usize) -> Result<Vec<(String, Vec<f32>)>> {
    let reader = search_index.index.reader()?;
    let searcher = reader.searcher();
    let all_docs = searcher.search(&tantivy::query::AllQuery, &tantivy::collector::TopDocs::with_limit(limit))?;

    eprintln!("Retrieved {} chunks from the index.", all_docs.len());

    let schema = search_index.schema;
    let content_field = schema.get_field("content")?;
    let embedding_field = schema.get_field("embedding")?;

    let mut chunks_with_embeddings: Vec<(String, Vec<f32>)> = Vec::new();

    for (_score, doc_address) in all_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        let content = retrieved_doc
            .get_first(content_field)
            .and_then(|v| v.as_value().as_str())
            .unwrap_or("")
            .to_string();

        let embedding_bytes = retrieved_doc
            .get_first(embedding_field)
            .and_then(|v| v.as_bytes());

        if let Some(bytes) = embedding_bytes {
            let embedding: Vec<f32> = bytes
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect();
            chunks_with_embeddings.push((content, embedding));
        } else {
            eprintln!("[WARN] Chunk has no embedding and will be skipped for grouping: {}", &content[..std::cmp::min(content.len(), 50)]);
        }
    }
    Ok(chunks_with_embeddings)
}
