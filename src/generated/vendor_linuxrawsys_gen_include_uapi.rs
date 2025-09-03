use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/linux-raw-sys/gen/include/uapi"]
pub struct OurVendorLinuxRawSysGenIncludeUapiExtractor;
