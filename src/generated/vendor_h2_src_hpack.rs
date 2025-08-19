use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/h2/src/hpack"]
pub struct OurVendorH2SrcHpackExtractor;
