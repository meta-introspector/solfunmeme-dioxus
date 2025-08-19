use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ed25519-dalek/src"]
pub struct OurVendorEd25519DalekSrcExtractor;
