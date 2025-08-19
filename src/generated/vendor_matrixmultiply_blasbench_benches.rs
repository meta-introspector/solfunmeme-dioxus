use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/matrixmultiply/blas-bench/benches"]
pub struct OurVendorMatrixmultiplyBlasBenchBenchesExtractor;
