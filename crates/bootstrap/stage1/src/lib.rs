//! Bootstrap Stage 1: The First Reflection (Duality)
//!
//! This crate introduces the first concrete plugin, the `Dualizer`.

use bootstrap_stage0::{Describable, Hash, Hasher, Proof, ZosComponent};

/// The `Dualizer` component, representing the concept of Duality (2).
///
/// It reflects an artifact by reversing its content.
pub struct Dualizer;

impl Describable for Dualizer {
    /// The canonical description of the Dualizer is its name.
    fn describe(&self) -> &[u8] {
        self.name().as_bytes()
    }
}

impl ZosComponent for Dualizer {
    fn name(&self) -> &'static str {
        "dualizer"
    }

    fn prove(&self) -> Proof {
        // For now, its proof is an atomic hash of its own name.
        // In a more advanced system, this would be a hash of its source code artifact.
        let hash = bootstrap_stage0::ChecksumHasher.hash(self.describe());
        Proof::Atomic(hash)
    }

    fn execute(&self) {
        // The Dualizer's core logic is not executed directly in this model,
        // but is represented by the `dualize` function.
    }
}

/// Performs the dualization operation.
///
/// Takes a "base" artifact and a `Hasher`, and returns a new "dual" artifact
/// with the content reversed.
pub fn dualize(base_artifact_content: &[u8], hasher: &dyn Hasher) -> (Vec<u8>, Hash) {
    let mut dual_content = base_artifact_content.to_vec();
    dual_content.reverse();
    let dual_hash = hasher.hash(&dual_content);
    (dual_content, dual_hash)
}
