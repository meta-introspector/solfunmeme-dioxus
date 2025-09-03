use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/tls/tokio-native-tls"]
pub struct OurVendorTlsTokioNativeTlsExtractor;
