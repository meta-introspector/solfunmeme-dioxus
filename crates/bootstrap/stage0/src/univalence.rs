//! Defines the structures for representing Univalence: Identity as Equivalence.

/// Represents a single, discrete rewrite operation on the Kernel.
///
/// A sequence of these operations forms a path that proves the equivalence
/// between two states of the Kernel. Each variant corresponds to a specific
/// component of the `Kernel` that can be replaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewriteOp {
    /// Represents the replacement of the Kernel's `Hasher`.
    UpdateHasher,
    /// Represents the replacement of the Kernel's `ManifoldProjector`.
    UpdateProjector,
}

/// A proof that two states are equivalent, represented by a path of rewrites.
///
/// In the context of the Univalent Axiom, this path *is* the identity
/// between the initial and final states. It makes the "Ship of Theseus"
/// concept a first-class, observable entity in the system.
#[derive(Debug, Clone, Default)]
pub struct EquivalenceProof {
    /// The sequence of rewrite operations that have occurred.
    pub path: Vec<RewriteOp>,
}

impl EquivalenceProof {
    /// Creates a new, empty proof.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equivalence_proof_creation() {
        let proof = EquivalenceProof::new();
        assert!(proof.path.is_empty());
    }

    #[test]
    fn test_equivalence_proof_add_op() {
        let mut proof = EquivalenceProof::new();
        proof.path.push(RewriteOp::UpdateHasher);
        assert_eq!(proof.path.len(), 1);
        assert_eq!(proof.path[0], RewriteOp::UpdateHasher);
    }
}
