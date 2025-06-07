use serde::Deserialize;

use crate::model::Account;

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccountResponse {
    pub pubkey: String,
    pub account: Account,
}

impl TokenAccountResponse {
    pub fn mint(&self) -> String {
        self.account.data.parsed.info.mint.to_owned()
    }

    pub fn ata_address(&self) -> String {
        self.pubkey.to_owned()
    }

    pub fn balance(&self) -> String {
        self.account
            .data
            .parsed
            .info
            .token_amount
            .ui_amount_string
            .to_owned()
    }

    pub fn state(&self) -> String {
        self.account.data.parsed.info.state.to_uppercase()
    }
}