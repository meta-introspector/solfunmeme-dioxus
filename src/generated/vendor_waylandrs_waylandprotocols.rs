use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wayland-rs/wayland-protocols"]
pub struct OurVendorWaylandRsWaylandProtocolsExtractor;
