use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::console;

const CONTRACT: &str = "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump";

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub slot: String,
    pub hash: String,
    pub transactions: Vec<String>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
struct ClientLog {
    level: String,
    message: String,
    timestamp: String,
    context: Option<String>,
}

// Send telemetry to server
fn log_to_server(level: &str, message: &str) {
    console::log_1(&format!("📤 {}: {}", level, message).into());
    // TODO: Implement actual HTTP POST to /api/telemetry/log
}

#[wasm_bindgen]
pub struct SolanaP2P {
    peer_id: String,
}

#[wasm_bindgen]
impl SolanaP2P {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console::log_1(&"📱 SolanaP2P plugin initialized".into());
        log_to_server("info", "Plugin loaded");
        
        Self {
            peer_id: format!("peer_{}", js_sys::Math::random()),
        }
    }
    
    pub fn name(&self) -> String {
        "solana-p2p".to_string()
    }
    
    pub async fn get_block(&self, slot: u64) -> Result<JsValue, JsValue> {
        console::log_1(&format!("🔍 Fetching block {} from Solana RPC", slot).into());
        log_to_server("info", &format!("Fetching block {}", slot));
        
        // Real Solana RPC call
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let request_body = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"getBlock","params":[{}, {{"encoding":"json","maxSupportedTransactionVersion":0}}]}}"#,
            slot
        );
        
        let opts = web_sys::RequestInit::new();
        opts.set_method("POST");
        opts.set_body(&JsValue::from_str(&request_body));
        
        let headers = web_sys::Headers::new().map_err(|e| JsValue::from_str("Failed to create headers"))?;
        headers.set("Content-Type", "application/json").map_err(|e| JsValue::from_str("Failed to set header"))?;
        opts.set_headers(&headers);
        
        let window = web_sys::window().ok_or(JsValue::from_str("No window"))?;
        let request = web_sys::Request::new_with_str_and_init(rpc_url, &opts)
            .map_err(|e| JsValue::from_str("Failed to create request"))?;
        
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| JsValue::from_str(&format!("Fetch failed: {:?}", e)))?;
        
        let resp: web_sys::Response = resp_value.dyn_into()
            .map_err(|e| JsValue::from_str("Not a response"))?;
        
        let json = wasm_bindgen_futures::JsFuture::from(resp.json().map_err(|e| JsValue::from_str("JSON parse failed"))?)
            .await
            .map_err(|e| JsValue::from_str(&format!("JSON await failed: {:?}", e)))?;
        
        log_to_server("info", &format!("Block {} fetched from RPC", slot));
        Ok(json)
    }
    
    pub async fn get_signatures(&self) -> Result<JsValue, JsValue> {
        console::log_1(&"🔍 Fetching signatures".into());
        log_to_server("info", "Fetching signatures");
        
        let sigs = vec!["sig1", "sig2", "sig3"];
        Ok(serde_wasm_bindgen::to_value(&sigs)?)
    }
    
    pub fn get_contract(&self) -> String {
        CONTRACT.to_string()
    }
}
