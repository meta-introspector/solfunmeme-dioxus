use anyhow::{Result, anyhow};
use std::io::{self, Read, Write};

// Import necessary types from the solfunmeme_clifford crate
use solfunmeme_clifford::{SolMultivector, SolCl};
use solfunmeme_models::{CliffordOperationRequest, CliffordOperationResponse, LlmTaskPayload};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let payload: LlmTaskPayload = serde_json::from_str(&buffer)?;

    let response = match payload {
        LlmTaskPayload::CliffordOperation(request) => {
            match request.operation.as_str() {
                "create_scalar_multivector" => {
                    let mv = SolMultivector::from_scalar(request.scalar_value);
                    CliffordOperationResponse { result: format!("{:?}", mv), error: None }
                },
                "create_vector_multivector" => {
                    if request.vector_values.len() > 0 {
                        let mv = SolMultivector::from_vector(request.vector_values.iter().cloned());
                        CliffordOperationResponse { result: format!("{:?}", mv), error: None }
                    } else {
                        CliffordOperationResponse { result: String::new(), error: Some("Vector values not provided for vector multivector creation.".to_string()) }
                    }
                },
                "reverse_multivector" => {
                    // This operation would require a multivector as input, which is not yet handled
                    CliffordOperationResponse { result: String::new(), error: Some("Reverse operation not fully implemented for direct input.".to_string()) }
                },
                "update_flow_multivector" => {
                    if let Some(input_mv) = request.input_multivector {
                        let new_mv = if request.vector_values.len() > 0 {
                            SolMultivector::from_vector(request.vector_values.iter().cloned())
                        } else {
                            SolMultivector::from_scalar(request.scalar_value)
                        };
                        let updated_mv = input_mv + new_mv;
                        CliffordOperationResponse { result: format!("{:?}", updated_mv), error: None }
                    } else {
                        CliffordOperationResponse { result: String::new(), error: Some("Input multivector not provided for update_flow_multivector.".to_string()) }
                    }
                },
                "compose_from_vector_primes" => {
                    if request.vector_values.len() > 0 {
                        let primes: Vec<u32> = request.vector_values.iter().map(|&f| f as u32).collect();
                        match solfunmeme_clifford::compose_prime_multivectors(&primes) {
                            Ok(mv) => CliffordOperationResponse { result: format!("{:?}", mv), error: None },
                            Err(e) => CliffordOperationResponse { result: String::new(), error: Some(format!("Failed to compose prime multivectors: {}", e)) },
                        }
                    } else {
                        CliffordOperationResponse { result: String::new(), error: Some("Vector values (primes) not provided for composition.".to_string()) }
                    }
                },
                "prime_identity_multivector" => {
                    if request.scalar_value > 0.0 {
                        match solfunmeme_clifford::prime_to_multivector(request.scalar_value as u32) {
                            Ok(mv) => CliffordOperationResponse { result: format!("{:?}", mv), error: None },
                            Err(e) => CliffordOperationResponse { result: String::new(), error: Some(format!("Failed to get prime identity multivector: {}", e)) },
                        }
                    } else {
                        CliffordOperationResponse { result: String::new(), error: Some("Scalar value (prime) not provided for prime identity.".to_string()) }
                    }
                }
                _ => CliffordOperationResponse { result: String::new(), error: Some(format!("Unknown operation: {}", request.operation)) },
            }
        },
        _ => CliffordOperationResponse { result: String::new(), error: Some("Unsupported task payload type.".to_string()) },
    };

    let response_json = serde_json::to_string_pretty(&response)?;
    io::stdout().write_all(response_json.as_bytes())?;

    Ok(())
}