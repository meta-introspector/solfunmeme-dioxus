use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nalgebra/benches/linalg"]
pub struct OurVendorNalgebraBenchesLinalgExtractor;
