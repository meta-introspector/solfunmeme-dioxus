use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/syn/tests/snapshot"]
pub struct OurVendorSynTestsSnapshotExtractor;
