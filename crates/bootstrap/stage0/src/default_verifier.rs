//! Defines the `DefaultProofVerifier`, the default, lattice-based proof system.

use crate::hash::Hash;
use crate::proof::Proof;
use crate::verifier::ProofVerifier;
use std::collections::HashMap;

/// The default implementation of `ProofVerifier`.
///
/// This verifier maintains an in-memory lattice of proofs and ensures that
/// all composite proofs are built from existing, known proofs.
#[derive(Default)]
pub struct DefaultProofVerifier {
    proofs: HashMap<Hash, Proof>,
}

impl DefaultProofVerifier {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ProofVerifier for DefaultProofVerifier {
    /// Ingests a new proof into the lattice.
    fn ingest_proof(&mut self, proof: Proof) -> Result<(), &'static str> {
        if let Proof::Composite { components, .. } = &proof {
            for component_hash in components {
                if !self.proofs.contains_key(component_hash) {
                    return Err("Composite proof contains an unknown component proof.");
                }
            }
        }

        let proof_hash = proof.hash().clone();
        if self.proofs.contains_key(&proof_hash) {
            return Err("Proof already exists in the lattice.");
        }

        self.proofs.insert(proof_hash, proof);
        Ok(())
    }
}
