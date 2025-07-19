use dioxus::prelude::*;
//use crate::extractor::components::filesummary::FileSummary;
//use crate::extractor::ExtractedFile;
use std::collections::HashSet;
//use crate::extractor::CodeSnippet;
use solfunmeme_function_analysis::ExtractedFile;
#[component]
pub fn FileSummary(
    file: ExtractedFile,
    copied_snippets: Signal<HashSet<String>>,
) -> Element {
    rsx! {
        div { class: "summary-stats",
            p { "📊 Found {file.snippets.len()} code snippets in {file.total_lines} lines" }
            p {
                "🔤 Languages: {file.snippets.iter().map(|s| s.language.as_str()).collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>().join(\", \")}"
            }
            button {
                class: "copy-btn",
                "📋 Copy All Snippets"
            }
        }
    }
}
