use dioxus::prelude::*;
//use crate::extractor::ExtractedFile;
use shared_analysis_types::ExtractedFile;
use std::collections::HashSet;

#[component]
pub fn ClearButton(
    files: Signal<Vec<ExtractedFile>>,
    copied_snippets: Signal<HashSet<String>>,
) -> Element {
    rsx! {
        button {
            class: "clear-btn",
            onclick: move |_| {
                files.write().clear();
                copied_snippets.write().clear();
            },
            "🗑️ Clear All Files"
        }
    }
}
