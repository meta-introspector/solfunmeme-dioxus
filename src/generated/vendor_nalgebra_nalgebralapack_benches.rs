use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nalgebra/nalgebra-lapack/benches"]
pub struct OurVendorNalgebraNalgebraLapackBenchesExtractor;
