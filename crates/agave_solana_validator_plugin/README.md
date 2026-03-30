# `agave_solana_validator_plugin`

This crate provides a wrapper around the `agave-solana-validator` submodule, offering functionalities to interact with a Solana validator.

## Purpose

The primary purpose of this plugin is to enable the starting and management of a local Solana validator instance, facilitating testing and development within the Solfunmeme ecosystem.

## Core Functionalities

-   **Start Validator**: Initialize and run a Solana validator.

## Usage (Conceptual)

```rust
use agave_solana_validator_plugin::AgaveSolanaValidatorPlugin;

fn main() {
    let plugin = AgaveSolanaValidatorPlugin::new().expect("Failed to create plugin");
    plugin.start_validator().expect("Failed to start validator");
    println!("Solana validator started (conceptual).");
}
```
