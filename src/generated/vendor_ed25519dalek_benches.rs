use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ed25519-dalek/benches"]
pub struct OurVendorEd25519DalekBenchesExtractor;
