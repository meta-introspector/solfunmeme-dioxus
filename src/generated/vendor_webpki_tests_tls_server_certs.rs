use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/webpki/tests/tls_server_certs"]
pub struct OurVendorWebpkiTestsTlsServerCertsExtractor;
