use dioxus::prelude::*;

mod plugin_loader;

fn main() {
    dioxus::launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    let mut node_url = use_signal(|| String::from("starting..."));
    let mut node_status = use_signal(|| String::from(""));

    // Spawn embedded node on non-wasm platforms
    #[cfg(not(target_arch = "wasm32"))]
    use_effect(move || {
        spawn(async move {
            let url = solfunmeme_dioxus::node::server::start(8080).await;
            node_url.set(url.clone());
            node_status.set(format!("Node running at {}", url));
        });
    });

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        node_url.set("wasm (no local server)".into());
        node_status.set("Running in browser — connect to a peer node".into());
    });

    rsx! {
        div { class: "app",
            h1 { "SOLFUNMEME v2" }
            p { class: "status", "{node_status}" }

            div { class: "services",
                h2 { "📡 Node Services" }
                ul {
                    li { a { href: "{node_url}/status", "GET /status — node info" } }
                    li { a { href: "{node_url}/zkperf", "GET /zkperf — witness" } }
                    li { a { href: "{node_url}/peers", "GET /peers — P2P peers" } }
                    li { "POST /paste — create paste" }
                    li { "POST /stego/encode — ZWC steganography" }
                    li { "POST /stego/decode — decode stego" }
                }
            }

            div { class: "info",
                p { "🔢 Crown product: 47 × 59 × 71 = 196,883" }
                p { "📱 Browsers can connect to this device's server" }
                p { "🔗 P2P: libp2p mDNS + gossipsub" }
            }
        }
    }
}
