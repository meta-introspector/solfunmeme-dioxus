#![feature(fn_traits)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::console;

// Crypto utilities (simplified for demo - in production use proper crypto libraries)
use sha2::{Sha256, Digest};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};

// ============================================================================
// DATA MODELS
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub encrypted_password: Vec<u8>,
    pub nonce: Vec<u8>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct DecryptedEntry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug)]
pub struct NewPasswordForm {
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
}

impl Default for NewPasswordForm {
    fn default() -> Self {
        Self {
            title: String::new(),
            username: String::new(),
            password: String::new(),
            url: String::new(),
            notes: String::new(),
        }
    }
}

// ============================================================================
// BUSINESS LOGIC LAYER
// ============================================================================

pub struct CryptoManager {
    cipher: Aes256Gcm,
}

impl CryptoManager {
    pub fn new(master_password: &str) -> Self {
        // Derive key from master password using SHA256 (in production, use PBKDF2 or Argon2)
        let mut hasher = Sha256::new();
        hasher.update(master_password.as_bytes());
        let key_bytes = hasher.finalize();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        
        Self { cipher }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<(Vec<u8>, Vec<u8>), String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        match self.cipher.encrypt(&nonce, plaintext.as_bytes()) {
            Ok(ciphertext) => Ok((ciphertext, nonce.to_vec())),
            Err(_) => Err("Encryption failed".to_string()),
        }
    }

    pub fn decrypt(&self, ciphertext: &[u8], nonce: &[u8]) -> Result<String, String> {
        let nonce = Nonce::from_slice(nonce);
        match self.cipher.decrypt(nonce, ciphertext) {
            Ok(plaintext) => String::from_utf8(plaintext).map_err(|_| "Invalid UTF-8".to_string()),
            Err(_) => Err("Decryption failed".to_string()),
        }
    }
}

pub struct PasswordStore {
    entries: HashMap<String, PasswordEntry>,
}

impl PasswordStore {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, form: NewPasswordForm, crypto: &CryptoManager) -> Result<String, String> {
        if form.title.is_empty() || form.username.is_empty() || form.password.is_empty() {
            return Err("Please fill in all required fields".to_string());
        }

        let (encrypted_password, nonce) = crypto.encrypt(&form.password)?;
        let id = format!("{}", js_sys::Date::now() as u64);
        let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
        
        let entry = PasswordEntry {
            id: id.clone(),
            title: form.title,
            username: form.username,
            encrypted_password,
            nonce,
            url: if form.url.is_empty() { None } else { Some(form.url) },
            notes: if form.notes.is_empty() { None } else { Some(form.notes) },
            created_at: now.clone(),
            updated_at: now,
        };

        self.entries.insert(id.clone(), entry);
        console::log_1(&"Password saved successfully".into());
        Ok(id)
    }

    pub fn get_entry(&self, id: &str) -> Option<&PasswordEntry> {
        self.entries.get(id)
    }

    pub fn get_all_entries(&self) -> Vec<(String, PasswordEntry)> {
        self.entries.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn decrypt_entry(&self, id: &str, crypto: &CryptoManager) -> Result<DecryptedEntry, String> {
        let entry = self.get_entry(id).ok_or("Entry not found".to_string())?;
        let password = crypto.decrypt(&entry.encrypted_password, &entry.nonce)?;
        
        Ok(DecryptedEntry {
            id: entry.id.clone(),
            title: entry.title.clone(),
            username: entry.username.clone(),
            password,
            url: entry.url.clone(),
            notes: entry.notes.clone(),
            created_at: entry.created_at.clone(),
            updated_at: entry.updated_at.clone(),
        })
    }
}

pub struct AppState {
    pub is_locked: bool,
    pub master_password: String,
    pub crypto_manager: Option<CryptoManager>,
    pub password_store: PasswordStore,
    pub show_add_form: bool,
    pub selected_entry: Option<String>,
    pub error_message: String,
    pub form_data: NewPasswordForm,
    pub show_password: bool,
    pub decrypted_password: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            is_locked: true,
            master_password: String::new(),
            crypto_manager: None,
            password_store: PasswordStore::new(),
            show_add_form: false,
            selected_entry: None,
            error_message: String::new(),
            form_data: NewPasswordForm::default(),
            show_password: false,
            decrypted_password: String::new(),
        }
    }
}

