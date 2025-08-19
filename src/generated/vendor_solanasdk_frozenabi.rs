use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/frozen-abi"]
pub struct OurVendorSolanaSdkFrozenAbiExtractor;
