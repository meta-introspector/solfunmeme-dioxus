use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nalgebra/tests"]
pub struct OurVendorNalgebraTestsExtractor;
