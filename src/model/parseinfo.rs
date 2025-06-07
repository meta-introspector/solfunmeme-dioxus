use serde::Deserialize;
use crate::model::TokenAmount;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseInfo {
    pub mint: String,
    pub state: String,
    pub token_amount: TokenAmount,
}