# `solfunmeme_views`

This crate contains all the UI components and views for the Solfunmeme Dioxus application, providing a comprehensive set of reusable components for building the user interface.

## Purpose

It serves as the presentation layer of the application, containing all the visual components, pages, and user interaction logic. The views are built using Dioxus and provide a modern, responsive web interface for the Solfunmeme ecosystem.

## Core Components

### Main Application Views
- **Dashboard**: Main application dashboard with overview and navigation
- **Accounts**: Account management and display components
- **Connections**: Blockchain connection management and status
- **Clusters**: Cluster management and visualization
- **Coins**: Cryptocurrency display and management

### Specialized Views
- **Crypto Frontend**: Cryptocurrency-specific UI components and forms
- **Meme Management**: Meme creation, editing, and management interfaces
- **Workflow Views**: Workflow visualization and management components
- **Wikidata Integration**: Wikidata meme display and interaction
- **Source Browser**: Code source browsing and analysis interface

### Utility Views
- **Encryption**: Encryption/decryption interface components
- **Expression Parsing**: Mathematical expression parsing and display
- **Notification System**: User notification and alert components
- **Styling**: Theme and styling management components

## Usage (Conceptual)

```rust
use dioxus::prelude::*;
use solfunmeme_views::{
    dashboard::Dashboard,
    accounts::Accounts,
    connections::Connections,
    crypto_frontend::CryptoApp,
};

fn main() {
    launch(App);
}

fn App(cx: Scope) -> Element {
    render!(
        Router::<Route> {}
    )
}
``` 