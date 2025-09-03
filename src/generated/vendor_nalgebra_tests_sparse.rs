use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nalgebra/tests/sparse"]
pub struct OurVendorNalgebraTestsSparseExtractor;