// ============================================================================
// BUSINESS LOGIC ACTIONS
// ============================================================================

pub fn unlock_vault(state: &mut AppState) {
    if state.master_password.is_empty() {
        state.error_message = "Please enter master password".to_string();
        return;
    }
    
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("Password: {}", &state.master_password)));

    let crypto = CryptoManager::new(&state.master_password);
    state.crypto_manager = Some(crypto);
    state.is_locked = false;
    state.error_message.clear();
    console::log_1(&"Vault unlocked".into());
}

pub fn lock_vault(state: &mut AppState) {
    state.is_locked = true;
    state.master_password.clear();
    state.crypto_manager = None;
    state.selected_entry = None;
    state.show_add_form = false;
    state.show_password = false;
    state.decrypted_password.clear();
    console::log_1(&"Vault locked".into());
}

pub fn show_add_form(state: &mut AppState) {
    state.show_add_form = true;
    state.selected_entry = None;
    state.form_data = NewPasswordForm::default();
}

pub fn hide_add_form(state: &mut AppState) {
    state.show_add_form = false;
    state.form_data = NewPasswordForm::default();
    state.error_message.clear();
}

pub fn select_entry(state: &mut AppState, entry_id: String) {
    state.selected_entry = Some(entry_id);
    state.show_add_form = false;
    state.show_password = false;
    state.decrypted_password.clear();
}

pub fn save_password(state: &mut AppState) {
    if let Some(crypto) = &state.crypto_manager {
        match state.password_store.add_entry(state.form_data.clone(), crypto) {
            Ok(_) => {
                state.form_data = NewPasswordForm::default();
                state.show_add_form = false;
                state.error_message.clear();
            }
            Err(e) => {
                state.error_message = format!("Failed to save password: {}", e);
            }
        }
    } else {
        state.error_message = "Crypto manager not available".to_string();
    }
}

pub fn toggle_password_visibility(state: &mut AppState) {
    if !state.show_password {
        if let (Some(crypto), Some(entry_id)) = (&state.crypto_manager, &state.selected_entry) {
            match state.password_store.decrypt_entry(entry_id, crypto) {
                Ok(decrypted) => {
                    state.decrypted_password = decrypted.password;
                    state.show_password = true;
                }
                Err(_) => {
                    console::log_1(&"Failed to decrypt password".into());
                }
            }
        }
    } else {
        state.show_password = false;
        state.decrypted_password.clear();
    }
}

pub fn copy_to_clipboard(text: String) {
    let _ = web_sys::window()
        .unwrap()
        .navigator()
        .clipboard();
 //       .unwrap()
        //.write_text(&text);
}

// ============================================================================
// PRESENTATION LAYER
// ============================================================================

#[component]
pub fn App() -> Element {
    let mut app_state = use_signal(AppState::default);

    rsx! {
        div {
            class: "min-h-screen bg-gray-900 text-white p-6",
            div {
                class: "max-w-4xl mx-auto",
                
                AppHeader { app_state }
                
                if !app_state.read().error_message.is_empty() {
                    ErrorMessage { message: app_state.read().error_message.clone() }
                }

                if app_state.read().is_locked {
                    LoginScreen { app_state }
                } else {
                    MainInterface { app_state }
                }
            }
        }
    }
}

#[component]
fn AppHeader(app_state: Signal<AppState>) -> Element {
    let handle_lock_vault = {
        let mut app_state = app_state.clone();
        move |_| {
            lock_vault(&mut app_state.write());
        }
    };

    rsx! {
        header {
            class: "flex justify-between items-center mb-8 p-4 bg-gray-800 rounded-lg",
            h1 {
                class: "text-3xl font-bold text-blue-400",
                "🔐 SecureVault Password Manager"
            }
            if !app_state.read().is_locked {
                button {
                    class: "px-4 py-2 bg-red-600 hover:bg-red-700 rounded transition-colors",
                    onclick: handle_lock_vault,
                    "🔒 Lock Vault"
                }
            }
        }
    }
}

#[component]
fn ErrorMessage(message: String) -> Element {
    rsx! {
        div {
            class: "mb-4 p-3 bg-red-600 rounded-lg",
            "{message}"
        }
    }
}

