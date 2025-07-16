use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::fs;
use toml;
use solfunmeme_models::{LlmProvider, LlmAccount};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LlmConfig {
    pub providers: Vec<LlmProvider>,
}

pub fn load_llm_config(llm_config_path: &PathBuf) -> Result<LlmConfig> {
    let llm_config_content = fs::read_to_string(llm_config_path)
        .map_err(|e| anyhow!("Failed to read LLM config file {}: {}", llm_config_path.display(), e))?;
    let llm_config: LlmConfig = toml::from_str(&llm_config_content)
        .map_err(|e| anyhow!("Failed to parse LLM config file {}: {}", llm_config_path.display(), e))?;
    Ok(llm_config)
}

pub fn select_llm_provider<'a>(providers: &'a [LlmProvider]) -> Option<(&'a LlmProvider, &'a LlmAccount)> {
    let mut best_provider: Option<(&LlmProvider, &LlmAccount)> = None;

    for provider in providers {
        for account in &provider.accounts {
            // Only consider accounts with a command and available credits
            if account.command.is_some() && account.usage_vector.available_credits > 0.0 {
                let current_cost = account.usage_vector.cost_per_token_input + account.usage_vector.cost_per_token_output;
                match best_provider {
                    Some((_, best_account)) => {
                        let best_cost = best_account.usage_vector.cost_per_token_input + best_account.usage_vector.cost_per_token_output;
                        if current_cost < best_cost {
                            best_provider = Some((provider, account));
                        } else if current_cost == best_cost && account.usage_vector.rate_limit_per_minute > best_account.usage_vector.rate_limit_per_minute {
                            best_provider = Some((provider, account));
                        }
                    }
                    None => {
                        best_provider = Some((provider, account));
                    }
                }
            }
        }
    }
    best_provider
}
