use crate::extractor::types::CodeSnippet;
use dioxus::prelude::*;
use dioxus_clipboard::prelude::*;
use dioxus_logger::tracing::info;
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
    snippet_id: String,
    copied_snippets: Signal<HashSet<String>>,
) {
    let clipboard = use_clipboard();

    info!("going to save: {:?}", text);

    // let eval = document::eval(r#"
    //                     const input = {text};
    //                     input.select();
    //                     const success = document.execCommand('copy');
    //                     return success ? 'Copied!' : 'Copy failed';
    //                 "#);
    // if let Ok(result) = eval.await {
    //     if let Ok(serde_json::Value::String(message)) = result {
    //         copy_status.set(message);
    //     }
    // }

    let _task = spawn_local(async move {
        let text2 = text.clone();
        let window = web_sys::window().expect("window"); // { obj: val };
        let nav = window.navigator().clipboard();
        let p = nav.write_text(&text2);
        let result = wasm_bindgen_futures::JsFuture::from(p)
            .await
            .expect("clipboard populated");
        info!("clippyboy worked");
    });

    // match clipboard.set(text) {
    //     Ok(o) => {
    // 	    info!("saved text: {:?}", o);
    //         spawn(async move {
    //             copied_snippets.write().insert(snippet_id.clone());
    // 		info!("copied_snippets: {:?}", copied_snippets);
    //             TimeoutFuture::new(2000).await;
    //             copied_snippets.write().remove(&snippet_id);
    //         });
    //     }
    //     Err(e) => {
    //         info!("Failed to copy to clipboard: {:?}", e);
    //     }
    // }
    // if let Ok(content) = clipboard.get() {
    // 	info!("GOT Clipboard {:?}", content);
    // }
}

// pub fn copy_to_clipboard(text: String, snippet_id: String, mut copied_snippets: Signal<HashSet<String>>) {

pub fn create_copy_handler(
    copied_snippets: Signal<HashSet<String>>,
) -> impl FnMut(String, String) + Clone {
    move |text: String, snippet_id: String| {
        copy_to_clipboard(text, snippet_id, copied_snippets.clone());
    }
}
