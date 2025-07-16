//! Defines the 8-dimensional manifold and the projection mechanisms.

use crate::hash::Hash;

/// Represents a point in the 8-dimensional Riemannian manifold.
///
/// The coordinates are normalized to represent a point on the surface of a unit
/// 8-sphere, making all points equidistant from the origin in an abstract sense.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate(pub [f64; 8]);

impl Coordinate {
    /// Creates a new coordinate from an 8D vector.
    pub fn new(vector: [f64; 8]) -> Self {
        Self(vector)
    }

    /// Normalizes the coordinate to lie on the surface of a unit 8-sphere.
    ///
    /// If the vector is the zero vector, it returns the zero vector. This ensures
    /// that the magnitude of the returned coordinate is 1, unless it's the origin.
    pub fn normalize(&self) -> Self {
        let mag_sq: f64 = self.0.iter().map(|&val| val * val).sum();

        if mag_sq == 0.0 {
            return Self([0.0; 8]);
        }
        let mag = mag_sq.sqrt();

        let mut normalized_vector = [0.0; 8];
        for (i, &val) in self.0.iter().enumerate() {
            normalized_vector[i] = val / mag;
        }
        Self(normalized_vector)
    }
}

/// A trait for projecting a hash onto a coordinate in the manifold.
///
/// This allows for different strategies of mapping a 1D hash value to a
/// multi-dimensional geometric space.
pub trait ManifoldProjector {
    /// Projects a hash to a coordinate in the 8D manifold.
    fn project(&self, hash: &Hash) -> Coordinate;
}

/// A default projector for Stage 0.
///
/// This provides a simple, deterministic projection for bootstrapping the system.
pub struct DefaultProjector;

impl ManifoldProjector for DefaultProjector {
    /// A simple, deterministic projection.
    ///
    /// It splits the hash bytes into 8 chunks and interprets each as a `u64`,
    /// then normalizes it to a float. This is a basic, non-uniform projection
    /// suitable for stage0.
    fn project(&self, hash: &Hash) -> Coordinate {
        let mut vector = [0.0; 8];
        let hash_bytes = &hash.hash_bytes;

        if hash_bytes.is_empty() {
            return Coordinate::new(vector).normalize();
        }

        // Create 8 values from the hash bytes to serve as the vector components.
        for i in 0..8 {
            let chunk_size = (hash_bytes.len() / 8).max(1);
            let start = (i * chunk_size) % hash_bytes.len();
            let end = (start + chunk_size).min(hash_bytes.len());
            let chunk = &hash_bytes[start..end];

            let mut val = 0u64;
            for (j, &byte) in chunk.iter().enumerate() {
                val = val.wrapping_add((byte as u64).wrapping_mul((j + 1) as u64));
            }
            vector[i] = val as f64;
        }

        Coordinate::new(vector).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_normalization() {
        let coord = Coordinate::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        let normalized = coord.normalize();
        let mag_sq: f64 = normalized.0.iter().map(|&x| x * x).sum();
        assert!((mag_sq - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_zero_vector_normalization() {
        let coord = Coordinate::new([0.0; 8]);
        let normalized = coord.normalize();
        assert_eq!(normalized.0, [0.0; 8]);
    }

    #[test]
    fn test_default_projector() {
        let projector = DefaultProjector;
        let hash = Hash {
            algorithm_id: 0,
            hash_bytes: (0..32).collect(),
        };
        let coord = projector.project(&hash);
        // The magnitude should be 1.0 for a non-zero hash
        let mag_sq: f64 = coord.0.iter().map(|&x| x * x).sum();
        assert!((mag_sq - 1.0).abs() < 1e-9);
    }
}
