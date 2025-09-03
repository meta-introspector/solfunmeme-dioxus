use rust_embed::Embed;

#[derive(Embed)]
#[folder = "vendor/wayland-rs/wayland-sys/src"]
pub struct OurVendorWaylandRsWaylandSysSrcExtractor;
