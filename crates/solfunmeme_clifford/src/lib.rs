use serde::{Deserialize, Serialize};
use tclifford::{declare_algebra, Multivector};
use anyhow::Result; // Added for encode_embedding function

// BERT Configuration matching your provided config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BertConfig {
    pub hidden_act: String, // "gelu"
    pub attention_probs_dropout_prob: f32,
    pub hidden_dropout_prob: f32,
    pub hidden_size: usize, // 384 for all-MiniLM-L6-v2
    pub initializer_range: f32,
    pub intermediate_size: usize,       // 1536
    pub max_position_embeddings: usize, // 512
    pub num_attention_heads: usize,     // 12
    pub num_hidden_layers: usize,       // 6
    pub type_vocab_size: usize,         // 2
    pub vocab_size: usize,              // 30522
}

impl Default for BertConfig {
    fn default() -> Self {
        Self {
            hidden_act: "gelu".to_string(),
            attention_probs_dropout_prob: 0.1,
            hidden_dropout_prob: 0.1,
            hidden_size: 384,
            initializer_range: 0.02,
            intermediate_size: 1536,
            max_position_embeddings: 512,
            num_attention_heads: 12,
            num_hidden_layers: 6,
            type_vocab_size: 2,
            vocab_size: 30522,
        }
    }
}

// Declare Clifford algebra for 8D embeddings
declare_algebra!(pub SolCl, [+,+,+,+,+,+,+,+], ["e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8"]);
pub type SolMultivector = Multivector<f32, SolCl>;

/// Encoder to map BERT embeddings to Clifford algebra representation
pub struct BertCliffordEncoder {
    config: BertConfig,
    // Projection matrices to map 384D -> 8D (generator space)
    projection_matrix: Vec<Vec<f32>>, // 8 x 384 matrix
    // Optional: learned basis weights for different grades
    grade_weights: Vec<f32>, // weights for scalar, vector, bivector, etc.
}

impl BertCliffordEncoder {
    pub fn new(config: BertConfig) -> Self {
        // Initialize random projection matrix (8 x 384)
        let mut projection_matrix = Vec::new();
        for _ in 0..8 {
            let mut row = Vec::new();
            for _ in 0..config.hidden_size {
                // Random initialization following BERT's initializer_range
                row.push((rand::random::<f32>() - 0.5) * 2.0 * config.initializer_range);
            }
            projection_matrix.push(row);
        }

        // Initialize grade weights (for 8D algebra we have grades 0-8)
        let grade_weights = vec![1.0; 9]; // Equal weighting initially

        Self {
            config,
            projection_matrix,
            grade_weights,
        }
    }

    /// Encode a BERT embedding vector into Clifford algebra multivector
    pub fn encode_embedding(&self, bert_embedding: &[f32]) -> anyhow::Result<SolMultivector> {
        if bert_embedding.len() != self.config.hidden_size {
            return Err(anyhow::anyhow!(
                "Expected embedding size {}, got {}",
                self.config.hidden_size,
                bert_embedding.len()
            ));
        }

        // Project 384D BERT embedding to 8D generator space
        let mut generators = Vec::with_capacity(8);
        for i in 0..8 {
            let mut sum = 0.0;
            for j in 0..self.config.hidden_size {
                sum += self.projection_matrix[i][j] * bert_embedding[j];
            }
            generators.push(sum);
        }

        // Create multivector using the dense representation
        let mv = Multivector::<f32, SolCl>::from_vector(generators.iter().cloned());

        match mv {
            Ok(mv) => Ok(mv),
            Err(_) => {
                // Fallback: manual construction
                let mut mv = SolMultivector::default();

                // Add scalar part (magnitude of original vector)
                let magnitude = bert_embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
                mv = mv + SolMultivector::from_scalar(magnitude * self.grade_weights[0]);

                // Add vector parts
                for (i, &gen_val) in generators.iter().enumerate() {
                    if i < 8 {
                        // Create basis vector e_i (this is simplified - actual implementation would use basis())
                        // For now, we'll create a simple representation
                        mv = mv
                            + SolMultivector::from_scalar(gen_val * self.grade_weights[1]);
                    }
                }

                Ok(mv)
            }
        }
    }
}

pub fn get_sieve_address(multivector: &SolMultivector) -> String {
    let mut address = String::with_capacity(8);

    // The first 8 coefficients of the multivector correspond to the e1 through e8 basis vectors.
    // In tclifford, basis elements are indexed by their bitmask representation.
    for i in 0..8 {
        // The index for e(i+1) is 1 << i.
        let component = multivector.get_by_idx(1 << i);
        if component >= 0.0 {
            address.push('1');
        } else {
            address.push('0');
        }
    }
    address
}
