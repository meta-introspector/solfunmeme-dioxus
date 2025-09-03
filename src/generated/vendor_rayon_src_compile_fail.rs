use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rayon/src/compile_fail"]
pub struct OurVendorRayonSrcCompileFailExtractor;
