use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tokio/target-specs"]
pub struct OurVendorTokioTargetSpecsExtractor;
