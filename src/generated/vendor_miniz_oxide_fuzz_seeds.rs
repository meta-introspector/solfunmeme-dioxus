use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/miniz_oxide/fuzz/seeds"]
pub struct OurVendorMinizOxideFuzzSeedsExtractor;
