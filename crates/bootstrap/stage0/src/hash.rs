//! Defines the core `Hash` struct and `Hasher` trait for content-addressable identification.

/// A generic representation of a cryptographic hash.
///
/// This struct allows the system to be agnostic about the specific hashing
/// algorithm used. It stores an identifier for the algorithm and the resulting
/// hash bytes, similar in spirit to the `multihash` standard.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hash {
    /// A unique identifier for the hashing algorithm (e.g., a registered ID
    /// for SHA-256, BLAKE3, etc.).
    pub algorithm_id: u64,
    /// The raw byte output of the hashing algorithm.
    pub hash_bytes: Vec<u8>,
}

/// A trait defining the contract for a pluggable hashing algorithm.
///
/// Implementors of this trait provide a specific hashing implementation that
/// can be dynamically loaded by the microkernel. This allows the system's
/// cryptographic foundations to be upgraded over time.
pub trait Hasher {
    /// Returns the unique identifier for this hashing algorithm.
    fn algorithm_id(&self) -> u64;

    /// Hashes the given data and returns a `Hash` struct.
    ///
    /// # Arguments
    ///
    /// * `data` - A byte slice representing the content to be hashed.
    fn hash(&self, data: &[u8]) -> Hash;
}
