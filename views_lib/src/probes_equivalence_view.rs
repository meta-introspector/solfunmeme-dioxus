use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn ProbesEquivalenceView(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "probes-equivalence-view p-6 bg-gradient-to-r from-yellow-100 to-green-100 dark:from-gray-800 dark:to-gray-900 rounded-xl shadow-lg",
            h2 { class: "text-2xl font-bold mb-4 text-yellow-700 dark:text-yellow-300", "Probes, eBPF, and Language Equivalence" }
            p { class: "mb-2 text-gray-700 dark:text-gray-300", "Visualizing cross-language data flow and equivalence analysis (see " a { href: "https://github.com/meta-introspector/meta-meme/issues/101", class: "underline text-blue-600", "meta-meme #101" } ")" }
            div { class: "flex flex-col space-y-4 mt-4",
                div { class: "flex items-center space-x-3",
                    span { class: "text-2xl", "🛠️" }
                    span { class: "font-semibold", "Instrument Code (Perf, uprobes, eBPF, LLVM, GCC)" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-2xl", "📊" }
                    span { class: "font-semibold", "Collect Data" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-2xl", "🔬" }
                    span { class: "font-semibold", "Visualize Flow" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-2xl", "⚖️" }
                    span { class: "font-semibold", "Compare Equivalence (Sealevel, Rust, WASM, Dioxus)" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-2xl", "😀✨" }
                    span { class: "font-semibold", "Emoji Visualization" }
                }
            }
        }
    })
} 