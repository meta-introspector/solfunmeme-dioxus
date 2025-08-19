use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ed25519-dalek/docs"]
pub struct OurVendorEd25519DalekDocsExtractor;
