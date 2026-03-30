# `solfunmeme_models`

This crate defines the core data models and structures used throughout the Solfunmeme application, including blockchain models, account management, and mathematical representations.

## Purpose

It provides a centralized repository for all data models, ensuring consistency and type safety across the application. This includes Solana blockchain models, account states, cryptographic structures, and mathematical representations like Clifford algebra.

## Core Data Models

### Blockchain & Account Models
- **Account Management**: Account states, token accounts, signatures, and blockchain responses
- **Connection Management**: Cluster management, network states, and connection handling
- **Crypto Operations**: Cryptographic utilities and wallet error handling

### Mathematical Models
- **Clifford Algebra**: Mathematical structures for geometric algebra operations
- **Lean Integration**: Lean theorem prover integration models and types
- **Meme Structures**: Core memetic units and metameme representations

### Utility Models
- **Storage**: Storage entry models and management
- **Parsing**: Parsed data structures and parse information
- **Responses**: RPC responses and context handling

## Usage (Conceptual)

```rust
use solfunmeme_models::{
    account::Account,
    crypto::CryptoKey,
    clifford::CliffordVector,
    lean::LeanExpression,
};

fn main() {
    // Example: Create an account
    // let account = Account::new("user123");
    
    // Example: Use Clifford algebra
    // let vector = CliffordVector::new([1.0, 2.0, 3.0]);
    
    // Example: Parse Lean expression
    // let expr = LeanExpression::parse("∀ x, x = x");
}
``` 