# `rrust_kontekst`

This crate provides a procedural macro (`#[mcp_component]`) for defining and registering Micro-Component Protocol (MCP) components in Rust.

## Purpose

Inspired by the Introspector/ZOS/Solfunmeme project, this macro simplifies the process of creating introspectable and discoverable components, enabling dynamic interaction and management within a larger system.

## Core Functionalities

-   **Component Annotation**: Apply `#[mcp_component(...)]` to functions to mark them as MCP components.
-   **Automatic Registration**: Generates code to automatically register the component with a global MCP registry at compile time.
-   **Metadata Generation**: Extracts and exposes metadata about the component (e.g., name, description, parameters) for introspection.
-   **Handler Function Generation**: Creates a standardized handler function for executing the component.

## Usage

Add `rrust_kontekst` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rrust_kontekst = { path = "../path/to/crates/rrust_kontekst" }
```

Then, use the macro to annotate your functions:

```rust
use rrust_kontekst::mcp_component;

#[mcp_component(
    menu = "my_menu",
    label = "My Component",
    emoji = "✨",
    description = "A sample MCP component."
)]
pub async fn my_component() -> Result<String, Box<dyn std::error::Error>> {
    Ok("Hello from my component!".to_string())
}

fn main() {
    // Components are registered automatically at startup.
    // You can then interact with them via the MCP registry.
}
```
