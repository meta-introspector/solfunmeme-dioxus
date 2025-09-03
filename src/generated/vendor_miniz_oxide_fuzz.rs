use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/miniz_oxide/fuzz"]
pub struct OurVendorMinizOxideFuzzExtractor;
