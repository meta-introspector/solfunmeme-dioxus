use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/mio/src/sys/unix/uds"]
pub struct OurVendorMioSrcSysUnixUdsExtractor;