#[component]
fn LoginScreen(app_state: Signal<AppState>) -> Element {
    // let handle_unlock = {
    //     let mut app_state = app_state.clone();
    //     move |_| {
    //          unlock_vault(&mut app_state.write());
    //     }
    // };

    let handle_password_change = {
        let mut app_state = app_state.clone();
        move |evt: FormEvent| {
            app_state.write().master_password = evt.value();
        }
    };

    let handle_form_submit = {
        //let handle_unlock2 = handle_unlock.clone();
        move |e: FormEvent| {
            e.prevent_default();
            //handle_unlock(e);
            let mut app_state = app_state.clone();
    //     move |_| {
            unlock_vault(&mut app_state.write());
    //     }
        }
    };

    rsx! {
        div {
            class: "max-w-md mx-auto bg-gray-800 p-8 rounded-lg shadow-lg",
            h2 {
                class: "text-2xl font-bold mb-6 text-center",
                "Enter Master Password"
            }
            form {
                onsubmit: handle_form_submit,
                div {
                    class: "mb-4",
                    input {
                        class: "w-full px-4 py-3 bg-gray-700 border border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent",
                        r#type: "password",
                        placeholder: "Master Password",
                        value: "{app_state.read().master_password}",
                        oninput: handle_password_change,
                        autofocus: true,
                    }
                }
                button {
                    class: "w-full px-4 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg transition-colors font-semibold",
                    r#type: "submit",
                    "🔓 Unlock Vault"
                }
            }
            p {
                class: "mt-4 text-sm text-gray-400 text-center",
                "Your master password encrypts all stored data. Make sure it's strong and memorable!"
            }
        }
    }
}

#[component]
fn MainInterface(app_state: Signal<AppState>) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
            
            div {
                class: "lg:col-span-1",
                PasswordList { app_state }
            }

            div {
                class: "lg:col-span-2",
                if app_state.read().show_add_form {
                    AddPasswordForm { app_state }
                } else if app_state.read().selected_entry.is_some() {
                    PasswordDetail { app_state }
                } else {
                    WelcomeScreen {}
                }
            }
        }
    }
}

#[component]
fn PasswordList(app_state: Signal<AppState>) -> Element {
    let handle_add_new = {
        let mut app_state = app_state.clone();
        move |_| {
            show_add_form(&mut app_state.write());
        }
    };

    let entries = app_state.read().password_store.get_all_entries();

    rsx! {
        div {
            class: "bg-gray-800 p-4 rounded-lg",
            div {
                class: "flex justify-between items-center mb-4",
                h2 {
                    class: "text-xl font-semibold",
                    "Passwords ({app_state.read().password_store.len()})"
                }
                button {
                    class: "px-3 py-2 bg-green-600 hover:bg-green-700 rounded text-sm transition-colors",
                    onclick: handle_add_new,
                    "➕ Add New"
                }
            }
            
            div {
                class: "space-y-2 max-h-96 overflow-y-auto",
                {
                    entries.into_iter().map(|(id, entry)| {
                        let entry_id = id.clone();
                        let is_selected = app_state.read().selected_entry == Some(id.clone());
                        
                        let handle_select = {
                            let mut app_state = app_state.clone();
                            move |_| {
                                select_entry(&mut app_state.write(), entry_id.clone());
                            }
                        };
                        
                        rsx! {
                            div {
                                key: "{id}",
                                class: if is_selected {
                                    "p-3 bg-blue-600 rounded cursor-pointer transition-colors"
                                } else {
                                    "p-3 bg-gray-700 hover:bg-gray-600 rounded cursor-pointer transition-colors"
                                },
                                onclick: handle_select,
                                div {
                                    class: "font-medium",
                                    "{entry.title}"
                                }
                                div {
                                    class: "text-sm text-gray-300",
                                    "{entry.username}"
                                }
                                if let Some(url) = &entry.url {
                                    div {
                                        class: "text-xs text-gray-400 truncate",
                                        "{url}"
                                    }
                                }
                            }
                        }
                    })
                }
                
                if app_state.read().password_store.is_empty() {
                    div {
                        class: "text-center text-gray-400 py-8",
                        "No passwords stored yet.\nAdd your first password!"
                    }
                }
            }
        }
    }
}

