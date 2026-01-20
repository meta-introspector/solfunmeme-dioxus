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
        console::log_1(&format!("🔍 Fetching block {}", slot).into());
        log_to_server("info", &format!("Fetching block {}", slot));
        
        let block = Block {
            slot: slot.to_string(),
            hash: format!("hash_{}", slot),
            transactions: vec![format!("tx_{}", slot)],
            timestamp: (js_sys::Date::now() as u64).to_string(),
        };
        
        log_to_server("info", &format!("Block {} fetched", slot));
        serde_wasm_bindgen::to_value(&block).map_err(|e| JsValue::from_str(&e.to_string()))
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
