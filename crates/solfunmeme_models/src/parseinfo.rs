use crate::tokenamount::TokenAmount;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseInfo {
    pub mint: String,
    pub state: String,
    pub token_amount: TokenAmount,
} 