# `rust_sbert_plugin`

This crate provides a wrapper around the `rust-sbert` library, offering functionalities for generating sentence embeddings using Sentence-BERT models.

## Purpose

It enables the Solfunmeme project to convert natural language text into high-dimensional numerical vectors, which can be used for tasks such as semantic search, text similarity, and clustering.

## Core Functionalities

-   **Embed Text**: Takes a text string as input and returns its corresponding sentence embedding (a vector of floating-point numbers).

## Usage (Conceptual)

```rust
use rust_sbert_plugin::RustSbertPlugin;

fn main() {
    let model_path = "./models/sbert-base-nli-mean-tokens"; // Placeholder path
    let plugin = RustSbertPlugin::new(model_path).expect("Failed to create plugin");
    let text = "Hello, world! This is a test sentence.";
    let embedding = plugin.embed_text(text).expect("Failed to embed text");
    println!("Text embedding: {:?}", embedding);
}
```
