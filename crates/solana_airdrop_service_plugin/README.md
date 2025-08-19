# `solana_airdrop_service_plugin`

This crate provides a wrapper around the `solana-airdrop-service` library, offering functionalities for requesting test SOL airdrops on the Solana devnet or testnet.

## Purpose

It enables the Solfunmeme project to easily acquire test tokens for development and testing purposes, facilitating the creation and interaction with Solana programs and accounts.

## Core Functionalities

-   **Request Airdrop**: Request a specified amount of test SOL for a given Solana address.

## Usage (Conceptual)

```rust
use solana_airdrop_service_plugin::SolanaAirdropServicePlugin;

fn main() {
    let plugin = SolanaAirdropServicePlugin::new().expect("Failed to create plugin");
    let address = "SomeSolanaAddressHere"; // Replace with a valid Solana address
    let amount = 1_000_000_000; // 1 SOL
    plugin.request_airdrop(address, amount).expect("Failed to request airdrop");
    println!("Airdrop requested for {} SOL to {}. (Conceptual)", amount as f64 / 1_000_000_000.0, address);
}
```
