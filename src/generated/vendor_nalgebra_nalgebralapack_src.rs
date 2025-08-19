use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/nalgebra/nalgebra-lapack/src"]
pub struct OurVendorNalgebraNalgebraLapackSrcExtractor;
