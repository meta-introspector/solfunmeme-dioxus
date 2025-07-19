use crate::describable::Describable;
use crate::hash::Hash;
use std::borrow::Cow;

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
    fn describe(&self) -> Cow<'static, [u8]> {
        match self {
            Proof::Atomic(hash) => Cow::Owned(hash.hash_bytes.clone()),
            Proof::Composite { components, .. } => {
                Cow::Owned(components
                    .iter()
                    .flat_map(|h| h.hash_bytes.clone())
                    .collect::<Vec<u8>>())}
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
