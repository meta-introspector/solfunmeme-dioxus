use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ed25519-dalek-bip32/src"]
pub struct OurVendorEd25519DalekBip32SrcExtractor;
