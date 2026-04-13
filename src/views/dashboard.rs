use dioxus::prelude::*;

const TOKEN_ADDRESS: &str = "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump";

#[derive(Clone, PartialEq, Debug)]
pub struct TokenTransaction {
    pub signature: String,
    pub block_time: Option<i64>,
    pub slot: u64,
    pub status: String,
}

#[component]
pub fn Dashboard() -> Element {
    let mut transactions: Signal<Vec<TokenTransaction>> = use_signal(Vec::new);
    let mut loading = use_signal(|| false);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut token_supply: Signal<Option<String>> = use_signal(|| None);

    let fetch_token_data = move |_| {
        spawn(async move {
            loading.set(true);
            error_msg.set(None);

            // Fetch token supply
            let rpc_url = "https://api.mainnet-beta.solana.com";
            let supply_body = format!(
                r#"{{"jsonrpc":"2.0","id":1,"method":"getTokenSupply","params":["{}"]}}"#,
                TOKEN_ADDRESS
            );

            #[cfg(target_arch = "wasm32")]
            {
                use gloo_net::http::Request;
                match Request::post(rpc_url)
                    .header("Content-Type", "application/json")
                    .body(supply_body)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if let Ok(text) = resp.text().await {
                            // Parse ui_amount_string from response
                            if let Some(start) = text.find("\"uiAmountString\":\"") {
                                let rest = &text[start + 18..];
                                if let Some(end) = rest.find('"') {
                                    token_supply.set(Some(rest[..end].to_string()));
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Supply fetch error: {e}")));
                    }
                }

                // Fetch signatures for the token mint address
                let sigs_body = format!(
                    r#"{{"jsonrpc":"2.0","id":1,"method":"getSignaturesForAddress","params":["{}",{{"limit":25}}]}}"#,
                    TOKEN_ADDRESS
                );

                match Request::post(rpc_url)
                    .header("Content-Type", "application/json")
                    .body(sigs_body)
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(resp) => {
                        if let Ok(text) = resp.text().await {
                            let mut txs = Vec::new();
                            // Simple manual parse of signature array
                            let mut search = text.as_str();
                            while let Some(sig_start) = search.find("\"signature\":\"") {
                                let rest = &search[sig_start + 13..];
                                if let Some(sig_end) = rest.find('"') {
                                    let sig = rest[..sig_end].to_string();

                                    // Extract slot
                                    let slot = if let Some(sl) = search.find("\"slot\":") {
                                        let sl_rest = &search[sl + 7..];
                                        let sl_end = sl_rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(sl_rest.len());
                                        sl_rest[..sl_end].parse::<u64>().unwrap_or(0)
                                    } else { 0 };

                                    // Extract blockTime
                                    let block_time = if let Some(bt) = search.find("\"blockTime\":") {
                                        let bt_rest = &search[bt + 12..];
                                        let bt_end = bt_rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(bt_rest.len());
                                        bt_rest[..bt_end].parse::<i64>().ok()
                                    } else { None };

                                    txs.push(TokenTransaction {
                                        signature: sig,
                                        block_time,
                                        slot,
                                        status: "confirmed".to_string(),
                                    });

                                    search = &rest[sig_end..];
                                } else {
                                    break;
                                }
                            }
                            transactions.set(txs);
                        }
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Signatures fetch error: {e}")));
                    }
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                error_msg.set(Some("Token fetching only available on WASM/Android".to_string()));
            }

            loading.set(false);
        });
    };

    rsx! {
        div {
            class: "flex flex-col w-full min-h-screen bg-rich-black text-white p-4",

            // Header card
            div {
                class: "flex flex-col w-full rounded-xl border border-true-blue bg-[rgba(220,20,60,0.08)] p-5 mb-6",
                div { class: "flex items-center mb-2",
                    span { class: "text-true-blue text-2xl font-bold mr-2", "◈" }
                    h1 { class: "text-white text-xl font-bold tracking-tight", "Token Explorer" }
                }
                p { class: "text-gray-400 text-xs font-mono break-all", "{TOKEN_ADDRESS}" }
                if let Some(supply) = token_supply.read().clone() {
                    div { class: "mt-3 flex items-center gap-2",
                        span { class: "text-gray-400 text-sm", "Total Supply:" }
                        span { class: "text-true-blue font-bold text-lg", "{supply}" }
                    }
                }
            }

            // Fetch button
            button {
                onclick: fetch_token_data,
                disabled: *loading.read(),
                class: "w-full mb-6 py-3 rounded-xl bg-true-blue hover:bg-cobalt-blue text-white font-bold text-base tracking-wide transition-all",
                if *loading.read() {
                    "⟳  Fetching block data..."
                } else {
                    "⬇  Fetch Token Transactions"
                }
            }

            // Error
            if let Some(err) = error_msg.read().clone() {
                div { class: "w-full mb-4 p-3 rounded-lg bg-[rgba(220,20,60,0.2)] border border-true-blue text-red-300 text-sm",
                    "{err}"
                }
            }

            // Transaction list
            if !transactions.read().is_empty() {
                div { class: "flex flex-col gap-3",
                    h2 { class: "text-true-blue font-semibold text-base mb-1",
                        "Recent Transactions ({transactions.read().len()})"
                    }
                    for tx in transactions.read().iter() {
                        TxRow {
                            signature: tx.signature.clone(),
                            block_time: tx.block_time,
                            slot: tx.slot,
                            status: tx.status.clone(),
                        }
                    }
                }
            } else if !*loading.read() {
                div { class: "flex flex-col items-center justify-center py-16 text-gray-600",
                    span { class: "text-4xl mb-3", "◈" }
                    p { class: "text-sm", "No data yet — tap Fetch to load transactions" }
                }
            }
        }
    }
}

#[component]
fn TxRow(signature: String, block_time: Option<i64>, slot: u64, status: String) -> Element {
    let short_sig = if signature.len() > 16 {
        format!("{}…{}", &signature[..8], &signature[signature.len()-8..])
    } else {
        signature.clone()
    };

    let time_str = block_time
        .map(|t| {
            let secs = t % 60;
            let mins = (t / 60) % 60;
            let hours = (t / 3600) % 24;
            format!("block time: {}h {}m {}s", hours, mins, secs)
        })
        .unwrap_or_else(|| "pending".to_string());

    rsx! {
        div {
            class: "flex flex-col w-full rounded-lg border border-[rgba(220,20,60,0.3)] bg-[rgba(220,20,60,0.05)] p-3 hover:border-true-blue transition-all",
            div { class: "flex justify-between items-center mb-1",
                span { class: "font-mono text-xs text-blue-yonder", "{short_sig}" }
                span {
                    class: "text-xs px-2 py-0.5 rounded-full bg-[rgba(220,20,60,0.2)] text-true-blue",
                    "{status}"
                }
            }
            div { class: "flex justify-between items-center",
                span { class: "text-gray-500 text-xs", "slot {slot}" }
                span { class: "text-gray-500 text-xs", "{time_str}" }
            }
        }
    }
}
