# `core_data_lib`

This crate defines the fundamental data structures and types used across the Solfunmeme Dioxus project.

## Purpose

It serves as a central repository for shared data models, ensuring consistency and type safety throughout the application.

## Core Data Structures

-   **`Meme`**: Represents a core memetic unit with properties like ID, name, emoji representation, and introspection role.
-   **`ZOS`**: Defines a sequence of memes, representing a structured memetic flow.
-   **`ReifiedState`**: Captures the state of the system, including a URL and an emoji string representing its current configuration.

## Usage (Conceptual)

```rust
// Example of using core data types
// use core_data_lib::{Meme, ZOS, ReifiedState};

// fn main() {
//     let my_meme = Meme { /* ... */ };
//     let my_zos = ZOS { /* ... */ };
//     let my_state = ReifiedState { /* ... */ };
// }
```
