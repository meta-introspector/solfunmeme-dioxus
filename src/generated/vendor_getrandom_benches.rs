use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/getrandom/benches"]
pub struct OurVendorGetrandomBenchesExtractor;
