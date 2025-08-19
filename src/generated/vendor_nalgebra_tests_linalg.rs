use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nalgebra/tests/linalg"]
pub struct OurVendorNalgebraTestsLinalgExtractor;
