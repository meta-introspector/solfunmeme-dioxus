use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wayland-rs/wayland-sys"]
pub struct OurVendorWaylandRsWaylandSysExtractor;
