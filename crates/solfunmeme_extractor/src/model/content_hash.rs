use sha2::{Digest, Sha256};

/// Creates a content-addressable hash for a code snippet.  
pub fn create_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}
