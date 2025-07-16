# `vibrato_plugin`

This crate provides a wrapper around the `vibrato` library, offering functionalities for fast and accurate Japanese tokenization.

## Purpose

It enables the Solfunmeme project to efficiently process Japanese text by segmenting it into words or sub-word units, which is essential for various natural language processing tasks.

## Core Functionalities

-   **Tokenize**: Takes a Japanese text string and returns a vector of tokens.

## Usage (Conceptual)

```rust
use vibrato_plugin::VibratoPlugin;

fn main() {
    let plugin = VibratoPlugin::new().expect("Failed to create plugin");
    let text = "これはテストです";
    let tokens = plugin.tokenize(text);
    println!("Tokens: {:?}", tokens);
}
```
