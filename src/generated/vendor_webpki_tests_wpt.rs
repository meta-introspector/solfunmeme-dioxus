use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/webpki/tests/wpt"]
pub struct OurVendorWebpkiTestsWptExtractor;
