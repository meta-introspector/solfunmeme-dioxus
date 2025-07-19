use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// Adds two u64 numbers. This is a placeholder function.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Represents a MemeToken, a content-addressable object within the SOLFUNMEME system.
/// Its `prime_id` encodes its semantic composition using prime numbers as multivector components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeToken {
    /// The unique prime-encoded identifier of the meme token.
    /// This is the product of prime numbers representing its semantic attributes.
    pub prime_id: u64,
    /// The narrative or textual content associated with the meme token.
    pub narrative: String,
    /// A score indicating the virality or propagation potential of the meme.
    pub virality_score: f64,
    /// The current consensus state of the meme token within the system.
    pub consensus_state: ConsensusState,
}

/// Represents the consensus state of a MemeToken.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusState {
    /// The meme token has been proposed and is awaiting consensus.
    Proposed,
    /// The meme token is currently undergoing a voting process.
    Voting,
    /// The meme token has been accepted by the consensus mechanism.
    Accepted,
    /// The meme token has been rejected by the consensus mechanism.
    Rejected,
}

/// Defines a rule for how a MemeToken can evolve based on its prime composition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionRule {
    /// A list of prime numbers that, when present in a MemeToken's `prime_id`,
    /// trigger this evolution rule.
    pub trigger_primes: Vec<u64>,
    /// The action to be performed when this rule is triggered.
    pub action: EvolutionAction,
}

/// Defines the actions that can be taken as part of a MemeToken's evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionAction {
    /// Adds a new method with the given name to the code representation of the MemeToken.
    AddMethod(String),
    /// Modifies a specific field in the code representation of the MemeToken.
    ModifyField(String, String),
    /// Adds a new trait implementation to the code representation of the MemeToken.
    AddTrait(String),
    /// Multiplies the MemeToken's `prime_id` by a new prime, evolving its semantic composition.
    MultiplyPrime(u64),
}



/// Performs prime factorization on a given u64 number.
/// Returns a vector of prime factors, including duplicates if the number is divisible by a prime multiple times.
pub fn get_prime_factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut d = 2;
    while d * d <= num {
        while num % d == 0 {
            factors.push(d);
            num /= d;
        }
        d += 1;
    }
    if num > 1 {
        factors.push(num);
    }
    factors
}

impl MemeToken {
    /// Creates a new `MemeToken` from a list of prime numbers and a narrative.
    /// The `prime_id` is calculated as the product of the given primes.
    pub fn new(primes: &[u64], narrative: String) -> Self {
        let prime_id = primes.iter().product();
        Self {
            prime_id,
            narrative,
            virality_score: 0.0,
            consensus_state: ConsensusState::Proposed,
        }
    }

    /// Evolves the `MemeToken` by multiplying its `prime_id` with a new prime.
    /// This effectively adds a new semantic attribute to the meme.
    /// It avoids adding duplicate primes to prevent redundant attributes.
    pub fn evolve_with_prime(&mut self, new_prime: u64) {
        // Avoid duplicate primes
        if self.prime_id % new_prime != 0 {
            self.prime_id *= new_prime;
        }
    }

    /// Returns a vector of static string slices representing the semantic composition of the `MemeToken`.
    /// This is derived by prime-factorizing the `prime_id` and mapping each prime factor to its
    /// corresponding emoji/semantic description using the `EMOJI_PRIME_MAPPING`.
    pub fn get_semantic_composition(&self, emoji_prime_mapping: &HashMap<u64, &'static str>) -> Vec<&'static str> {
        let factors = get_prime_factors(self.prime_id);
        let mut composition = Vec::new();
        for factor in factors {
            if let Some(&emoji) = emoji_prime_mapping.get(&factor) {
                composition.push(emoji);
            }
        }
        composition.sort_unstable(); // Ensure consistent order
        composition.dedup(); // Remove duplicate emojis if a prime appears multiple times
        composition
    }
}
