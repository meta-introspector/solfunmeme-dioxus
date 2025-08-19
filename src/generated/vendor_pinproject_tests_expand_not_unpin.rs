use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pin-project/tests/expand/not_unpin"]
pub struct OurVendorPinProjectTestsExpandNotUnpinExtractor;
