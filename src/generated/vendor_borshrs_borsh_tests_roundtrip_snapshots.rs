use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/borsh-rs/borsh/tests/roundtrip/snapshots"]
pub struct OurVendorBorshRsBorshTestsRoundtripSnapshotsExtractor;
