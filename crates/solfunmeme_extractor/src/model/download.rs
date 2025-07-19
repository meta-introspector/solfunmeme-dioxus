use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;

use crate::model::files::create_download_filename;
use solfunmeme_function_analysis::CodeChunk as CodeSnippet;
//use crate::extractor::CodeSnippet;
pub fn download_rust_snippets_as_file(snippets: &[CodeSnippet]) {
    let rust_snippets: Vec<_> = snippets
        .iter()
        .filter(|s| s.language.to_lowercase() == "rust" || s.language.to_lowercase() == "rs")
        .collect();

    if rust_snippets.is_empty() {
        return;
    }

    let combined = rust_snippets
        .iter()
        .map(|snippet| snippet.content.clone())
        .collect::<Vec<_>>()
        .join("\n\n");

    if let Some(window) = window() {
        if let Ok(blob) = web_sys::Blob::new_with_str_sequence(
            &js_sys::Array::from_iter([JsValue::from(combined)]),
        ) {
            if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                if let Some(document) = window.document() {
                    if let Ok(anchor) = document.create_element("a") {
                        let anchor = anchor.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                        anchor.set_href(&url);
                        anchor.set_download("rust_snippets.rs");
                        let _ = document.body().unwrap().append_child(&anchor);
                        anchor.click();
                        let _ = document.body().unwrap().remove_child(&anchor);
                        let _ = web_sys::Url::revoke_object_url(&url);
                    }
                }
            }
        }
    }
}

pub fn create_download_handler() -> impl FnMut((Vec<CodeSnippet>, String)) {
    move |args: (Vec<CodeSnippet>, String)| {
        let (snippets, language) = args;
        let filtered_snippets: Vec<_> = snippets
            .iter()
            .filter(|s| s.language.to_lowercase() == language.to_lowercase())
            .collect();

        if filtered_snippets.is_empty() {
            return;
        }

        let combined = filtered_snippets
            .iter()
            .map(|snippet| snippet.content.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        let filename = create_download_filename(&language);

        #[cfg(target_arch = "wasm32")]
        download_web(&combined, &filename);

        #[cfg(not(target_arch = "wasm32"))]
        download_desktop(&combined, &filename);
    }
}

#[cfg(target_arch = "wasm32")]
fn download_web(content: &str, filename: &str) {
    // if let Some(window) = window() {
    //     if let Ok(blob) = Blob::new_with_str_sequence_and_options(
    //         &Array::from_iter([content.into()]),
    //         BlobPropertyBag::new().type_("text/plain")
    //     ) {
    //         if let Ok(url) = Url::create_object_url_with_blob(&blob) {
    //             if let Some(document) = window.document() {
    //                 if let Ok(anchor) = document.create_element("a") {
    //                     let anchor = anchor.dyn_into::<HtmlAnchorElement>().unwrap();
    //                     anchor.set_href(&url);
    //                     anchor.set_download(filename);
    //                     let _ = document.body().unwrap().append_child(&anchor);
    //                     anchor.click();
    //                     let _ = document.body().unwrap().remove_child(&anchor);
    //                     let _ = Url::revoke_object_url(&url);
    //                 }
    //             }
    //         }
    //     }
    // }
}

#[cfg(not(target_arch = "wasm32"))]
fn download_desktop(content: &str, filename: &str) {
    if let Err(e) = std::fs::write(filename, content) {
        log::error!("Failed to write file: {:?}", e);
    } else {
        log::info!("File saved as: {}", filename);
    }
}
