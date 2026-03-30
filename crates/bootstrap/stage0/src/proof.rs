//! Defines the recursive `Proof` structure for the lattice of knowledge.

use crate::describable::Describable;
use crate::hash::Hash;

/// A recursive proof of a component's uniqueness and composition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Proof {
    /// An atomic proof, representing a fundamental, irreducible artifact.
    Atomic(Hash),
    /// A composite proof, representing an artifact that is composed of other proofs.
    Composite {
        hash: Hash,
        components: Vec<Hash>,
    },
}

impl Describable for Proof {
    /// The canonical description of a proof is derived from its constituent hashes.
    fn describe(&self) -> Vec<u8> {
        match self {
            Proof::Atomic(hash) => hash.hash_bytes.clone(),
            Proof::Composite { components, .. } => {
                // A simple, deterministic concatenation of component hash bytes.
                components
                    .iter()
                    .flat_map(|h| h.hash_bytes.clone())
                    .collect()
            }
        }
    }
}

impl Proof {
    /// Returns the definitive hash for this proof.
    pub fn hash(&self) -> &Hash {
        match self {
            Proof::Atomic(hash) => hash,
            Proof::Composite { hash, .. } => hash,
        }
    }
}
