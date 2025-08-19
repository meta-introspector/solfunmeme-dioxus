# `vtext_plugin`

This crate provides a wrapper around the `vtext` library, offering functionalities for natural language processing tasks such as tokenization and text vectorization.

## Purpose

It enables the Solfunmeme project to process and analyze textual data by converting raw text into numerical representations (vectors), which are essential for machine learning models and semantic analysis.

## Core Functionalities

-   **Tokenize**: Breaks down text into individual words or sub-word units.
-   **Vectorize**: Converts a collection of text documents into a matrix of numerical features (e.g., TF-IDF vectors).

## Usage (Conceptual)

```rust
use vtext_plugin::VtextPlugin;

fn main() {
    let mut plugin = VtextPlugin::new().expect("Failed to create plugin");
    let text = "The quick brown fox jumps over the lazy dog.";
    let tokens = plugin.tokenize(text);
    println!("Tokens: {:?}", tokens);

    let texts = vec!["hello world", "foo bar"];
    let vectors = plugin.vectorize(&texts).expect("Failed to vectorize");
    println!("Vectors: {:?}", vectors);
}
```
