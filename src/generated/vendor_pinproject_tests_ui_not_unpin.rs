use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/pin-project/tests/ui/not_unpin"]
pub struct OurVendorPinProjectTestsUiNotUnpinExtractor;
