use serde::Deserialize;
use solana_transaction_error::TransactionError;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignaturesResponse {
    pub block_time: Option<i64>,
    pub confirmation_status: Option<String>,
    pub err: Option<TransactionError>,
    pub signature: String,
}
