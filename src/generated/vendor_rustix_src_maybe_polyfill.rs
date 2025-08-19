use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/maybe_polyfill"]
pub struct OurVendorRustixSrcMaybePolyfillExtractor;
