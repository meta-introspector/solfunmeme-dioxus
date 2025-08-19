use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ed25519-dalek/tests/examples"]
pub struct OurVendorEd25519DalekTestsExamplesExtractor;