#[component]
fn AddPasswordForm(app_state: Signal<AppState>) -> Element {
    let handle_cancel = {
        let mut app_state = app_state.clone();
        move |_| {
            hide_add_form(&mut app_state.write());
        }
    };

    // let handle_save = {
    //     //..let mut app_state = app_state.clone();
    //     // move |_| {
    //     //     save_password(&mut app_state.write());
    //     // }
    // };

    let handle_form_submit = {
        //let handle_save = handle_save.clone();
        move |e: FormEvent| {
            e.prevent_default();
            //handle_save.call(());
            let mut app_state = app_state.clone();
            save_password(&mut app_state.write());
        }
    };

    let handle_title_change = {
        let mut app_state = app_state.clone();
        move |evt: FormEvent| {
            app_state.write().form_data.title = evt.value();
        }
    };

    let handle_username_change = {
        let mut app_state = app_state.clone();
        move |evt: FormEvent| {
            app_state.write().form_data.username = evt.value();
        }
    };

    let handle_password_change = {
        let mut app_state = app_state.clone();
        move |evt: FormEvent| {
            app_state.write().form_data.password = evt.value();
        }
    };

    let handle_url_change = {
        let mut app_state = app_state.clone();
        move |evt: FormEvent| {
            app_state.write().form_data.url = evt.value();
        }
    };

    let handle_notes_change = {
        let mut app_state = app_state.clone();
        move |evt: FormEvent| {
            app_state.write().form_data.notes = evt.value();
        }
    };

    rsx! {
        div {
            class: "bg-gray-800 p-6 rounded-lg",
            div {
                class: "flex justify-between items-center mb-6",
                h2 {
                    class: "text-xl font-semibold",
                    "Add New Password"
                }
                button {
                    class: "text-gray-400 hover:text-white",
                    onclick: handle_cancel,
                    "✕ Cancel"
                }
            }

            form {
                class: "space-y-4",
                onsubmit: handle_form_submit,
                
                div {
                    label {
                        class: "block text-sm font-medium mb-2",
                        "Title *"
                    }
                    input {
                        class: "w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded focus:ring-2 focus:ring-blue-500",
                        r#type: "text",
                        placeholder: "e.g., Gmail, GitHub, Bank",
                        value: "{app_state.read().form_data.title}",
                        oninput: handle_title_change,
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium mb-2",
                        "Username/Email *"
                    }
                    input {
                        class: "w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded focus:ring-2 focus:ring-blue-500",
                        r#type: "text",
                        placeholder: "username or email",
                        value: "{app_state.read().form_data.username}",
                        oninput: handle_username_change,
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium mb-2",
                        "Password *"
                    }
                    input {
                        class: "w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded focus:ring-2 focus:ring-blue-500",
                        r#type: "password",
                        placeholder: "Enter password",
                        value: "{app_state.read().form_data.password}",
                        oninput: handle_password_change,
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium mb-2",
                        "URL"
                    }
                    input {
                        class: "w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded focus:ring-2 focus:ring-blue-500",
                        r#type: "url",
                        placeholder: "https://example.com",
                        value: "{app_state.read().form_data.url}",
                        oninput: handle_url_change,
                    }
                }

                div {
                    label {
                        class: "block text-sm font-medium mb-2",
                        "Notes"
                    }
                    textarea {
                        class: "w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded focus:ring-2 focus:ring-blue-500",
                        rows: "3",
                        placeholder: "Additional notes...",
                        value: "{app_state.read().form_data.notes}",
                        oninput: handle_notes_change,
                    }
                }

                button {
                    class: "w-full px-4 py-2 bg-green-600 hover:bg-green-700 rounded transition-colors font-semibold",
                    r#type: "submit",
                    "💾 Save Password"
                }
            }
        }
    }
}

