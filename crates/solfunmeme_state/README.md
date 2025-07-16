# `solfunmeme_state`

This crate provides state management functionality for the Solfunmeme application, handling application state, user preferences, and data persistence.

## Purpose

It serves as the central state management system for the application, providing reactive state handling, persistence, and synchronization across different components and views.

## Core Functionalities

### State Management
- **Application State**: Global application state management and synchronization
- **User State**: User preferences, settings, and session management
- **Data Persistence**: Local storage and state persistence across sessions
- **State Synchronization**: Real-time state updates and synchronization between components

### Meme State Management
- **Meme State**: Management of meme data, metadata, and relationships
- **Meme Operations**: State changes for meme creation, editing, and deletion
- **Meme Synchronization**: Keeping meme state consistent across the application

### Utility Functions
- **State Utilities**: Helper functions for state manipulation and validation
- **Persistence Helpers**: Utilities for saving and loading state from storage
- **State Observers**: Reactive state observation and change detection

## Usage (Conceptual)

```rust
use dioxus::prelude::*;
use solfunmeme_state::{
    use_memes::use_memes,
    state::AppState,
};

fn MyComponent(cx: Scope) -> Element {
    let memes = use_memes(cx);
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    
    render!(
        div {
            "Current meme count: {memes.len()}"
        }
    )
}
``` 