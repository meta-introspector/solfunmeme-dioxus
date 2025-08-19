use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/bumpalo/benches"]
pub struct OurVendorBumpaloBenchesExtractor;
