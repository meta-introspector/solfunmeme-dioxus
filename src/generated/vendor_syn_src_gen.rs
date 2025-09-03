use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/syn/src/gen"]
pub struct OurVendorSynSrcGenExtractor;
