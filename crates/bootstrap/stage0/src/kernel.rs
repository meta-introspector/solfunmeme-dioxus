//! Defines the `Kernel`, the central orchestrating component of the system.

use crate::chord::ChordStore;
use crate::hash::{Hash, Hasher};
use crate::manifold::{Coordinate, ManifoldProjector};
use crate::proof::Proof;
use crate::univalence::{EquivalenceProof, RewriteOp};
use crate::verifier::ProofVerifier;

/// The central orchestrator of the microkernel.
pub struct Kernel {
    /// The current step in the 42-step Ouroboros cycle.
    pub step: u64,
    store: ChordStore,
    verifier: Box<dyn ProofVerifier>,
    projector: Box<dyn ManifoldProjector>,
    hasher: Box<dyn Hasher>,
    /// A record of all rewrite operations applied to this Kernel instance.
    pub history: EquivalenceProof,
}

impl Kernel {
    /// Creates a new Kernel with the given components.
    pub fn new(
        verifier: Box<dyn ProofVerifier>,
        projector: Box<dyn ManifoldProjector>,
        hasher: Box<dyn Hasher>,
    ) -> Self {
        Self {
            step: 0,
            store: ChordStore::new(),
            verifier,
            projector,
            hasher,
            history: EquivalenceProof::default(),
        }
    }

    /// Ingests a proof into the kernel's verifier, then places it on the Chord.
    pub fn ingest(&mut self, proof: Proof) -> Result<(), &'static str> {
        self.verifier.ingest_proof(proof.clone())?;
        self.store.place(proof.hash());
        Ok(())
    }

    /// Retrieves the geometric coordinate of an entity on the manifold.
    pub fn get_coordinate(&self, hash: &Hash) -> Option<Coordinate> {
        if self.store.exists(hash) {
            Some(self.projector.project(hash))
        } else {
            None
        }
    }

    /// Replaces the kernel's hasher with a new one, advancing the cycle.
    pub fn rewrite_hasher(&mut self, new_hasher: Box<dyn Hasher>) {
        self.hasher = new_hasher;
        self.history.path.push(RewriteOp::UpdateHasher);
        self.advance_cycle();
    }

    /// Replaces the kernel's projector with a new one, advancing the cycle.
    pub fn rewrite_projector(&mut self, new_projector: Box<dyn ManifoldProjector>) {
        self.projector = new_projector;
        self.history.path.push(RewriteOp::UpdateProjector);
        self.advance_cycle();
    }

    /// Advances the Ouroboros cycle counter, wrapping at 42.
    fn advance_cycle(&mut self) {
        self.step = (self.step + 1) % 42;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::default_hasher::ChecksumHasher;
    use crate::manifold::DefaultProjector;

    fn create_test_kernel() -> Kernel {
        Kernel::new(Box::new(DefaultProjector), Box::new(ChecksumHasher))
    }

    #[test]
    fn test_kernel_creation() {
        let kernel = create_test_kernel();
        assert_eq!(kernel.step, 0);
        assert!(kernel.history.path.is_empty());
    }

    #[test]
    fn test_store_and_get_coordinate() {
        let mut kernel = create_test_kernel();
        let content = b"hello ship of theseus".to_vec();

        let hash = kernel.store_artifact(content).unwrap();
        let coord = kernel.get_artifact_coordinate(&hash);

        assert!(coord.is_some());
    }

    #[test]
    fn test_hardcoded_example_flow() {
        let mut kernel = create_test_kernel();

        // 1. Store a hard-coded example artifact
        let example1_content = b"I am the first plank.".to_vec();
        let hash1 = kernel.store_artifact(example1_content).unwrap();
        assert!(kernel.get_artifact_coordinate(&hash1).is_some());

        // 2. Store another
        let example2_content = b"I am the second plank.".to_vec();
        let hash2 = kernel.store_artifact(example2_content).unwrap();
        assert!(kernel.get_artifact_coordinate(&hash2).is_some());

        // 3. Ensure they are distinct
        assert_ne!(hash1, hash2);
        assert_ne!(
            kernel.get_artifact_coordinate(&hash1),
            kernel.get_artifact_coordinate(&hash2)
        );
    }

    #[test]
    fn test_rewrite_cycle_and_history() {
        let mut kernel = create_test_kernel();
        assert_eq!(kernel.step, 0);
        assert_eq!(kernel.history.path.len(), 0);

        // Perform a rewrite
        kernel.rewrite_hasher(Box::new(ChecksumHasher));
        assert_eq!(kernel.step, 1);
        assert_eq!(kernel.history.path.len(), 1);
        assert_eq!(kernel.history.path[0], RewriteOp::UpdateHasher);

        // Perform another
        kernel.rewrite_projector(Box::new(DefaultProjector));
        assert_eq!(kernel.step, 2);
        assert_eq!(kernel.history.path.len(), 2);
        assert_eq!(kernel.history.path[1], RewriteOp::UpdateProjector);
    }

    #[test]
    fn test_ouroboros_cycle_wraps() {
        let mut kernel = create_test_kernel();
        for i in 0..41 {
            assert_eq!(kernel.step, i);
            kernel.rewrite_hasher(Box::new(ChecksumHasher));
        }
        assert_eq!(kernel.step, 41);

        // The 42nd step should wrap the cycle back to 0
        kernel.rewrite_hasher(Box::new(ChecksumHasher));
        assert_eq!(kernel.step, 0);
    }
}
