use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/crypto-bigint/benches"]
pub struct OurVendorCryptoBigintBenchesExtractor;
