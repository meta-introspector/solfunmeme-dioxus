# `vaporetto_plugin`

This crate provides a wrapper around the `vaporetto` library, offering functionalities for Japanese morphological analysis and tokenization.

## Purpose

It enables the Solfunmeme project to process Japanese text by breaking it down into meaningful units (tokens), which is essential for natural language understanding, search, and other text-based analyses.

## Core Functionalities

-   **Tokenize**: Takes a Japanese text string and returns a vector of tokens, representing words or sub-word units.

## Usage (Conceptual)

```rust
use vaporetto_plugin::VaporettoPlugin;

fn main() {
    let model_data = include_bytes!("path/to/your/model.bin"); // Placeholder for model data
    let plugin = VaporettoPlugin::new(model_data).expect("Failed to create plugin");
    let text = "すもももももももものうち";
    let tokens = plugin.tokenize(text);
    println!("Tokens: {:?}", tokens);
}
```
