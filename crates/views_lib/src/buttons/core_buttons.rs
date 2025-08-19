use dioxus::prelude::*;
use shared_types_lib::MenuOption;

// Note: This component will need Styles from model_lib once we move it
// For now, we'll use a placeholder style function
fn primary_button() -> &'static str {
    "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded m-1"
}

#[component]
pub fn CoreButtons(on_menu_change: EventHandler<MenuOption>) -> Element {
    rsx! {
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Embedding),
            "Embedding Operations"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::PerformanceCharts),
            "Performance Charts"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::BertTest),
            "🧠 WASM BERT Test"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::RustParser),
            "🔧 Rust AST Parser"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::MemeManagement),
            "Meme Management"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::ExpressionParsing),
            "Expression Parsing"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Encryption),
            "Encryption"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::MetaMemeOperations),
            "MetaMeme Operations"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::StylingAndEmojis),
            "Styling & Emojis"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::MonsterMetaMeme),
            "🧬 Monster Meta-Meme"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::SolFunMeme),
            "🧬 SOLFUNMEME"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Extractor),
            "Exraction"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::SourceBrowser),
            "Source Browser"
        }
    }
} 