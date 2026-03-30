use linfa::prelude::*;
use linfa_reduction::Pca;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThemeNode {
    pub emoji: String,
    pub color: String,
    pub dim_values: [f64; 8],
}

/// Reduce 768D BERT embeddings to 8D ThemeNodes using PCA.
pub fn reduce_bert_to_8d(embeddings: Array2<f64>) -> Vec<ThemeNode> {
    let n_samples = embeddings.nrows();

    // Determine the number of PCA components based on available samples
    // We need at least as many samples as components for PCA to work
    let n_components = std::cmp::min(8, n_samples);

    // Create a dataset with dummy targets (zeros)
    let targets = Array1::<f64>::zeros(n_samples);
    let dataset = DatasetBase::new(embeddings.clone(), targets);

    // Fit PCA with appropriate number of components
    let pca = Pca::params(n_components).fit(&dataset).unwrap();

    // Transform the data
    let reduced = pca.transform(dataset);

    // Get the reduced data as Array2
    let reduced_data = reduced.records();

    let emojis = vec!["🚀", "📜", "🔍", "💬", "🔀", "💡", "💭", "🔑"];
    let colors = vec![
        "rgba(255, 0, 0, 0.8)",
        "rgba(255, 255, 0, 0.8)",
        "rgba(0, 255, 255, 0.8)",
        "rgba(255, 0, 255, 0.8)",
        "rgba(0, 255, 0, 0.8)",
        "rgba(255, 128, 0, 0.8)",
        "rgba(128, 255, 0, 0.8)",
        "rgba(255, 0, 128, 0.8)",
    ];

    reduced_data
        .outer_iter()
        .enumerate()
        .map(|(i, row)| {
            let emoji_idx = i % emojis.len();
            let color_idx = i % colors.len();

            // Create 8D array, padding with zeros if PCA produced fewer dimensions
            let mut dim_values = [0.0; 8];
            for (j, val) in row.iter().enumerate() {
                if j < 8 {
                    dim_values[j] = *val;
                }
            }

            ThemeNode {
                emoji: emojis[emoji_idx].to_string(),
                color: colors[color_idx].to_string(),
                dim_values,
            }
        })
        .collect()
}

fn convert_embeddings(embeddings: Array1<[f64; 768]>) -> Array2<f64> {
    let n = embeddings.len();
    Array2::from_shape_vec((n, 768), embeddings.into_iter().flatten().collect())
        .expect("Failed to reshape embeddings")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_reduce_bert_to_8d_output_length() {
        let embeddings = array![
            [0.1; 768], [0.2; 768], [0.3; 768], [0.4; 768], [0.5; 768], [0.6; 768], [0.7; 768],
            [0.8; 768],
        ];
        let nodes = reduce_bert_to_8d(convert_embeddings(embeddings));
        assert_eq!(nodes.len(), 8);
        for node in nodes {
            assert_eq!(node.dim_values.len(), 8);
        }
    }

    #[test]
    fn test_theme_node_fields() {
        // Need at least 8 samples for 8D PCA, so let's use more samples
        let embeddings = array![
            [0.1; 768], [0.2; 768], [0.3; 768], [0.4; 768], [0.5; 768], [0.6; 768], [0.7; 768],
            [0.8; 768], [0.9; 768], [1.0; 768],
        ];
        let nodes = reduce_bert_to_8d(convert_embeddings(embeddings));
        assert_eq!(nodes[0].emoji, "🚀");
        assert_eq!(nodes[1].emoji, "📜");
        assert_eq!(nodes[0].color, "rgba(255, 0, 0, 0.8)");
        assert_eq!(nodes[1].color, "rgba(255, 255, 0, 0.8)");

        // Verify we have the expected number of nodes
        assert_eq!(nodes.len(), 10);

        // Verify all nodes have 8 dimensions
        for node in &nodes {
            assert_eq!(node.dim_values.len(), 8);
        }
    }

    #[test]
    fn test_with_fewer_samples_than_dimensions() {
        // Test with only 2 samples - should work but produce fewer than 8 components
        let embeddings = array![[1.0; 768], [2.0; 768],];
        let nodes = reduce_bert_to_8d(convert_embeddings(embeddings));
        assert_eq!(nodes.len(), 2);

        // Should still have 8D output (padded with zeros)
        for node in &nodes {
            assert_eq!(node.dim_values.len(), 8);
        }

        // Check that emojis and colors are assigned correctly
        assert_eq!(nodes[0].emoji, "🚀");
        assert_eq!(nodes[1].emoji, "📜");
    }

    #[test]
    fn test_dimensionality_reduction() {
        // Test with different input data to ensure PCA actually reduces dimensions
        let embeddings =
            Array2::from_shape_vec((4, 768), (0..4 * 768).map(|i| i as f64 / 1000.0).collect())
                .unwrap();

        let nodes = reduce_bert_to_8d(embeddings);
        assert_eq!(nodes.len(), 4);

        // Verify each node has 8 dimensions
        for node in &nodes {
            assert_eq!(node.dim_values.len(), 8);
            // Check that values are not all zeros (PCA should produce meaningful output)
            assert!(node.dim_values.iter().any(|&x| x != 0.0));
        }
    }
}
