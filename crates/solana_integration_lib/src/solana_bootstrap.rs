use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use borsh::BorshSerialize;
use anyhow::Result;
use tracing::info;

use crate::{Ontology, CodeFile, Term, BuyOrder};

pub fn bootstrap_to_solana(
    ontology_data: Ontology,
    code_files: Vec<CodeFile>,
    terms: Vec<Term>,
    buy_orders: Vec<BuyOrder>,
    program_id: Pubkey,
    payer: &Keypair,
    rpc_url: &str,
) -> Result<()> {
    let client = RpcClient::new(rpc_url.to_string());

    info!("Deploying Ontology account...");
    let (ontology_pda, _bump) = Pubkey::find_program_address(&[b"ontology", ontology_data.id.as_bytes()], &program_id);
    let ontology_instruction = Instruction {
        program_id,
        accounts: vec![AccountMeta::new(ontology_pda, false), AccountMeta::new(payer.pubkey(), true)],
        data: ontology_data.try_to_vec()?,
    };
    let mut tx = Transaction::new_with_payer(&[ontology_instruction], Some(&payer.pubkey()));
    tx.sign(&[payer], client.get_latest_blockhash()?);
    client.send_and_confirm_transaction(&tx)?;
    info!("Deployed ontology: {}", ontology_data.id);

    info!("Deploying CodeFile accounts...");
    for file in code_files {
        let (file_pda, _bump) = Pubkey::find_program_address(&[b"code_file", file.name.as_bytes()], &program_id);
        let file_instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(file_pda, false),
                AccountMeta::new(file.ontology, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
            data: file.try_to_vec()?,
        };
        let mut tx = Transaction::new_with_payer(&[file_instruction], Some(&payer.pubkey()));
        tx.sign(&[payer], client.get_latest_blockhash()?);
        client.send_and_confirm_transaction(&tx)?;
        info!("Deployed CodeFile: {}", file.name);
    }

    

    info!("Deploying Term accounts...");
    for term in terms {
        let (term_pda, _bump) = Pubkey::find_program_address(&[b"term", term.text.as_bytes()], &program_id);
        let term_instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(term_pda, false),
                AccountMeta::new(term.ontology, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
            data: term.try_to_vec()?,
        };
        let mut tx = Transaction::new_with_payer(&[term_instruction], Some(&payer.pubkey()));
        tx.sign(&[payer], client.get_latest_blockhash()?);
        client.send_and_confirm_transaction(&tx)?;
        info!("Deployed Term: {}", term.text);
    }

    info!("Deploying BuyOrder accounts...");
    for buy_order in buy_orders {
        let (buy_order_pda, _bump) = Pubkey::find_program_address(&[b"buy_order", buy_order.id.as_bytes()], &program_id);
        let buy_order_instruction = Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(buy_order_pda, false),
                AccountMeta::new(buy_order.ontology, false),
                AccountMeta::new(payer.pubkey(), true),
            ],
            data: buy_order.try_to_vec()?,
        };
        let mut tx = Transaction::new_with_payer(&[buy_order_instruction], Some(&payer.pubkey()));
        tx.sign(&[payer], client.get_latest_blockhash()?);
        client.send_and_confirm_transaction(&tx)?;
        info!("Deployed BuyOrder: {}", buy_order.id);
    }

    Ok(())
}

// Helper to derive PDA (Program Derived Address)
pub fn derive_pda(seeds: &[&[u8]], program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(seeds, program_id).0
}
