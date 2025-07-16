//! Defines the `ProofVerifier` trait, which abstracts the argument of knowledge.

use crate::proof::Proof;

/// A trait that abstracts the verification and ingestion of proofs.
///
/// This allows the Kernel to be agnostic about the underlying proof system,
/// which could be a simple lattice, a ZKP verifier, or an external service.
pub trait ProofVerifier {
    /// Ingests a new proof, verifying its integrity against known proofs.
    fn ingest_proof(&mut self, proof: Proof) -> Result<(), &'static str>;
}
