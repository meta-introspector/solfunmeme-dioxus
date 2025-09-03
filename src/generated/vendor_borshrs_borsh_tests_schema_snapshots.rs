use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/borsh-rs/borsh/tests/schema/snapshots"]
pub struct OurVendorBorshRsBorshTestsSchemaSnapshotsExtractor;
