use dioxus::prelude::*;
//use crate::extractor::CodeSnippet;
//use crate::extractor::ExtractedFile;
use crate::{
    components::codesnippet::CodeSnippetView,
};
use solfunmeme_function_analysis::{CodeChunk, ExtractedFile};

#[component]
pub fn FileResults(
    file: ExtractedFile,
    copied_snippets: std::collections::HashSet<String>,
    on_copy: EventHandler<(String, String)>,
    on_download: EventHandler<(Vec<CodeChunk>, String)>,
) -> Element {
    let unique_languages: Vec<String> = file
        .snippets
        .iter()
        .map(|s| s.language.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    rsx! {
        div {
            h2 { "📝 {file.name}" }

            if !file.snippets.is_empty() {
                div { class: "summary-stats",
                    p { "📊 Found {file.snippets.len()} code snippets in {file.total_lines} lines" }
                    p {
                        "🔤 Languages: {unique_languages.join(\", \")}"
                    }
                    div { class: "button-group",
                        button {
                            class: if copied_snippets.contains("all_snippets") { "copy-btn copied" } else { "copy-btn" },
                            onclick: {
                                let on_copy = on_copy.clone();
                                let combined = file.snippets.iter()
                                    .map(|s| s.content.clone())
                                    .collect::<Vec<_>>()
                                    .join("\n\n");
                                move |_| on_copy((combined.clone(), "all_snippets".to_string()))
                            },
                            if copied_snippets.contains("all_snippets") { "✅ Copied All!" } else { "📋 Copy All Snippets" }
                        }

                        for language in unique_languages.iter() {
                            button {
                                key: "{language}_download",
                                class: "download-btn",
                                onclick: {
                                    let on_download = on_download.clone();
                                    let snippets = file.snippets.clone();
                                    let lang = language.clone();
                                    move |_| on_download((snippets.clone(), lang.clone()))
                                },
                                "⬇️ Download {language.to_uppercase()}"
                            }
                        }
                    }
                }

                for (idx, snippet) in file.snippets.iter().enumerate() {
                    CodeSnippetView {
                        key: "{idx}",
                        snippet: snippet.clone(),
                        snippet_id: format!("{}_{}", file.name, idx),
                        is_copied: copied_snippets.contains(&format!("{}_{}", file.name, idx)),
                        on_copy,
                    }
                }
            } else {
                p { "ℹ️ No code snippets found in this file" }
            }
        }
    }
}
