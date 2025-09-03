use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/slot-hashes"]
pub struct OurVendorSolanaSdkSlotHashesExtractor;
