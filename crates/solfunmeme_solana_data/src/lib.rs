use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::pubkey::Pubkey;
use sha2::{Sha256, Digest};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey as SolanaPubkey, // Alias to avoid conflict with solana_program::pubkey::Pubkey
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use anyhow::Result;
use std::str::FromStr;
use tracing::{info, instrument};

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
    pub function_refs: Vec<SolanaPubkey>, // Dynamic size for now, consider fixed array for on-chain
    pub buy_order_refs: Vec<SolanaPubkey>, // Links to buy orders
    pub ontology: SolanaPubkey,
    pub creator: SolanaPubkey,
    pub timestamp: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Function {
    pub name: String, // Max 32 chars
    pub semantic_summary: String, // Max 256 chars
    pub code_snippet: String, // Max 512 chars
    pub sieve_address: String, // 8 chars
    pub embedding_hash: [u8; 32], // Hashed multivector
    pub file: SolanaPubkey,
    pub emoji_refs: Vec<SolanaPubkey>, // Dynamic size for now, consider fixed array for on-chain
    pub emoji_distances: Vec<f64>,
    pub buy_order_ref: SolanaPubkey, // Link to buy order
    pub ontology: SolanaPubkey,
    pub creator: SolanaPubkey,
    pub timestamp: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ClosestEmojiInfo {
    pub symbol: String, // Max 8 chars
    pub category: String, // Max 16 chars
    pub ontology: SolanaPubkey,
    pub creator: SolanaPubkey,
    pub timestamp: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Term {
    pub text: String, // Max 32 chars
    pub description: String, // Max 128 chars
    pub emoji_refs: Vec<SolanaPubkey>,
    pub function_refs: Vec<SolanaPubkey>,
    pub buy_order_ref: SolanaPubkey,
    pub ontology: SolanaPubkey,
    pub creator: SolanaPubkey,
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
    pub instance_ref: SolanaPubkey, // Link to Function, Term, etc.
    pub status: BuyOrderStatus, // Open, OfferReceived, Fulfilled
    pub ontology: SolanaPubkey,
    pub creator: SolanaPubkey,
    pub timestamp: i64,
}

#[instrument(skip(payer, rpc_url))]
pub fn bootstrap_to_solana(
    ontology_data: Ontology,
    code_files: Vec<CodeFile>,
    functions: Vec<Function>,
    emojis: Vec<ClosestEmojiInfo>,
    terms: Vec<Term>,
    buy_orders: Vec<BuyOrder>,
    program_id: SolanaPubkey,
    payer: &Keypair,
    rpc_url: &str,
) -> Result<()> {
    let client = RpcClient::new(rpc_url.to_string());

    info!("Deploying Ontology account...");
    let (ontology_pda, _bump) = SolanaPubkey::find_program_address(&[b"ontology", ontology_data.id.as_bytes()], &program_id);
    let ontology_instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(ontology_pda, false), AccountMeta::new(payer.pubkey(), true)],
        data: BorshSerialize::try_to_vec(&ontology_data).unwrap(),
    };
    let mut tx = Transaction::new_with_payer(&[ontology_instruction], Some(&payer.pubkey()));
    tx.sign(&[payer], client.get_latest_blockhash()?);
    client.send_and_confirm_transaction(&tx)?;
    info!("Deployed ontology: {}", ontology_data.id);

    info!("Deploying CodeFile accounts...");
    for file in code_files {
        let (file_pda, _bump) = SolanaPubkey::find_program_address(&[b"code_file", file.name.as_bytes()], &program_id);
        let file_instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(file_pda, false),
                AccountMeta::new(file.ontology, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
            data: BorshSerialize::try_to_vec(&file).unwrap(),
        };
        let mut tx = Transaction::new_with_payer(&[file_instruction], Some(&payer.pubkey()));
        tx.sign(&[payer], client.get_latest_blockhash()?);
        client.send_and_confirm_transaction(&tx)?;
        info!("Deployed CodeFile: {}", file.name);
    }

    info!("Deploying Function accounts...");
    for func in functions {
        let (func_pda, _bump) = SolanaPubkey::find_program_address(&[b"function", func.name.as_bytes()], &program_id);
        let func_instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(func_pda, false),
                AccountMeta::new(func.file, false),
                AccountMeta::new(func.ontology, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
            data: BorshSerialize::try_to_vec(&func).unwrap(),
        };
        let mut tx = Transaction::new_with_payer(&[func_instruction], Some(&payer.pubkey()));
        tx.sign(&[payer], client.get_latest_blockhash()?);
        client.send_and_confirm_transaction(&tx)?;
        info!("Deployed Function: {}", func.name);
    }

    info!("Deploying ClosestEmojiInfo accounts...");
    for emoji_info in emojis {
        let (emoji_pda, _bump) = SolanaPubkey::find_program_address(&[b"emoji_info", emoji_info.symbol.as_bytes()], &program_id);
        let emoji_instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(emoji_pda, false),
                AccountMeta::new(emoji_info.ontology, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
            data: BorshSerialize::try_to_vec(&emoji_info).unwrap(),
        };
        let mut tx = Transaction::new_with_payer(&[emoji_instruction], Some(&payer.pubkey()));
        tx.sign(&[payer], client.get_latest_blockhash()?);
        client.send_and_confirm_transaction(&tx)?;
        info!("Deployed ClosestEmojiInfo: {}", emoji_info.symbol);
    }

    info!("Deploying Term accounts...");
    for term in terms {
        let (term_pda, _bump) = SolanaPubkey::find_program_address(&[b"term", term.text.as_bytes()], &program_id);
        let term_instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(term_pda, false),
                AccountMeta::new(term.ontology, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
            data: BorshSerialize::try_to_vec(&term).unwrap(),
        };
        let mut tx = Transaction::new_with_payer(&[term_instruction], Some(&payer.pubkey()));
        tx.sign(&[payer], client.get_latest_blockhash()?);
        client.send_and_confirm_transaction(&tx)?;
        info!("Deployed Term: {}", term.text);
    }

    info!("Deploying BuyOrder accounts...");
    for buy_order in buy_orders {
        let (buy_order_pda, _bump) = SolanaPubkey::find_program_address(&[b"buy_order", buy_order.id.as_bytes()], &program_id);
        let buy_order_instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(buy_order_pda, false),
                AccountMeta::new(buy_order.ontology, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
            data: BorshSerialize::try_to_vec(&buy_order).unwrap(),
        };
        let mut tx = Transaction::new_with_payer(&[buy_order_instruction], Some(&payer.pubkey()));
        tx.sign(&[payer], client.get_latest_blockhash()?);
        client.send_and_confirm_transaction(&tx)?;
        info!("Deployed BuyOrder: {}", buy_order.id);
    }

    Ok(())
}

// Helper to derive PDA (Program Derived Address)
pub fn derive_pda(seeds: &[&[u8]], program_id: &SolanaPubkey) -> SolanaPubkey {
    SolanaPubkey::find_program_address(seeds, program_id).0
}
