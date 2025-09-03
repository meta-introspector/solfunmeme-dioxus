use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/elliptic-curves/.cargo"]
pub struct OurVendorEllipticCurvesCargoExtractor;
