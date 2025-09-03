use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nalgebra/src"]
pub struct OurVendorNalgebraSrcExtractor;
