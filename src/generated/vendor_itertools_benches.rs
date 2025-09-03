use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/itertools/benches"]
pub struct OurVendorItertoolsBenchesExtractor;
