use serde::{Deserialize, Serialize};

/// Represents the usage characteristics and cost metrics of an LLM account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageVector {
    /// The cost per input token for this LLM account.
    pub cost_per_token_input: f64,
    /// The cost per output token for this LLM account.
    pub cost_per_token_output: f64,
    /// The rate limit (requests per minute) for this LLM account.
    pub rate_limit_per_minute: u32,
    /// The available credits or budget for this LLM account.
    pub available_credits: f64,
}

/// Represents a single LLM account with its credentials and usage characteristics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmAccount {
    /// A unique identifier for the LLM account.
    pub id: String,
    /// The API key or token for authenticating with the LLM provider (optional).
    pub api_key: Option<String>,
    /// The usage characteristics and cost metrics for this account.
    pub usage_vector: UsageVector,
    /// The command to execute the external LLM provider binary (optional).
    pub command: Option<String>,
}

/// Represents an LLM provider, which can have multiple associated accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProvider {
    /// The name of the LLM provider (e.g., "Gemini", "Groq", "OpenAI").
    pub name: String,
    /// A list of accounts associated with this LLM provider.
    pub accounts: Vec<LlmAccount>,
}
