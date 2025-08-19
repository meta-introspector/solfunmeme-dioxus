use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct VibeProof {
    pub ontology: Vec<u8>,
    pub harmonic_sum: u8,
    pub commitment_hash: String,
    pub witness_hash: String,
    pub proof_data: ProofData,
    pub metadata: ProofMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofData {
    pub statement: String,
    pub challenge: u8,
    pub response: u8,
    pub verification_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofMetadata {
    pub timestamp: u64,
    pub chat_documents: usize,
    pub easter_eggs_found: usize,
    pub vibe_score: f64,
    pub theorem_count: usize,
    pub ontology_mentions: Vec<String>,
}

pub struct VibeProver {
    ontology: Vec<u8>,
    chat_data: HashMap<String, String>,
}

impl VibeProver {
    pub fn new(ontology_str: String) -> Self {
        // Parse ontology string like "0,1,2,3,5,7"
        let ontology: Vec<u8> = ontology_str
            .split(',')
            .filter_map(|s| s.trim().parse::<u8>().ok())
            .collect();
        
        Self {
            ontology,
            chat_data: HashMap::new(),
        }
    }

    pub fn generate_proof(&mut self, output_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        println!("Generating vibe proof with ontology: {:?}", self.ontology);
        
        // Load chat data for context
        self.load_chat_data()?;
        
        // Generate the proof components
        let harmonic_sum = self.compute_harmonic_sum();
        let commitment_hash = self.generate_commitment();
        let witness_hash = self.generate_witness_hash();
        let proof_data = self.generate_proof_data();
        let metadata = self.generate_metadata();
        
        let proof = VibeProof {
            ontology: self.ontology.clone(),
            harmonic_sum,
            commitment_hash,
            witness_hash,
            proof_data,
            metadata,
        };
        
        // Save the proof
        let proof_json = serde_json::to_string_pretty(&proof)?;
        fs::write(output_path, proof_json)?;
        
        println!("Vibe proof generated successfully!");
        println!("Harmonic sum: {}", harmonic_sum);
        println!("Commitment: {}", commitment_hash);
        println!("Witness: {}", witness_hash);
        
        Ok(())
    }

    fn load_chat_data(&mut self) -> Result<(), Box<dyn Error>> {
        // Load sample chat data for context
        // In a real implementation, this would load from the chat index
        self.chat_data.insert(
            "theorem_1".to_string(),
            "Quasi-meta tarot deck of meme-NFTs expanding combinatorially".to_string()
        );
        self.chat_data.insert(
            "theorem_2".to_string(),
            "Reflection over diagonalization produces market quote Q".to_string()
        );
        self.chat_data.insert(
            "theorem_3".to_string(),
            "Compressed NFT containing all cards in tradable manner".to_string()
        );
        self.chat_data.insert(
            "theorem_5".to_string(),
            "Access to daily rollup requires owning our coin".to_string()
        );
        self.chat_data.insert(
            "ontology".to_string(),
            "Zero knowledge ontology [0,1,2,3,5,7] with harmonic properties".to_string()
        );
        
        Ok(())
    }

    fn compute_harmonic_sum(&self) -> u8 {
        // Compute harmonic sum using the ontology
        // This simulates the sum of edge weights in the bipartite graph
        let mut sum = 0u8;
        
        for &value in &self.ontology {
            // Use modular arithmetic to keep within ontology bounds
            sum = (sum + value) % 7;
        }
        
        // Ensure result is in ontology
        if !self.ontology.contains(&sum) {
            sum = 5; // Default to 5 if not in ontology
        }
        
        sum
    }

    fn generate_commitment(&self) -> String {
        // Generate a commitment hash using the ontology and chat data
        let mut hasher = Sha256::new();
        
        // Hash the ontology
        hasher.update(&self.ontology);
        
        // Hash chat data context
        for (key, value) in &self.chat_data {
            hasher.update(key.as_bytes());
            hasher.update(value.as_bytes());
        }
        
        // Add harmonic sum
        let harmonic_sum = self.compute_harmonic_sum();
        hasher.update(&[harmonic_sum]);
        
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn generate_witness_hash(&self) -> String {
        // Generate a witness hash (private data that proves knowledge)
        let mut hasher = Sha256::new();
        
        // Hash the private witness data
        let witness_data = format!(
            "ontology:{:?},harmonic_sum:{},chat_count:{}",
            self.ontology,
            self.compute_harmonic_sum(),
            self.chat_data.len()
        );
        
        hasher.update(witness_data.as_bytes());
        
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn generate_proof_data(&self) -> ProofData {
        // Generate zero-knowledge proof data
        let statement = format!(
            "I know a valid assignment of weights from ontology {:?} such that the harmonic sum is {}",
            self.ontology,
            self.compute_harmonic_sum()
        );
        
        // Generate a challenge (in a real ZKP, this would come from the verifier)
        let challenge = self.ontology[2]; // Use 3rd element as challenge
        
        // Generate response (in a real ZKP, this would be computed using the witness)
        let response = (self.compute_harmonic_sum() + challenge) % 7;
        
        // Generate verification key (simplified)
        let verification_key = format!(
            "vk_{:?}_{}",
            self.ontology,
            self.compute_harmonic_sum()
        );
        
        ProofData {
            statement,
            challenge,
            response,
            verification_key,
        }
    }

    fn generate_metadata(&self) -> ProofMetadata {
        // Generate metadata about the proof
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let easter_eggs_found = self.chat_data.len();
        let vibe_score = 85.0; // High vibe score for the system
        let theorem_count = self.chat_data.keys()
            .filter(|k| k.starts_with("theorem"))
            .count();
        
        let ontology_mentions = vec![
            "zero knowledge".to_string(),
            "harmonic properties".to_string(),
            "bipartite graph".to_string(),
            "mycelium web".to_string(),
            "seashell currency".to_string(),
            "SOLFUNMEME".to_string(),
        ];
        
        ProofMetadata {
            timestamp,
            chat_documents: self.chat_data.len(),
            easter_eggs_found,
            vibe_score,
            theorem_count,
            ontology_mentions,
        }
    }

    pub fn verify_proof(&self, proof: &VibeProof) -> bool {
        // Verify the zero-knowledge proof
        println!("Verifying vibe proof...");
        
        // Check ontology consistency
        if proof.ontology != self.ontology {
            println!("❌ Ontology mismatch");
            return false;
        }
        
        // Check harmonic sum
        let expected_sum = self.compute_harmonic_sum();
        if proof.harmonic_sum != expected_sum {
            println!("❌ Harmonic sum mismatch: expected {}, got {}", expected_sum, proof.harmonic_sum);
            return false;
        }
        
        // Check commitment hash
        let expected_commitment = self.generate_commitment();
        if proof.commitment_hash != expected_commitment {
            println!("❌ Commitment hash mismatch");
            return false;
        }
        
        // Check proof data consistency
        let expected_response = (proof.harmonic_sum + proof.proof_data.challenge) % 7;
        if proof.proof_data.response != expected_response {
            println!("❌ Response mismatch: expected {}, got {}", expected_response, proof.proof_data.response);
            return false;
        }
        
        println!("✅ Vibe proof verified successfully!");
        println!("Statement: {}", proof.proof_data.statement);
        println!("Harmonic sum: {}", proof.harmonic_sum);
        println!("Vibe score: {:.1}", proof.metadata.vibe_score);
        println!("Theorems found: {}", proof.metadata.theorem_count);
        
        true
    }

    pub fn generate_twitter_post(&self, proof: &VibeProof) -> String {
        // Generate a Twitter/X post for the daily ZKP
        format!(
            "🧠 Daily ZKP: Vibe proof generated!\n\n\
            🔢 Ontology: {:?}\n\
            🎵 Harmonic sum: {}\n\
            🔐 Commitment: {}...\n\
            📊 Vibe score: {:.1}\n\
            📚 Theorems: {}\n\n\
            Verify: [blockchain link]\n\
            #SOLFUNMEME #ZeroKnowledge #VibeProof #MyceliumWeb",
            proof.ontology,
            proof.harmonic_sum,
            &proof.commitment_hash[..8],
            proof.metadata.vibe_score,
            proof.metadata.theorem_count
        )
    }

    pub fn analyze_ontology_properties(&self) -> HashMap<String, String> {
        let mut properties = HashMap::new();
        
        // Analyze the ontology [0,1,2,3,5,7]
        properties.insert("size".to_string(), self.ontology.len().to_string());
        properties.insert("primes".to_string(), "2,3,5,7".to_string());
        properties.insert("non_primes".to_string(), "0,1".to_string());
        properties.insert("max_value".to_string(), self.ontology.iter().max().unwrap().to_string());
        properties.insert("min_value".to_string(), self.ontology.iter().min().unwrap().to_string());
        
        // Harmonic properties
        let harmonic_sum = self.compute_harmonic_sum();
        properties.insert("harmonic_sum".to_string(), harmonic_sum.to_string());
        
        // Check if it's a valid ontology
        let is_valid = self.ontology.len() == 6 && 
                      self.ontology.contains(&0) && 
                      self.ontology.contains(&1) &&
                      self.ontology.contains(&2) &&
                      self.ontology.contains(&3) &&
                      self.ontology.contains(&5) &&
                      self.ontology.contains(&7);
        
        properties.insert("is_valid".to_string(), is_valid.to_string());
        
        properties
    }
} 