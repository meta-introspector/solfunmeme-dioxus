use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nalgebra/tests/proptest"]
pub struct OurVendorNalgebraTestsProptestExtractor;
