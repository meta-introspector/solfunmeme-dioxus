# `rhai_plugin`

This crate provides a wrapper around the `rhai` scripting engine, enabling the execution of Rhai scripts within the Solfunmeme application.

## Purpose

It allows for dynamic and flexible logic to be integrated into the project, supporting custom scripting for various tasks without requiring recompilation of the main application.

## Core Functionalities

-   **Run Script**: Execute a given Rhai script string and return the result.

## Usage (Conceptual)

```rust
use rhai_plugin::RhaiPlugin;

fn main() {
    let plugin = RhaiPlugin::new();
    let script = "let x = 10 + 20; x";
    let result = plugin.run_script(script).expect("Failed to run script");
    println!("Script result: {}", result);
}
```
