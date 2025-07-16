use std::collections::HashSet;
use solfunmeme_clifford::{BertCliffordEncoder, SolMultivector, BertConfig as CliffordBertConfig};
use super::embedding_processing::cosine_similarity;

pub fn group_chunks_by_similarity(chunks_with_embeddings: Vec<(String, Vec<f32>)>, similarity_threshold: f32) -> anyhow::Result<Vec<(Vec<String>, SolMultivector)>> {
    let mut processed_indices = HashSet::new();
    let mut groups: Vec<(Vec<String>, SolMultivector)> = Vec::new();

    let bert_clifford_encoder = BertCliffordEncoder::new(CliffordBertConfig::default());

    for i in 0..chunks_with_embeddings.len() {
        if processed_indices.contains(&i) {
            continue;
        }

        let (current_content, current_embedding) = &chunks_with_embeddings[i];
        let mut current_group_contents = vec![current_content.clone()];
        let mut current_group_multivector = bert_clifford_encoder.encode_embedding(current_embedding)?;
        processed_indices.insert(i);

        for j in (i + 1)..chunks_with_embeddings.len() {
            if processed_indices.contains(&j) {
                continue;
            }

            let (other_content, other_embedding) = &chunks_with_embeddings[j];
            let similarity = cosine_similarity(current_embedding, other_embedding);

            if similarity >= similarity_threshold {
                current_group_contents.push(other_content.clone());
                // For simplicity, we'll just use the first chunk's multivector for the group
                // A more sophisticated approach might average or combine multivectors
                processed_indices.insert(j);
            }
        }
        groups.push((current_group_contents, current_group_multivector));
    }
    Ok(groups)
}
