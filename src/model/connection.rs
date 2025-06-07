use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use dioxus::prelude::*;

use wallet_adapter::Cluster;

use crate::model::storage_entry::StorageEntry;
use crate::model::adaptercluster::AdapterCluster;
// storage_entry
#[derive(Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Connection {
    pub name: String,
    pub url: String,
    pub cluster_name: String, // Reference to cluster by name
}

impl Connection {
    // Mask sensitive parts of the URL for display
    pub fn masked_url(&self) -> String {
        if let Some(token_start) = self.url.find("token=") {
            let (before_token, after_token) = self.url.split_at(token_start + 6); // "token=".len() = 6
            if let Some(token_end) = after_token.find('&') {
                let (token_part, rest) = after_token.split_at(token_end);
                let masked_token = if token_part.len() > 8 {
                    format!(
                        "{}...{}",
                        &token_part[..4],
                        &token_part[token_part.len() - 4..]
                    )
                } else {
                    "*".repeat(token_part.len())
                };
                format!("{}{}{}", before_token, masked_token, rest)
            } else {
                // Token is at the end of URL
                let masked_token = if after_token.len() > 8 {
                    format!(
                        "{}...{}",
                        &after_token[..4],
                        &after_token[after_token.len() - 4..]
                    )
                } else {
                    "*".repeat(after_token.len())
                };
                format!("{}{}", before_token, masked_token)
            }
        } else if self.url.contains("access_token=") {
            // Handle access_token parameter
            if let Some(token_start) = self.url.find("access_token=") {
                let (before_token, after_token) = self.url.split_at(token_start + 13); // "access_token=".len() = 13
                if let Some(token_end) = after_token.find('&') {
                    let (token_part, rest) = after_token.split_at(token_end);
                    let masked_token = if token_part.len() > 8 {
                        format!(
                            "{}...{}",
                            &token_part[..4],
                            &token_part[token_part.len() - 4..]
                        )
                    } else {
                        "*".repeat(token_part.len())
                    };
                    format!("{}{}{}", before_token, masked_token, rest)
                } else {
                    let masked_token = if after_token.len() > 8 {
                        format!(
                            "{}...{}",
                            &after_token[..4],
                            &after_token[after_token.len() - 4..]
                        )
                    } else {
                        "*".repeat(after_token.len())
                    };
                    format!("{}{}", before_token, masked_token)
                }
            } else {
                self.url.clone()
            }
        } else if self.url.contains("://") && self.url.matches(':').count() >= 2 {
            // Handle URLs with embedded credentials (user:pass@host)
            if let Some(at_pos) = self.url.find('@') {
                if let Some(scheme_end) = self.url.find("://") {
                    let scheme_part = &self.url[..scheme_end + 3];
                    let after_at = &self.url[at_pos..];
                    format!("{}***{}", scheme_part, after_at)
                } else {
                    self.url.clone()
                }
            } else {
                self.url.clone()
            }
        } else {
            self.url.clone()
        }
    }
}

