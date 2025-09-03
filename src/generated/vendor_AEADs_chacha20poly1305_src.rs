use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/AEADs/chacha20poly1305/src"]
pub struct OurVendorAEADsChacha20poly1305SrcExtractor;
