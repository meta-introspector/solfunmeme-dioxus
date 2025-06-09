use serde::Deserialize;
use crate::model::TokenData;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub data: TokenData,
}