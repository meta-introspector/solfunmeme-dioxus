use crate::model::{SignaturesResponse, TokenAccountResponse};


#[derive(Debug, Default, PartialEq)]
pub struct AccountState {
    pub balance: String,
    pub token_accounts: Vec<TokenAccountResponse>,
    pub transactions: Vec<SignaturesResponse>,
}

impl AccountState {
    pub fn token_accounts_is_empty(&self) -> bool {
        self.token_accounts.is_empty()
    }

    pub fn transactions_is_empty(&self) -> bool {
        self.token_accounts.is_empty()
    }

    pub fn token_accounts(&self) -> &[TokenAccountResponse] {
        self.token_accounts.as_slice()
    }

    pub fn transactions(&self) -> &[SignaturesResponse] {
        self.transactions.as_slice()
    }
}