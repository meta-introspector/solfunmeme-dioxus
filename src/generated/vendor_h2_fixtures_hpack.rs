use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/h2/fixtures/hpack"]
pub struct OurVendorH2FixturesHpackExtractor;
