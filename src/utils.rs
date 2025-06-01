use dioxus::prelude::*;
use qrcodegen::{QrCode, QrCodeEcc};
use wallet_adapter::{wasm_bindgen_futures::JsFuture, Cluster, WalletResult};

use crate::{DevnetSvg, LocalnetSvg, MainnetSvg, TestnetSvg, CLUSTER_STORAGE, WINDOW};

pub fn trunk_cluster_name(name: &str) -> String {
    if name.len() > 10 {
        name.chars().take(10).collect::<String>() + "..."
    } else {
        name.to_string()
    }
}

pub fn get_cluster_svg(cluster: Cluster) -> fn() -> Result<VNode, RenderError> {
    if cluster == Cluster::MainNet {
        MainnetSvg
    } else if cluster == Cluster::TestNet {
        TestnetSvg
    } else if cluster == Cluster::DevNet {
        DevnetSvg
    } else if cluster == Cluster::LocalNet {
        LocalnetSvg
    } else {
        DevnetSvg
    }
}

const EXPLORER: &str = "https://explorer.solana.com/";

pub fn format_address_url(address: &str) -> String {
    String::new() + EXPLORER + "address/" + address + &adapter_query_string()
}

pub fn format_tx_url(tx: &str) -> String {
    String::new() + EXPLORER + "tx/" + tx + &adapter_query_string()
}

pub fn adapter_query_string() -> String {
    CLUSTER_STORAGE
        .read()
        .active_cluster()
        .query_string()
        .to_owned()
}

pub(crate) fn link_target_blank(href: &str, text: &str) -> Element {
    rsx! {a {class:"underline", href, target:"_blank", rel:"noopener noreferrer", {text}"⇗"}}
}

pub async fn copied_address(address: &str) -> WalletResult<()> {
    let pending: JsFuture = WINDOW
        .read()
        .navigator()
        .clipboard()
        .write_text(address)
        .into();

    pending.await?;

    Ok(())
}

// Creates a single QR Code, then prints it to the console.
pub fn address_qrcode(address: &str) -> WalletResult<Element> {
    let errcorlvl: QrCodeEcc = QrCodeEcc::High; // Error correction level

    // Make and print the QR Code symbol
    let qr: QrCode = QrCode::encode_text(address, errcorlvl).unwrap();
    Ok(qr_to_svg(&qr, 4))
}

fn qr_to_svg(qr: &QrCode, border: i32) -> Element {
    assert!(border >= 0, "Border must be non-negative");

    let mut path_d = String::new();
    let dimension = qr.size() + border * 2;

    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    path_d += " ";
                }
                path_d += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }

    rsx! {
        svg {
            view_box: "0 0 {dimension} {dimension}",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: path_d,
                stroke_width: "1.32",
            }
        }
    }
}
