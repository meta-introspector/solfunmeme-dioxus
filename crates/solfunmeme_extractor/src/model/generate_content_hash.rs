use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Generate a simple hash for content deduplication
pub fn generate_content_hash(content: &str) -> String {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
