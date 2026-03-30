use dioxus::prelude::*;

//```rust
// #[component]
// pub fn WelcomeMessage() -> Element {
//     rsx! {
//         div { class: "summary-stats",
//             p { "👋 Welcome! Upload some markdown files to get started extracting code snippets." }
//             p { "💡 This tool will find all code blocks wrapped in ``` and make them easy to copy." }
//         }
//     }
// }

#[component]
pub fn WelcomeMessage(show: bool) -> Element {
    if show {
        rsx! {
            div { class: "summary-stats",
                p { "👋 Welcome! Upload some markdown files to get started extracting code snippets." }
                p { "💡 This tool will find all code blocks wrapped in ``` and make them easy to copy." }
            }
        }
    } else {
        rsx! { div {} }
    }
}
//}
