# `tongrams_rs_plugin`

This crate provides a wrapper around the `tongrams-rs` library, offering functionalities for querying n-gram language models.

## Purpose

It enables the Solfunmeme project to analyze and leverage statistical properties of text by providing efficient access to n-gram frequencies, which can be used for tasks such as text generation, language modeling, and spell checking.

## Core Functionalities

-   **Query N-grams**: Query the n-gram model for frequencies or probabilities of specific n-gram sequences.

## Usage (Conceptual)

```rust
use tongrams_rs_plugin::TongramsRsPlugin;

fn main() {
    let model_path = "./path/to/your/model.bin"; // Placeholder path
    let plugin = TongramsRsPlugin::new(model_path).expect("Failed to create plugin");
    let text = "hello world";
    let ngrams = plugin.query_ngrams(text);
    println!("Queried n-grams: {:?}", ngrams);
}
```
