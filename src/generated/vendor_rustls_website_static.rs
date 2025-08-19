use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustls/website/static"]
pub struct OurVendorRustlsWebsiteStaticExtractor;