#[component]
fn PasswordDetail(app_state: Signal<AppState>) -> Element {
    let entry_id = app_state.read().selected_entry.clone().unwrap();
    
    let handle_toggle_password = {
        let mut app_state = app_state.clone();
        move |_| {
            toggle_password_visibility(&mut app_state.write());
        }
    };

    if let Some(entry) = app_state.read().password_store.get_entry(&entry_id) {
        let entry = entry.clone();
        
        let handle_copy_username = {
            let username = entry.username.clone();
            move |_| {
                copy_to_clipboard(username.clone());
            }
        };

        let handle_copy_password = {
            let decrypted_password = app_state.read().decrypted_password.clone();
            move |_| {
                copy_to_clipboard(decrypted_password.clone());
            }
        };

        let handle_copy_url = {
            let url = entry.url.clone().unwrap_or_default();
            move |_| {
                copy_to_clipboard(url.clone());
            }
        };

        rsx! {
            div {
                class: "bg-gray-800 p-6 rounded-lg",
                h2 {
                    class: "text-2xl font-semibold mb-6 text-blue-400",
                    "{entry.title}"
                }

                div {
                    class: "space-y-4",
                    
                    div {
                        class: "flex justify-between items-center p-3 bg-gray-700 rounded",
                        div {
                            span {
                                class: "text-sm text-gray-400",
                                "Username: "
                            }
                            span {
                                class: "font-mono",
                                "{entry.username}"
                            }
                        }
                        button {
                            class: "px-2 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm transition-colors",
                            onclick: handle_copy_username,
                            "📋 Copy"
                        }
                    }

                    div {
                        class: "flex justify-between items-center p-3 bg-gray-700 rounded",
                        div {
                            span {
                                class: "text-sm text-gray-400",
                                "Password: "
                            }
                            span {
                                class: "font-mono",
                                if app_state.read().show_password {
                                    "{app_state.read().decrypted_password}"
                                } else {
                                    "••••••••••••"
                                }
                            }
                        }
                        div {
                            class: "flex gap-2",
                            button {
                                class: "px-2 py-1 bg-yellow-600 hover:bg-yellow-700 rounded text-sm transition-colors",
                                onclick: handle_toggle_password,
                                if app_state.read().show_password { "👁️ Hide" } else { "👁️ Show" }
                            }
                            if app_state.read().show_password {
                                button {
                                    class: "px-2 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm transition-colors",
                                    onclick: handle_copy_password,
                                    "📋 Copy"
                                }
                            }
                        }
                    }

                    if let Some(url) = &entry.url {
                        div {
                            class: "flex justify-between items-center p-3 bg-gray-700 rounded",
                            div {
                                span {
                                    class: "text-sm text-gray-400",
                                    "URL: "
                                }
                                a {
                                    class: "text-blue-400 hover:text-blue-300 underline",
                                    href: "{url}",
                                    target: "_blank",
                                    "{url}"
                                }
                            }
                            button {
                                class: "px-2 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm transition-colors",
                                onclick: handle_copy_url,
                                "📋 Copy"
                            }
                        }
                    }

                    if let Some(notes) = &entry.notes {
                        div {
                            class: "p-3 bg-gray-700 rounded",
                            div {
                                class: "text-sm text-gray-400 mb-2",
                                "Notes:"
                            }
                            div {
                                class: "whitespace-pre-wrap",
                                "{notes}"
                            }
                        }
                    }

                    div {
                        class: "text-xs text-gray-500 pt-4 border-t border-gray-600",
                        div { "Created: {entry.created_at}" }
                        div { "Updated: {entry.updated_at}" }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "bg-gray-800 p-6 rounded-lg text-center",
                "Entry not found"
            }
        }
    }
}

#[component]
fn WelcomeScreen() -> Element {
    rsx! {
        div {
            class: "bg-gray-800 p-8 rounded-lg text-center",
            h2 {
                class: "text-2xl font-semibold mb-4 text-blue-400",
                "Welcome to SecureVault"
            }
            p {
                class: "text-gray-300 mb-6",
                "Your passwords are encrypted with AES-256 encryption using your master password as the key. All data is stored locally and never transmitted to any server."
            }
            div {
                class: "grid grid-cols-1 md:grid-cols-2 gap-4 text-left",
                div {
                    class: "p-4 bg-gray-700 rounded",
                    h3 {
                        class: "font-semibold text-green-400 mb-2",
                        "🔒 Security Features"
                    }
                    ul {
                        class: "text-sm text-gray-300 space-y-1",
                        li { "• AES-256-GCM encryption" }
                        li { "• Master password protection" }
                        li { "• Local storage only" }
                        li { "• Zero knowledge architecture" }
                    }
                }
                div {
                    class: "p-4 bg-gray-700 rounded",
                    h3 {
                        class: "font-semibold text-blue-400 mb-2",
                        "🚀 Getting Started"
                    }
                    ul {
                        class: "text-sm text-gray-300 space-y-1",
                        li { "• Click 'Add New' to create entries" }
                        li { "• Store passwords, usernames, URLs" }
                        li { "• Add notes for additional info" }
                        li { "• Click entries to view details" }
                    }
                }
            }
        }
    }
}

// fn main() {
//     dioxus::launch(App);
// }