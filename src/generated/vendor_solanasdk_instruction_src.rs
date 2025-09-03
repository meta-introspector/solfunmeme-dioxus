use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/instruction/src"]
pub struct OurVendorSolanaSdkInstructionSrcExtractor;
