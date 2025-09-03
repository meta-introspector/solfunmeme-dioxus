use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/ring-compat/tests/digest/data"]
pub struct OurVendorRingCompatTestsDigestDataExtractor;
