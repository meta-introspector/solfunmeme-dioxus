//! Provides functionality for managing memes in a Dioxus application with local storage persistence.
//!
//! This module implements a state management system for memes using Dioxus signals and browser's
//! localStorage API through gloo_storage.

/// Represents a single meme with a unique identifier and content.
///
/// # Fields
/// * `id` - Unique identifier for the meme
/// * `content` - The actual content/data of the meme
///
/// # Examples
/// ```
/// let meme = Meme {
///     id: "unique_id".to_string(),
///     content: "meme_content".to_string()
/// };
/// ```
/// Container for meme-related state.
///
/// # Fields
/// * `key` - Storage key prefix for localStorage operations
/// * `memes` - Vector containing all stored memes

/// Main state management structure for memes.
///
/// Handles both the collection of memes and tracks the currently active meme.
///
/// # Fields
/// * `inner` - Signal containing the meme collection state
/// * `active_meme` - Signal tracking the currently selected meme's ID

/// Implementation of meme management functionality.
///
/// Provides methods for:
/// * Retrieving all memes
/// * Adding new memes
/// * Setting the active meme
/// * Getting the currently active meme
///
/// All operations are automatically persisted to localStorage.
/// Hook function for creating a new meme management instance.
///
/// # Arguments
/// * `key` - Implements ToString, used as a prefix for localStorage keys
///
/// # Returns
/// * `UseMemes` - A new instance of the meme management system
///
/// # Examples
/// ```
/// use crate::state::use_memes
/// let memes = use_memes("app");
/// ```
///
/// This will create a new meme management instance with localStorage keys
/// prefixed with "app_".
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

#[derive(Clone, PartialEq)]
pub struct Meme {
    pub id: String,
    pub content: String,
}

#[derive(Clone, PartialEq)]
pub struct MemeState {
    pub key: String,
    pub memes: Vec<Meme>,
}

#[derive(Clone, PartialEq)]
pub struct UseMemes {
    inner: Signal<MemeState>,
    active_meme: Signal<String>,
}

impl UseMemes {
    #[allow(dead_code)]
    pub fn get_all_memes(&self) -> Vec<Meme> {
        self.inner.read().memes.clone()
    }
    #[allow(dead_code)]
    pub fn add_meme(&mut self, meme: Meme) {
        let mut inner = self.inner.write();
        inner.memes.push(meme.clone());
        // LocalStorage::set(&format!("{}_memes", inner.key), &inner.memes).ok();
    }
    #[allow(dead_code)]
    pub fn set_active_meme(&mut self, id: String) {
        self.active_meme.set(id.clone());
        let key_name = &self.inner.read().key;
        LocalStorage::set(format!("{key_name}_active_meme",), &id).ok();
    }
    #[allow(dead_code)]
    pub fn active_meme(&self) -> Option<Meme> {
        let id = self.active_meme.read().clone();
        self.inner.read().memes.iter().find(|m| m.id == id).cloned()
    }
}

#[allow(dead_code)]
// FIXME : exercize
pub fn use_memes(key: impl ToString) -> UseMemes {
    let key = key.to_string();
    let key_for_state = key.clone();
    let key_for_active = key.clone();

    let state = use_signal(move || {
        // let memes: Vec<Meme> = LocalStorage::get(&format!("{}_memes", &key_for_state)).ok().unwrap_or_default();
        let memes: Vec<Meme> = vec![];
        MemeState {
            key: key_for_state,
            memes,
        }
    });

    let active_meme = use_signal(move || {
        LocalStorage::get(format!("{key_for_active}_active_meme"))
            .ok()
            .unwrap_or_default()
    });

    UseMemes {
        inner: state,
        active_meme,
    }
}
