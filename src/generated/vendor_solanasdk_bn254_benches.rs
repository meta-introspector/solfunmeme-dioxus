use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/bn254/benches"]
pub struct OurVendorSolanaSdkBn254BenchesExtractor;
