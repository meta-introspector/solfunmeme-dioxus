use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/matrixmultiply/blas-bench/src"]
pub struct OurVendorMatrixmultiplyBlasBenchSrcExtractor;
