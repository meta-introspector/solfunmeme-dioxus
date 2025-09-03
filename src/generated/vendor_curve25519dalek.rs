use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/curve25519-dalek"]
pub struct OurVendorCurve25519DalekExtractor;
