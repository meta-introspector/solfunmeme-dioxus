use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/half-rs/benches"]
pub struct OurVendorHalfRsBenchesExtractor;
