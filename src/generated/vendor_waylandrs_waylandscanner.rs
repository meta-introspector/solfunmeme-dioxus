use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wayland-rs/wayland-scanner"]
pub struct OurVendorWaylandRsWaylandScannerExtractor;
