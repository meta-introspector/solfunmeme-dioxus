# `layered_nlp_plugin`

This crate provides a wrapper around the `layered-nlp` library, offering functionalities for processing text through multiple NLP layers.

## Purpose

It enables the Solfunmeme project to perform complex natural language processing tasks by applying a sequence of analysis steps, such as tokenization, part-of-speech tagging, and named entity recognition.

## Core Functionalities

-   **Process Text**: Takes a text string and processes it through the configured NLP layers, returning the annotated or transformed text.

## Usage (Conceptual)

```rust
use layered_nlp_plugin::LayeredNlpPlugin;

fn main() {
    let plugin = LayeredNlpPlugin::new().expect("Failed to create plugin");
    let text = "The quick brown fox jumps over the lazy dog.";
    let processed_text = plugin.process_text(text).expect("Failed to process text");
    println!("Processed text: {}", processed_text);
}
```
