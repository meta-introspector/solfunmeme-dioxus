use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/rustix/src/maybe_polyfill/no_std"]
pub struct OurVendorRustixSrcMaybePolyfillNoStdExtractor;
