use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curve25519-dalek/x25519-dalek/src"]
pub struct OurVendorCurve25519DalekX25519DalekSrcExtractor;
