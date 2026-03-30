//! Defines the `Artifact` struct, a fundamental, content-addressable unit.

//! Defines the `Artifact` struct, a fundamental, content-addressable unit.

use crate::describable::Describable;
use crate::hash::{Hash, Hasher};

/// Represents a fundamental, content-addressable unit in the system.
///
/// An artifact's identity is determined by its content, verified by a set of
/// one or more cryptographic hashes. It is `Describable`, meaning its canonical
/// representation for hashing is its raw content.
#[derive(Debug, Clone)]
pub struct Artifact {
    /// The raw binary content of the artifact.
    pub content: Vec<u8>,
    /// A list of hashes that identify the content. This allows for multiple
    /// hashing algorithms to be used simultaneously.
    pub hashes: Vec<Hash>,
}

impl Describable for Artifact {
    /// The canonical description of an artifact is its raw content.
    fn describe(&self) -> &[u8] {
        &self.content
    }
}

impl Artifact {
    /// Creates a new artifact from its content, using the provided hasher
    /// to generate its initial hash.
    pub fn new(content: Vec<u8>, hasher: &dyn Hasher) -> Self {
        // The hash is calculated from the canonical description of the artifact.
        let hash = hasher.hash(content.as_slice());
        Self {
            content,
            hashes: vec![hash],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::default_hasher::ChecksumHasher;

    #[test]
    fn test_artifact_new() {
        let hasher = ChecksumHasher;
        let content = b"test content".to_vec();
        let artifact = Artifact::new(content.clone(), &hasher);

        assert_eq!(artifact.content, content);
        assert_eq!(artifact.hashes.len(), 1);
        assert_eq!(artifact.hashes[0], hasher.hash(&content));
    }

    #[test]
    fn test_artifact_is_describable() {
        let content = b"describable content".to_vec();
        let artifact = Artifact::new(content.clone(), &ChecksumHasher);
        assert_eq!(artifact.describe(), content.as_slice());
    }
}
