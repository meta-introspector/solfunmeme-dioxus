use crate::extractor::types::CodeSnippet;
use dioxus::prelude::*;
use log::info;
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;

pub fn copy_all_snippets_combined(
    snippets: Vec<CodeSnippet>,
    copied_snippets: Signal<HashSet<String>>,
) {
    let combined = snippets
        .iter()
        .map(|snippet| snippet.content.clone())
        .collect::<Vec<_>>()
        .join("\n\n");

    copy_to_clipboard(combined, "all_snippets".to_string(), copied_snippets);
}

pub fn copy_to_clipboard(
    text: String,
    _snippet_id: String,
    _copied_snippets: Signal<HashSet<String>>,
) {
    info!("going to save to clipboard");
    let _task = spawn_local(async move {
        let text2 = text.clone();
        let window = web_sys::window().expect("window");
        let nav = window.navigator().clipboard();
        let p = nav.write_text(&text2);
        let _result = wasm_bindgen_futures::JsFuture::from(p)
            .await
            .expect("clipboard populated");
        info!("clipboard write done");
    });
}

pub fn create_copy_handler(
    copied_snippets: Signal<HashSet<String>>,
) -> impl FnMut(String, String) + Clone {
    move |text: String, snippet_id: String| {
        copy_to_clipboard(text, snippet_id, copied_snippets.clone());
    }
}
