use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/chrono/src/offset/local/tz_info"]
pub struct OurVendorChronoSrcOffsetLocalTzInfoExtractor;
