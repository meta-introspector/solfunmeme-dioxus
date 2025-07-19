use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;


#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Ontology {
    pub id: String, // e.g., "prepare_sources_ontology"
    pub classes: Vec<String>, // e.g., ["CodeFile", "Function", "ClosestEmojiInfo", "Term"]
    pub properties: Vec<String>, // e.g., ["isInFile", "hasCodeSnippet"]
    pub creator: Pubkey,
    pub timestamp: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct CodeFile {
    pub name: String, // Max 32 chars
    pub path: String, // Max 64 chars
    pub function_refs: Vec<Pubkey>, // Dynamic size for now, consider fixed array for on-chain
    pub buy_order_refs: Vec<Pubkey>, // Links to buy orders
    pub ontology: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
}



#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Term {
    pub text: String, // Max 32 chars
    pub description: String, // Max 128 chars
    pub emoji_refs: Vec<Pubkey>,
    pub function_refs: Vec<Pubkey>,
    pub buy_order_ref: Pubkey,
    pub ontology: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum BuyOrderStatus {
    Open,
    OfferReceived,
    Fulfilled,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct BuyOrder {
    pub id: String, // Unique ID (e.g., "buyorder_analyze_project")
    pub source_chunk: String, // Code snippet (e.g., function code)
    pub vectorized_result: [u8; 32], // Hashed embedding
    pub instance_ref: Pubkey, // Link to Function, Term, etc.
    pub status: BuyOrderStatus, // Open, OfferReceived, Fulfilled
    pub ontology: Pubkey,
    pub creator: Pubkey,
    pub timestamp: i64,
}

pub mod solana_bootstrap;
