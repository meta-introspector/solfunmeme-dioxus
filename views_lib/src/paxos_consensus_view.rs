use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn PaxosConsensusView(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "paxos-consensus-view p-6 bg-gradient-to-r from-green-100 to-blue-100 dark:from-gray-800 dark:to-gray-900 rounded-xl shadow-lg",
            h2 { class: "text-2xl font-bold mb-4 text-green-700 dark:text-green-300", "Paxos Consensus Meme View" }
            p { class: "mb-2 text-gray-700 dark:text-gray-300", "Visualizing the SOLFUNMEME consensus process as a meme-driven protocol (see " a { href: "https://github.com/meta-introspector/meta-meme/issues/99", class: "underline text-blue-600", "meta-meme #99" } ")" }
            div { class: "flex flex-col space-y-4 mt-4",
                div { class: "flex items-center space-x-3",
                    span { class: "text-3xl", "🤖" }
                    span { class: "text-3xl", "🤖🦾" }
                    span { class: "text-3xl", "🤖🤖" }
                    span { class: "ml-2 font-semibold", "Nodes" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-3xl", "👑" }
                    span { class: "ml-2 font-semibold", "Leadership Election" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-3xl", "📜" }
                    span { class: "ml-2 font-semibold", "Proposal Phase" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-3xl", "✅" }
                    span { class: "ml-2 font-semibold", "Acceptance Phase" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-3xl", "👍" }
                    span { class: "ml-2 font-semibold", "Consensus Achieved" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-3xl", "⚠️" }
                    span { class: "ml-2 font-semibold", "Failure Handling" }
                }
                div { class: "flex items-center space-x-3",
                    span { class: "text-3xl", "🔄" }
                    span { class: "ml-2 font-semibold", "Node Recovery" }
                }
            }
        }
    })
} 