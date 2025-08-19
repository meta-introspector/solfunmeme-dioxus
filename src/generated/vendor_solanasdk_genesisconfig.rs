use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/genesis-config"]
pub struct OurVendorSolanaSdkGenesisConfigExtractor;
