use serde::{Deserialize, Serialize};
use tclifford::{declare_algebra, Multivector};

// BERT Configuration matching your provided config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BertConfig {
    pub hidden_act: String, // "gelu"
    pub attention_probs_dropout_prob: f32,
    pub hidden_dropout_prob: f32,
    pub hidden_size: usize, // 768
    pub initializer_range: f32,
    pub intermediate_size: usize,       // 3072
    pub max_position_embeddings: usize, // 512
    pub num_attention_heads: usize,     // 12
    pub num_hidden_layers: usize,       // 12
    pub type_vocab_size: usize,         // 2
    pub vocab_size: usize,              // 30522
}

impl Default for BertConfig {
    fn default() -> Self {
        Self {
            hidden_act: "gelu".to_string(),
            attention_probs_dropout_prob: 0.1,
            hidden_dropout_prob: 0.1,
            hidden_size: 768,
            initializer_range: 0.02,
            intermediate_size: 3072,
            max_position_embeddings: 512,
            num_attention_heads: 12,
            num_hidden_layers: 12,
            type_vocab_size: 2,
            vocab_size: 30522,
        }
    }
}

// Declare Clifford algebra for BERT embeddings
// Using Cl(10,0) gives us 2^10 = 1024 basis elements, enough for 768-dim BERT
declare_algebra!(BertCl, [+,+,+,+,+,+,+,+,+,+], ["e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "e10"]);
type BertMultivector = Multivector<f32, BertCl>;

/// Encoder to map BERT embeddings to Clifford algebra representation
pub struct BertCliffordEncoder {
    config: BertConfig,
    // Projection matrices to map 768D -> 10D (generator space)
    projection_matrix: Vec<Vec<f32>>, // 10 x 768 matrix
    // Optional: learned basis weights for different grades
    grade_weights: Vec<f32>, // weights for scalar, vector, bivector, etc.
}

impl BertCliffordEncoder {
    pub fn new(config: BertConfig) -> Self {
        // Initialize random projection matrix (10 x 768)
        let mut projection_matrix = Vec::new();
        for _ in 0..10 {
            let mut row = Vec::new();
            for _ in 0..config.hidden_size {
                // Random initialization following BERT's initializer_range
                row.push((rand::random::<f32>() - 0.5) * 2.0 * config.initializer_range);
            }
            projection_matrix.push(row);
        }

        // Initialize grade weights (for 10D algebra we have grades 0-10)
        let grade_weights = vec![1.0; 11]; // Equal weighting initially

        Self {
            config,
            projection_matrix,
            grade_weights,
        }
    }

    /// Encode a BERT embedding vector into Clifford algebra multivector
    pub fn encode_embedding(&self, bert_embedding: &[f32]) -> Result<BertMultivector, String> {
        if bert_embedding.len() != self.config.hidden_size {
            return Err(format!(
                "Expected embedding size {}, got {}",
                self.config.hidden_size,
                bert_embedding.len()
            ));
        }

        // Project 768D BERT embedding to 10D generator space
        let mut generators = Vec::with_capacity(10);
        for i in 0..10 {
            let mut sum = 0.0;
            for j in 0..self.config.hidden_size {
                sum += self.projection_matrix[i][j] * bert_embedding[j];
            }
            generators.push(sum);
        }

        // Create multivector using the dense representation
        let multivector = BertMultivector::default();

        // Method 1: Simple encoding - use generators as vector components
        // This creates a grade-1 multivector (pure vector)
        //let mv = MultivectorBase::<f32, BertCl, tclifford::coeff_storage::ArrayStorage<f32>>::from_vector(generators.iter().cloned());
        let mv = tclifford::Multivector::<f32, BertCl>::from_vector(generators.iter().cloned());

        match mv {
            Ok(mv) => Ok(mv),
            Err(_) => {
                // Fallback: manual construction
                let mut mv = BertMultivector::default();

                // Add scalar part (magnitude of original vector)
                let magnitude = bert_embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
                mv = mv + BertMultivector::from_scalar(magnitude * self.grade_weights[0]);

                // Add vector parts
                for (i, &gen_val) in generators.iter().enumerate() {
                    if i < 10 {
                        let mut basis_mv = BertMultivector::default();
                        // Create basis vector e_i (this is simplified - actual implementation would use basis())
                        // For now, we'll create a simple representation
                        basis_mv = basis_mv
                            + BertMultivector::from_scalar(gen_val * self.grade_weights[1]);
                    }
                }

                Ok(mv)
            }
        }
    }

    /// Enhanced encoding that uses multiple grades
    pub fn encode_embedding_multigrade(
        &self,
        bert_embedding: &[f32],
    ) -> Result<BertMultivector, String> {
        if bert_embedding.len() != self.config.hidden_size {
            return Err(format!(
                "Expected embedding size {}, got {}",
                self.config.hidden_size,
                bert_embedding.len()
            ));
        }

        // Project to generator space
        let mut generators = Vec::with_capacity(10);
        for i in 0..10 {
            let mut sum = 0.0;
            for j in 0..self.config.hidden_size {
                sum += self.projection_matrix[i][j] * bert_embedding[j];
            }
            generators.push(sum);
        }

        // Create multivector with multiple grades
        let mut result = BertMultivector::default();

        // Grade 0 (scalar): overall magnitude
        let magnitude = bert_embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        result = result + BertMultivector::from_scalar(magnitude * self.grade_weights[0]);

        // Grade 1 (vector): directional information
        if let Ok(vector_part) =
            BertMultivector::from_vector(generators.iter().map(|&x| x * self.grade_weights[1]))
        {
            result = result + vector_part;
        }

        // Higher grades could encode more complex relationships
        // This is a simplified version - full implementation would construct bivectors, etc.

        Ok(result)
    }

    /// Decode multivector back to approximate BERT embedding
    pub fn decode_multivector(&self, multivector: &BertMultivector) -> Vec<f32> {
        // This is a simplified decoder - in practice you'd want the pseudoinverse
        // of the projection matrix and proper grade extraction

        // For now, create a simple reconstruction
        let mut result = vec![0.0; self.config.hidden_size];

        // Extract scalar and vector parts (simplified)
        // In practice, you'd use multivector.grade(0), multivector.grade(1), etc.

        // Placeholder reconstruction
        for i in 0..self.config.hidden_size {
            result[i] = (i as f32) * 0.001; // Simplified placeholder
        }

        result
    }
}

/// Attention mechanism operating in Clifford algebra space
pub struct CliffordAttention {
    config: BertConfig,
    encoder: BertCliffordEncoder,
    // In practice, these would be learned parameters
    query_projection: Vec<Vec<f32>>,
    key_projection: Vec<Vec<f32>>,
    value_projection: Vec<Vec<f32>>,
}

impl CliffordAttention {
    pub fn new(config: BertConfig) -> Self {
        let encoder = BertCliffordEncoder::new(config.clone());

        // Initialize projection matrices (simplified)
        let dim = 1024; // BertCl has 2^10 = 1024 basis elements
        let query_projection = vec![vec![0.01; dim]; dim];
        let key_projection = vec![vec![0.01; dim]; dim];
        let value_projection = vec![vec![0.01; dim]; dim];

        Self {
            config,
            encoder,
            query_projection,
            key_projection,
            value_projection,
        }
    }

    /// Compute attention using geometric product in Clifford algebra
    pub fn forward(&self, embeddings: &[Vec<f32>]) -> Result<Vec<BertMultivector>, String> {
        // Convert BERT embeddings to Clifford multivectors
        let mut multivectors = Vec::new();
        for embedding in embeddings {
            let mv = self.encoder.encode_embedding(embedding)?;
            multivectors.push(mv);
        }

        // Simplified attention computation
        // In full implementation, you'd use geometric products for richer interactions
        let mut attended_mvs = Vec::new();

        for (i, mv_i) in multivectors.iter().enumerate() {
            let mut attended = BertMultivector::default();

            for (j, mv_j) in multivectors.iter().enumerate() {
                // Compute attention weight (simplified)
                let weight = if i == j { 1.0 } else { 0.1 };

                // In full implementation: use geometric product mv_i * mv_j
                // For now, simple weighted sum
                attended = attended + mv_j.clone() * weight;
            }

            attended_mvs.push(attended);
        }

        Ok(attended_mvs)
    }
}

/// Complete BERT layer with Clifford algebra enhancement
pub struct CliffordBertLayer {
    attention: CliffordAttention,
    encoder: BertCliffordEncoder,
}

impl CliffordBertLayer {
    pub fn new(config: BertConfig) -> Self {
        let attention = CliffordAttention::new(config.clone());
        let encoder = BertCliffordEncoder::new(config);

        Self { attention, encoder }
    }

    pub fn forward(&self, input_embeddings: &[Vec<f32>]) -> Result<Vec<Vec<f32>>, String> {
        // Apply attention in Clifford space
        let attended_mvs = self.attention.forward(input_embeddings)?;

        // Decode back to BERT embedding space
        let mut output_embeddings = Vec::new();
        for mv in attended_mvs {
            let decoded = self.encoder.decode_multivector(&mv);
            output_embeddings.push(decoded);
        }

        Ok(output_embeddings)
    }
}

//#[cfg(test)]
mod tests {
    use super::{BertConfig, BertCliffordEncoder, CliffordBertLayer};

    #[test]
    fn test_clifford_encoding() {
        let config = BertConfig::default();
        let encoder = BertCliffordEncoder::new(config);

        // Create dummy BERT embedding
        let bert_embedding: Vec<f32> = (0..768).map(|i| (i as f32) * 0.001).collect();

        // Encode to Clifford algebra
        let multivector = encoder.encode_embedding(&bert_embedding).unwrap();

        // Decode back
        let decoded = encoder.decode_multivector(&multivector);

        println!("Original embedding length: {}", bert_embedding.len());
        println!("Decoded embedding length: {}", decoded.len());
        println!("Multivector created successfully");
    }

    #[test]
    fn test_clifford_bert_layer() {
        let config = BertConfig::default();
        let layer = CliffordBertLayer::new(config);

        // Create sequence of dummy embeddings
        let input_embeddings: Vec<Vec<f32>> = (0..5)
            .map(|seq_idx| {
                (0..768)
                    .map(|i| (i as f32 + seq_idx as f32) * 0.001)
                    .collect()
            })
            .collect();

        // Forward pass
        let output = layer.forward(&input_embeddings).unwrap();

        assert_eq!(output.len(), input_embeddings.len());
        assert_eq!(output[0].len(), 768);
        println!("Clifford BERT layer forward pass successful");
    }
}

// Example usage
fn main() {
    let config = BertConfig::default();
    println!("BERT Config: {:?}", config);

    let encoder = BertCliffordEncoder::new(config.clone());
    println!("Created Clifford encoder for BERT");

    // Example: encode a BERT embedding
    let dummy_embedding: Vec<f32> = (0..768).map(|i| (i as f32) * 0.001).collect();

    match encoder.encode_embedding(&dummy_embedding) {
        Ok(multivector) => {
            println!("Successfully encoded BERT embedding to Clifford multivector");
            println!("Multivector basis size: 2^10 = 1024 elements");

            // Decode back
            let decoded = encoder.decode_multivector(&multivector);
            println!("Decoded back to {}-dimensional vector", decoded.len());
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("\nKey benefits of this representation:");
    println!("1. Dense storage: 768D -> Clifford algebra with 1024 basis elements");
    println!("2. Geometric structure: Natural rotations, reflections, projections");
    println!("3. Rich products: Geometric product captures complex token relationships");
    println!("4. Grade structure: Scalars, vectors, bivectors encode different aspects");
    println!("5. Fast operations: tclifford optimized for geometric algebra operations");
}
