use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nalgebra/benches"]
pub struct OurVendorNalgebraBenchesExtractor;
