use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/solana-sdk/package-metadata"]
pub struct OurVendorSolanaSdkPackageMetadataExtractor;
