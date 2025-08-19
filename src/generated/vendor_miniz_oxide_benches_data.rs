use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/miniz_oxide/benches/data"]
pub struct OurVendorMinizOxideBenchesDataExtractor;
