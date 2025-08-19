# `emoji_lang_plugin`

This crate implements an emoji-based language for defining and managing workflows within the Solfunmeme ecosystem.

## Purpose

It provides a unique and visually intuitive way to compose complex processing pipelines using sequences of emojis, leveraging semantic mappings from an ontology.

## Core Functionalities

-   **Emoji String Parsing**: Interprets emoji sequences into meaningful workflow steps.
-   **URL Encoding/Decoding**: Handles the conversion of emoji strings for safe inclusion in URLs and vice-versa.
-   **Ontology Integration**: Utilizes `solfunmem.jsonld` to provide semantic context for emojis.

## Usage (Conceptual)

```rust
use emoji_lang_plugin::EmojiWorkflow;

fn main() {
    let emoji_str = "🚀📜🔍";
    let workflow = EmojiWorkflow::new("my_workflow".to_string(), emoji_str.to_string());
    let parsed_steps = workflow.parse_emoji_string();
    println!("Parsed emoji workflow steps: {:?}", parsed_steps);

    let url = workflow.to_url();
    println!("Workflow URL: {}", url);

    let loaded_workflow = EmojiWorkflow::from_url(&url).expect("Failed to load from URL");
    println!("Loaded workflow emoji string: {}", loaded_workflow.emoji_string);
}
```
