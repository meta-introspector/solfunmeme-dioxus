//! Defines the `ChecksumHasher`, the default, built-in hasher for Stage 0.

use crate::hash::{Hash, Hasher};

/// The default, built-in hasher for Stage 0.
///
/// This provides a simple, dependency-free checksum implementation. It is not
/// cryptographically secure and is intended only for basic content integrity
/// checks within the initial bootstrap environment. Its primary purpose is to
/// ensure the `Kernel` has a working `Hasher` implementation upon creation
/// without requiring external plugins.
pub struct ChecksumHasher;

impl Hasher for ChecksumHasher {
    /// The algorithm ID for the `ChecksumHasher` is `0`.
    fn algorithm_id(&self) -> u64 {
        0 // 0 is reserved for the stage0 default checksum.
    }

    /// Calculates a simple checksum of the data.
    ///
    /// The checksum is the wrapping sum of all bytes in the data, cast to `u64`.
    fn hash(&self, data: &[u8]) -> Hash {
        let checksum = data
            .iter()
            .fold(0u64, |acc, &byte| acc.wrapping_add(byte as u64));
        Hash {
            algorithm_id: self.algorithm_id(),
            hash_bytes: checksum.to_be_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_hasher_determinism() {
        let hasher = ChecksumHasher;
        let data1 = b"hello world";
        let data2 = b"hello world";
        assert_eq!(hasher.hash(data1), hasher.hash(data2));
    }

    #[test]
    fn test_checksum_hasher_difference() {
        let hasher = ChecksumHasher;
        let data1 = b"hello world";
        let data2 = b"hello universe";
        assert_ne!(hasher.hash(data1), hasher.hash(data2));
    }

    #[test]
    fn test_checksum_algorithm_id() {
        let hasher = ChecksumHasher;
        assert_eq!(hasher.algorithm_id(), 0);
        assert_eq!(hasher.hash(b"").algorithm_id, 0);
    }
}
