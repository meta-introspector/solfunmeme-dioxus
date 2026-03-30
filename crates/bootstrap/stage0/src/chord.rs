//! Defines the `ChordStore`, an in-memory representation of a Chord ring.

use crate::hash::Hash;
use std::collections::BTreeSet;

/// An in-memory, sorted set that represents a Chord-like ring.
///
/// The ring stores the `u64` positions of all known entities. This acts as a
/// universal, content-addressable index for every artifact, proof, and component
/// in the system.
#[derive(Default)]
pub struct ChordStore {
    nodes: BTreeSet<u64>,
}

impl ChordStore {
    /// Creates a new, empty Chord store.
    pub fn new() -> Self {
        Self {
            nodes: BTreeSet::new(),
        }
    }

    /// Places a hash onto the Chord ring.
    ///
    /// Returns `true` if the hash was not already present.
    pub fn place(&mut self, hash: &Hash) -> bool {
        let position = Self::hash_to_position(hash);
        self.nodes.insert(position)
    }

    /// Checks if a hash exists on the Chord ring.
    pub fn exists(&self, hash: &Hash) -> bool {
        let position = Self::hash_to_position(hash);
        self.nodes.contains(&position)
    }

    /// Converts a `Hash` to a `u64` position on the Chord ring.
    fn hash_to_position(hash: &Hash) -> u64 {
        let bytes = &hash.hash_bytes;
        let mut arr = [0u8; 8];
        let len = std::cmp::min(bytes.len(), 8);
        arr[..len].copy_from_slice(&bytes[..len]);
        u64::from_be_bytes(arr)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::default_hasher::ChecksumHasher;

    #[test]
    fn test_store_and_retrieve() {
        let mut store = ChordStore::new();
        let hasher = ChecksumHasher;
        let content = b"chord content".to_vec();
        let artifact = Artifact::new(content, &hasher);
        let primary_hash = artifact.hashes.first().unwrap().clone();

        assert!(store.store(artifact).is_ok());
        let retrieved = store.retrieve(&primary_hash);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().content, b"chord content");
    }

    #[test]
    fn test_hash_to_position() {
        let hash1 = Hash {
            algorithm_id: 0,
            hash_bytes: vec![0, 0, 0, 0, 0, 0, 0, 1],
        };
        let hash2 = Hash {
            algorithm_id: 0,
            hash_bytes: vec![0, 0, 0, 0, 0, 0, 0, 2],
        };
        assert_eq!(ChordStore::hash_to_position(&hash1), 1);
        assert_ne!(
            ChordStore::hash_to_position(&hash1),
            ChordStore::hash_to_position(&hash2)
        );
    }
}
