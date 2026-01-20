use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

const CONTRACT: &str = "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump";

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub slot: u64,
    pub hash: String,
    pub transactions: Vec<String>,
    pub timestamp: u64,
}

#[wasm_bindgen]
pub struct SolanaP2P {
    peer_id: String,
}

#[wasm_bindgen]
impl SolanaP2P {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            peer_id: format!("peer_{}", js_sys::Math::random()),
        }
    }
    
    pub fn name(&self) -> String {
        "solana-p2p".to_string()
    }
    
    pub async fn get_block(&self, slot: u64) -> Result<JsValue, JsValue> {
        // Fetch from P2P network (or fallback to RPC)
        let block = Block {
            slot,
            hash: format!("hash_{}", slot),
            transactions: vec![],
            timestamp: js_sys::Date::now() as u64,
        };
        
        Ok(serde_wasm_bindgen::to_value(&block)?)
    }
    
    pub async fn get_signatures(&self) -> Result<JsValue, JsValue> {
        // Get signatures for SOLFUNMEME contract
        let sigs = vec!["sig1", "sig2", "sig3"];
        Ok(serde_wasm_bindgen::to_value(&sigs)?)
    }
    
    pub fn get_contract(&self) -> String {
        CONTRACT.to_string()
    }
}
