use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/manganis/test-package/src"]
pub struct OurVendorManganisTestPackageSrcExtractor;
