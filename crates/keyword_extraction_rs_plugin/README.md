# `keyword_extraction_rs_plugin`

This crate provides a wrapper around the `keyword-extraction-rs` library, offering functionalities for extracting keywords from text.

## Purpose

It enables the Solfunmeme project to identify important terms and phrases within textual content, which can be used for indexing, summarization, or semantic analysis.

## Core Functionalities

-   **Extract Keywords**: Takes a text string as input and returns a list of extracted keywords.

## Usage (Conceptual)

```rust
use keyword_extraction_rs_plugin::KeywordExtractionRsPlugin;

fn main() {
    let plugin = KeywordExtractionRsPlugin::new().expect("Failed to create plugin");
    let text = "Rust is a programming language that is fast, safe, and concurrent.";
    let keywords = plugin.extract_keywords(text).expect("Failed to extract keywords");
    println!("Extracted keywords: {:?}", keywords);
}
```
