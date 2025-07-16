# `views_lib`

This crate contains the reusable UI components and views for the Solfunmeme Dioxus application.

## Purpose

It provides a modular and organized structure for the application's user interface, promoting reusability, consistency, and maintainability across different screens and features.

## Core Functionalities

-   **Reusable Components**: Defines common UI elements such as buttons, input fields, and navigation bars.
-   **Page Views**: Implements the visual layout and interactive elements for different application screens.
-   **Styling Integration**: Manages the application's visual styles and themes.

## Usage (Conceptual)

```rust
// use views_lib::components::Button;
// use dioxus::prelude::*;

// fn MyPage(cx: Scope) -> Element {
//     render!(
//         Button { onclick: move |_| println!("Button clicked!"), label: "Click Me" }
//     )
// }
```
