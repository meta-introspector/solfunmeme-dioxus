use std::str::FromStr;

use dioxus::prelude::*;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, system_instruction::transfer,
    transaction::Transaction,
};
//use solana_transaction_error::TransactionError;
use wallet_adapter::{
    web_sys::{js_sys::Date, wasm_bindgen::JsValue},
    SendOptions, WalletError, WalletResult,
};

use crate::{model::{storage::{ACCOUNT_STATE, CLUSTER_STORAGE, WALLET_ADAPTER}, AccountState, BlockHashResponseValue, ResponseWithContext, RpcResponse, SignaturesResponse, TokenAccountResponse}, FetchReq};
//, views::FetchReq
//, WALLET_ADAPTER, CLUSTER_STORAGE

//ACCOUNT_STATE, CLUSTER_STORAGE, WALLET_ADAPTER};

pub fn format_timestamp(unix_timestamp: i64) -> String {
    let timestamp_ms = unix_timestamp as f64 * 1000.0; //Convert seconds to millisconds

    let js_date = Date::new(&JsValue::from_f64(timestamp_ms));

    js_date
        .to_string()
        .as_string()
        .unwrap_or("Invalid Timestamp".to_string())
}

async fn get_blockhash() -> WalletResult<solana_sdk::hash::Hash> {
    let options = jzon::object! {
        "id":1,
        "jsonrpc":"2.0",
        "method":"getLatestBlockhash",
        "params":[

        ]
    }
    .to_string();

    // NOTE: You can use Reqwest crate instead to fetch the blockhash but
    // this code shows how to use the browser `fetch` api

    let response_body = FetchReq::new("POST")?
        .add_header("content-type", "application/json")?
        .add_header("Accept", "application/json")?
        .set_body(&options)
        .send()
        .await?;

    let deser = serde_json::from_str::<RpcResponse<ResponseWithContext<BlockHashResponseValue>>>(
        &response_body,
    )
    .unwrap();

    solana_sdk::hash::Hash::from_str(deser.result.value.blockhash)
        .map_err(|error| WalletError::Op(error.to_string()))
}



pub async fn get_balance(address: &str) -> WalletResult<String> {
    let balance_options = jzon::object! {
        "id":1,
        "jsonrpc":"2.0",
        "method": "getBalance",
        "params": [
            address
        ]
    }
    .to_string();

    let balance_response = FetchReq::new_for_rpc()?
        .set_body(&balance_options)
        .send()
        .await?;

    let parsed_balance =
        serde_json::from_str::<RpcResponse<ResponseWithContext<u64>>>(&balance_response)
            .map_err(|error| WalletError::Op(error.to_string()))?;

    // WARNING: Do better financial math here
    Ok((parsed_balance.result.value as f64 / LAMPORTS_PER_SOL as f64).to_string())
}

pub async fn send_sol_req(
    recipient: &str,
    lamports: u64,
    public_key_bytes: [u8; 32],
) -> WalletResult<()> {
    let cluster = CLUSTER_STORAGE.read().active_cluster().cluster();

    let pubkey = Pubkey::new_from_array(public_key_bytes);
    let recipient = Pubkey::from_str(recipient).or(Err(WalletError::Op(
        "Invalid Recipient Address".to_string(),
    )))?;

    let send_sol_instruction = transfer(&pubkey, &recipient, lamports);
    let mut tx = Transaction::new_with_payer(&[send_sol_instruction], Some(&pubkey));
    let blockhash = get_blockhash().await?;

    tx.message.recent_blockhash = blockhash;
    let tx_bytes = bincode::serialize(&tx).map_err(|error| WalletError::Op(error.to_string()))?;

    WALLET_ADAPTER
        .read()
        .sign_and_send_transaction(&tx_bytes, cluster, SendOptions::default())
        .await?;

    Ok(())
}

pub async fn request_airdrop(lamports: u64, address: &str) -> WalletResult<()> {
    let options = jzon::object! {
        "id":1,
        "jsonrpc":"2.0",
        "method": "requestAirdrop",
        "params": [
            address,
            lamports
        ]
    }
    .to_string();

    let response = FetchReq::new_for_rpc()?.set_body(&options).send().await?;

    serde_json::from_str::<RpcResponse<String>>(&response)
        .map_err(|error| WalletError::Op(error.to_string()))?;

    Ok(())
}

pub async fn accounts_runner(address: &str) -> WalletResult<AccountState> {
    *ACCOUNT_STATE.write() = AccountState::default();

    let balance = crate::get_balance(&address).await?;

    let token_accounts_options = jzon::object! {
        "id":1,
        "jsonrpc":"2.0",
        "method": "getTokenAccountsByOwner",
        "params": [
            address,
            {
                "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            },
            {
                "encoding": "jsonParsed"
            }
        ]
    }
    .to_string();

    let fetched_token_accounts = FetchReq::new_for_rpc()?
        .set_body(&token_accounts_options)
        .send()
        .await?;

    let parsed_token_accounts = serde_json::from_str::<
        RpcResponse<ResponseWithContext<Vec<TokenAccountResponse>>>,
    >(&fetched_token_accounts)
    .map_err(|error| WalletError::Op(error.to_string()))?;

    let token_accounts = parsed_token_accounts.result.value;

    let get_signatures_options = jzon::object! {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSignaturesForAddress",
        "params": [
          address
        ]
    }
    .to_string();
    let fetched_signatures = FetchReq::new("POST")?
        .add_header("Content-Type", "application/json")?
        // .add_header("Accept", "application/json")?
        .set_body(&get_signatures_options)
        .send()
        .await?;

    let parsed_signatures_response =
        serde_json::from_str::<RpcResponse<Vec<SignaturesResponse>>>(&fetched_signatures)
            .map_err(|error| WalletError::Op(error.to_string()))?;

    let signatures = parsed_signatures_response.result;

    Ok(AccountState {
        balance,
        token_accounts,
        transactions: signatures.clone(),
    })
}





