use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/mio/src/sys/unix/waker"]
pub struct OurVendorMioSrcSysUnixWakerExtractor;
