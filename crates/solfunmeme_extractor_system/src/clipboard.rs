use dioxus::prelude::*;
use dioxus_clipboard::prelude::*;
use dioxus_logger::tracing::info;
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;

// Define a simple CodeSnippet type for this crate
#[derive(Debug, Clone)]
pub struct CodeSnippet {
    pub content: String,
    pub id: String,
}

pub fn copy_all_snippets_combined(
    snippets: Vec<CodeSnippet>,
    _copied_snippets: Signal<HashSet<String>>,
) {
    let combined = snippets
        .iter()
        .map(|snippet| snippet.content.clone())
        .collect::<Vec<_>>()
        .join("\n\n");

    copy_to_clipboard(combined, "all_snippets".to_string(), _copied_snippets);
}

pub fn copy_to_clipboard(
    text: String,
    _snippet_id: String,
    _copied_snippets: Signal<HashSet<String>>,
) {
    let _clipboard = use_clipboard();

    info!("going to save: {:?}", text);

    let _task = spawn_local(async move {
        let text2 = text.clone();
        let window = web_sys::window().expect("window");
        let nav = window.navigator().clipboard();
        let p = nav.write_text(&text2);
        let _result = wasm_bindgen_futures::JsFuture::from(p)
            .await
            .expect("clipboard populated");
        info!("clippyboy worked");
    });
}

pub fn create_copy_handler(
    _copied_snippets: Signal<HashSet<String>>,
) -> impl FnMut(String, String) + Clone {
    move |text: String, snippet_id: String| {
        copy_to_clipboard(text, snippet_id, _copied_snippets.clone());
    }
} 