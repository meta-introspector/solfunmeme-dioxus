# `emoji_workflow_macro`

This crate provides a procedural macro (`#[emoji_workflow]`) that allows developers to define and register workflows directly within Rust code using emoji annotations.

## Purpose

It simplifies the process of integrating emoji-based workflow definitions into Rust applications, enabling compile-time generation of registration logic and enhancing code readability with visual cues.

## Core Functionalities

-   **Emoji Annotation**: Apply `#[emoji_workflow("...")]` to functions to associate them with an emoji string representing a workflow.
-   **Code Generation**: Automatically generates code to register the annotated function as a workflow with the `workflow_manager` and `emoji_lang_plugin`.
-   **Compile-time Integration**: Performs workflow setup at compile time, reducing runtime overhead.

## Usage

Add `emoji_workflow_macro` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
emoji_workflow_macro = { path = "../path/to/crates/emoji_workflow_macro" }
```

Then, use the macro to annotate your functions:

```rust
use emoji_workflow_macro::emoji_workflow;

#[emoji_workflow("📜🔗")]
fn my_document_workflow() {
    println!("Executing my document workflow!");
    // ... your workflow logic here ...
}

fn main() {
    my_document_workflow();
    // The workflow is automatically registered at compile time.
    // You can then retrieve and execute it via the global workflow manager:
    // use emoji_lang_plugin::GLOBAL_WORKFLOW_MANAGER;
    // let manager = GLOBAL_WORKFLOW_MANAGER.lock().unwrap();
    // manager.execute_workflow("my_document_workflow").unwrap();
}
```
