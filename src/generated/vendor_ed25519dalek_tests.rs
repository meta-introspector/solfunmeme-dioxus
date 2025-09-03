use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ed25519-dalek/tests"]
pub struct OurVendorEd25519DalekTestsExtractor;
