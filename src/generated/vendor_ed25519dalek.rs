use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ed25519-dalek"]
pub struct OurVendorEd25519DalekExtractor;
