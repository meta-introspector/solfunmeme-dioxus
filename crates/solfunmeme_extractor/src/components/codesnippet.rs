use solfunmeme_function_analysis::CodeChunk as CodeSnippet;
use dioxus::prelude::*;
//use crate::extractor::CodeSnippet;
use std::collections::HashSet;
fn get_snippet_id(file_name: &str, snippet_idx: usize) -> String {
    format!("{}_{}", file_name, snippet_idx)
}

#[component]
pub fn CodeSnippetComponent(
    snippet: CodeSnippet,
    file_name: String,
    idx: usize,
    copied_snippets: Signal<HashSet<String>>,
) -> Element {
    let snippet_id = get_snippet_id(&file_name, idx);

    rsx! {
        div { class: "snippet-container", key: "{idx}",
            div { class: "snippet-header",
                span {
                    "🏷️ {snippet.language} (lines {snippet.line_start}-{snippet.line_end})"
                }
                button {
                    class: if copied_snippets.read().contains(&snippet_id) { "copy-btn copied" } else { "copy-btn" },
                    onclick: move |_| {},
                    "📋 Copy"
                }
            }
            div { class: "snippet-content",
                pre { class: "snippet-code", "{snippet.content}" }
            }
        }
    }
}

#[component]
pub fn CodeSnippetView(
    snippet: CodeSnippet,
    snippet_id: String,
    is_copied: bool,
    on_copy: EventHandler<(String, String)>,
) -> Element {
    rsx! {
    div { class: "snippet-container",
          div { class: "snippet-header",
                span {
                    "🏷️ {snippet.language} (lines {snippet.line_start}-{snippet.line_end})"
                }
                button {
                    class: if is_copied { "copy-btn copied" } else { "copy-btn" },
                    onclick: {
                        let content = snippet.content.clone();
                        let id = snippet_id.clone();
                        move |_| on_copy((content.clone(), id.clone()))
                    },
                    if is_copied { "✅ Copied!" } else { "📋 Copy" }
                }
            }
            div { class: "snippet-content",
                pre { class: "snippet-code", "{snippet.content}" }
            }
        }
    }
}
