use crate::model::TokenData;
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub data: TokenData,
}
