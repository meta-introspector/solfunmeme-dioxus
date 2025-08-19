# `steel_plugin`

This crate provides a wrapper around the `steel` Scheme-like scripting language, enabling the execution of Steel scripts within the Solfunmeme application.

## Purpose

It allows for dynamic and extensible logic to be integrated into the project, supporting custom scripting for various tasks and enabling a more flexible and programmable system.

## Core Functionalities

-   **Run Script**: Execute a given Steel script string and return the result.

## Usage (Conceptual)

```rust
use steel_plugin::SteelPlugin;

fn main() {
    let plugin = SteelPlugin::new().expect("Failed to create plugin");
    let script = "(+ 1 2)";
    let result = plugin.run_script(script).expect("Failed to run script");
    println!("Script result: {}", result);
}
```
