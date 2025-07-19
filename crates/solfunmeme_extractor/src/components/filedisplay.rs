use dioxus::prelude::*;
//use crate::extractor::FileEngine;
//use crate::extractor::ExtractedFile;
use std::collections::HashSet;
//use crate::extractor::CodeSnippet;
use crate::{
    components::{codesnippet::CodeSnippetComponent, filesummary::FileSummary},
};
use solfunmeme_function_analysis::ExtractedFile;

#[component]
pub fn FileDisplay(
    file: ExtractedFile,
    copied_snippets: Signal<HashSet<String>>,
) -> Element {
    //    let get_snippet_id = |file_name: &str, snippet_idx: usize| -> String {
    //        format!("{}_{}", file_name, snippet_idx)
    //    };

    rsx! {
        div { key: "{file.name}",
            h2 { "📝 {file.name}" }

            if !file.snippets.is_empty() {
                FileSummary {
                    file: file.clone(),
                    copied_snippets
                }

                for (idx, snippet) in file.snippets.iter().enumerate() {
                    CodeSnippetComponent {
                        snippet: snippet.clone(),
                        file_name: file.name.clone(),
                        idx,
                        copied_snippets
                    }
                }
            } else {
                p { "ℹ️ No code snippets found in this file" }
            }
        }
    }
}