use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/signatures/dsa/tests/pems"]
pub struct OurVendorSignaturesDsaTestsPemsExtractor;
