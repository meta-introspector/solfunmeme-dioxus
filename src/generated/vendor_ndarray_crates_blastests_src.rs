use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ndarray/crates/blas-tests/src"]
pub struct OurVendorNdarrayCratesBlasTestsSrcExtractor;
