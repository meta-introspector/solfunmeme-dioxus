# `rust_sentence_transformers_plugin`

This crate provides a wrapper around the `rust-sentence-transformers` library, offering functionalities for encoding sentences into vector embeddings.

## Purpose

It enables the Solfunmeme project to convert natural language sentences into numerical representations, which are crucial for various NLP tasks such as semantic search, text classification, and clustering.

## Core Functionalities

-   **Encode Sentences**: Takes a list of sentences and returns their corresponding vector embeddings.

## Usage (Conceptual)

```rust
use rust_sentence_transformers_plugin::RustSentenceTransformersPlugin;

#[tokio::main]
async fn main() {
    let model_name = "all-MiniLM-L6-v2"; // Placeholder model name
    let plugin = RustSentenceTransformersPlugin::new(model_name).expect("Failed to create plugin");
    let sentences = vec![
        "Hello, world!".to_string(),
        "This is a test sentence.".to_string(),
    ];
    let embeddings = plugin.encode_sentences(sentences).expect("Failed to encode sentences");
    println!("Sentence embeddings: {:?}", embeddings);
}
```
