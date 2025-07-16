use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeVector {
    pub dimensions: Vec<f32>,
    pub metadata: HashMap<String, String>,
}

impl CodeVector {
    pub fn new(dimensions: Vec<f32>) -> Self {
        Self {
            dimensions,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn similarity(&self, other: &CodeVector) -> f32 {
        if self.dimensions.len() != other.dimensions.len() {
            return 0.0;
        }

        let dot_product: f32 = self
            .dimensions
            .iter()
            .zip(other.dimensions.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: f32 = self.dimensions.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = other.dimensions.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
}

pub struct CodeVectorizer {
    dimension_size: usize,
}

impl CodeVectorizer {
    pub fn new(dimension_size: usize) -> Self {
        Self { dimension_size }
    }

    pub fn vectorize(&self, code: &str) -> CodeVector {
        let mut dimensions = vec![0.0; self.dimension_size];

        // Simple hash-based vectorization
        for (i, byte) in code.bytes().enumerate() {
            let idx = (byte as usize + i) % self.dimension_size;
            dimensions[idx] += 1.0;
        }

        // Normalize
        let sum: f32 = dimensions.iter().sum();
        if sum > 0.0 {
            for dim in &mut dimensions {
                *dim /= sum;
            }
        }

        CodeVector::new(dimensions).with_metadata("length".to_string(), code.len().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_vector_creation() {
        let vector = CodeVector::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(vector.dimensions, vec![1.0, 2.0, 3.0]);
        assert!(vector.metadata.is_empty());
    }

    #[test]
    fn test_code_vector_with_metadata() {
        let vector = CodeVector::new(vec![1.0, 2.0])
            .with_metadata("type".to_string(), "function".to_string());
        assert_eq!(vector.metadata.get("type"), Some(&"function".to_string()));
    }

    #[test]
    fn test_similarity_identical_vectors() {
        let v1 = CodeVector::new(vec![1.0, 0.0, 0.0]);
        let v2 = CodeVector::new(vec![1.0, 0.0, 0.0]);
        assert!((v1.similarity(&v2) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_similarity_orthogonal_vectors() {
        let v1 = CodeVector::new(vec![1.0, 0.0]);
        let v2 = CodeVector::new(vec![0.0, 1.0]);
        assert!((v1.similarity(&v2) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_vectorizer() {
        let vectorizer = CodeVectorizer::new(10);
        let vector = vectorizer.vectorize("fn main() {}");
        assert_eq!(vector.dimensions.len(), 10);
        assert_eq!(vector.metadata.get("length"), Some(&"12".to_string()));
    }

    #[test]
    fn test_vectorizer_normalization() {
        let vectorizer = CodeVectorizer::new(5);
        let vector = vectorizer.vectorize("test");
        let sum: f32 = vector.dimensions.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
