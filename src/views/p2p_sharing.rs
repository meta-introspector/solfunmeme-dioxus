use dioxus::prelude::*;
use crate::model::erdfa::TOKEN_CA;

/// Known deployment endpoints for mesh discovery
const MESH_PEERS: &[(&str, &str)] = &[
    ("github-pages", "https://meta-introspector.github.io/solfunmeme-dioxus/"),
    ("cloudflare", "https://solfunmeme-dioxus.pages.dev/"),
    ("vercel", "https://solfunmeme-dioxus.vercel.app/"),
    ("huggingface", "https://introspector-solfunmeme-dioxus.hf.space/"),
    ("oracle-oci", "https://objectstorage.us-ashburn-1.oraclecloud.com/n/id1iqr236pdp/b/solfunmeme-dioxus/o/index.html"),
    ("netlify", "https://solfunmeme.netlify.app/"),
    ("render", "https://solfunmeme-dioxus.onrender.com/"),
    ("self-hosted", "https://solana.solfunmeme.com/dioxus/"),
];

/// P2P data sharing — connect wallet, share/receive tx data, stego storage
#[component]
pub fn P2pSharing() -> Element {
    let mut peer_count = use_signal(|| 0u32);
    let mut shared_count = use_signal(|| 0u32);
    let mut received_count = use_signal(|| 0u32);
    let mut stego_status = use_signal(|| "Not connected".to_string());
    let mut mesh_results = use_signal(|| Vec::<(String, String, bool)>::new());
    let mut scanning = use_signal(|| false);

    rsx! {
        div { class: "p2p-view",
            h2 { "🌐 P2P Data Network" }

            div { class: "p2p-status",
                h3 { "Network Status" }
                table {
                    tr { td { "Peers" } td { "{peer_count}" } }
                    tr { td { "Shared" } td { "{shared_count} tx" } }
                    tr { td { "Received" } td { "{received_count} tx" } }
                    tr { td { "Stego Storage" } td { "{stego_status}" } }
                    tr { td { "Mesh Nodes" } td { "{MESH_PEERS.len()}" } }
                }
            }

            div { class: "p2p-mesh",
                h3 { "🕸️ Deployment Mesh ({MESH_PEERS.len()} nodes)" }
                button {
                    disabled: *scanning.read(),
                    onclick: move |_| {
                        scanning.set(true);
                        mesh_results.set(Vec::new());
                        // Ping each deployment
                        for &(name, url) in MESH_PEERS {
                            let name = name.to_string();
                            let url = url.to_string();
                            let mut results = mesh_results.clone();
                            let mut peers = peer_count.clone();
                            let mut scan = scanning.clone();
                            spawn(async move {
                                let ok = match gloo_net::http::Request::get(&url)
                                    .send().await {
                                    Ok(r) => r.ok(),
                                    Err(_) => false,
                                };
                                results.write().push((name, url, ok));
                                if ok { peers += 1; }
                                if results.read().len() == MESH_PEERS.len() {
                                    scan.set(false);
                                }
                            });
                        }
                    },
                    if *scanning.read() { "⏳ Scanning..." } else { "🔍 Scan Mesh" }
                }
                table {
                    tr { th { "Node" } th { "Status" } th { "URL" } }
                    for (name, url, ok) in mesh_results.read().iter() {
                        tr {
                            td { "{name}" }
                            td { if *ok { "✅" } else { "❌" } }
                            td { a { href: "{url}", target: "_blank", "{url}" } }
                        }
                    }
                }
            }

            div { class: "p2p-wallet",
                h3 { "🔑 Wallet Connection" }
                p { "Connect your Solana wallet to:" }
                ul {
                    li { "Prove token holdings (ZK freshness)" }
                    li { "Sign NFT credentials for voting" }
                    li { "Submit tx data and earn bounties" }
                    li { "Store votes in stego images" }
                }
                p { class: "token-info", "Token: {TOKEN_CA}" }
            }

            div { class: "p2p-actions",
                h3 { "Actions" }
                button {
                    class: "btn-share",
                    onclick: move |_| {
                        shared_count += 1;
                        stego_status.set("Encoding to stego...".into());
                    },
                    "📤 Share TX Data"
                }
                button {
                    class: "btn-receive",
                    onclick: move |_| {
                        received_count += 1;
                    },
                    "📥 Receive TX Data"
                }
                button {
                    class: "btn-stego",
                    onclick: move |_| {
                        stego_status.set("Stego storage active".into());
                    },
                    "🖼️ Stego Encode"
                }
            }

            div { class: "p2p-channels",
                h3 { "📡 Channels" }
                table {
                    tr { th { "Channel" } th { "Type" } th { "Status" } }
                    tr { td { "Telegram" } td { "Public" } td { "Agent scraping" } }
                    tr { td { "Discord" } td { "Public" } td { "Agent scraping" } }
                    tr { td { "WireGuard" } td { "Private" } td { "Stego tunnel" } }
                    tr { td { "Direct API" } td { "HTTP" } td { "POST /solfunmeme/paste" } }
                    tr { td { "RabbitMQ" } td { "AMQP" } td { "solfunmeme exchange" } }
                    tr { td { "HuggingFace" } td { "Dataset" } td { "introspector/solfunmeme" } }
                }
            }
        }
    }
}

crate::register_plugin!("p2p_sharing", "P2P data sharing + stego", crate::plugin::PluginCategory::Data, "🌐", || rsx!{ div{"plugin"} });
