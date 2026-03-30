#[cfg(feature = "rust-embed")]
use rust_embed::RustEmbed;

#[cfg(feature = "rust-embed")]
#[derive(RustEmbed)]
#[folder = "../../.."] // Relative to crates/solfunmeme_tools/Cargo.toml
#[include = "README.md"]
pub struct ProjectAssets;

#[cfg(not(feature = "rust-embed"))]
pub struct ProjectAssets;

#[cfg(not(feature = "rust-embed"))]
impl ProjectAssets {
    pub fn get(_name: &str) -> Option<Vec<u8>> {
        None
    }
}
