use dioxus::prelude::*;
#[component]
pub fn ExtractorAppHeader() -> Element {
    rsx! {
        h1 { "📄 Markdown Code Extractor" }
        p { "Upload markdown files to extract and copy code snippets with ease" }
    }
}
