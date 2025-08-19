use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/toml/crates/toml"]
pub struct OurVendorTomlCratesTomlExtractor;
