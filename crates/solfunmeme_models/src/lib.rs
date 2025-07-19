

pub mod wallet_manager;
pub use wallet_manager::{WalletCredentials, SecretStore, EncryptedSecret, WalletManager};

pub mod llm_config_models;
pub mod clifford_ops_models;
pub mod llm_task_definitions;
pub mod meme_definitions;
pub mod serializable_multivector;

pub use llm_config_models::{UsageVector, LlmAccount, LlmProvider};
pub use clifford_ops_models::{CliffordOperationRequest, CliffordOperationResponse};
pub use llm_task_definitions::{CodeReflectionTask, LlmTaskPayload, LlmTaskGroup};
pub use meme_definitions::{MemeToken, ConsensusState, EvolutionRule, EvolutionAction, get_prime_factors};
pub use serializable_multivector::SerializableMultivector;

#[cfg(test)]
mod tests {
    use super::meme_definitions::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
