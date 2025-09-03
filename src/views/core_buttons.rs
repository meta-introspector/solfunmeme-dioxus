use crate::{model::lean::style::Styles, playground::MenuOption};
use dioxus::prelude::*;

#[component]
pub fn CoreButtons(on_menu_change: EventHandler<MenuOption>) -> Element {
    rsx! {
    button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Embedding),
            "Embedding Operations"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::PerformanceCharts),
            "Performance Charts"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::BertTest),
            "🧠 WASM BERT Test"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::RustParser),
            "🔧 Rust AST Parser"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::MemeManagement),
            "Meme Management"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::ExpressionParsing),
            "Expression Parsing"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Encryption),
            "Encryption"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::MetaMemeOperations),
            "MetaMeme Operations"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::StylingAndEmojis),
            "Styling & Emojis"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::MonsterMetaMeme),
            "🧬 Monster Meta-Meme"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::SolFunMeme),
            "🧬 SOLFUNMEME"
        }
    button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Extractor),
            "Exraction"
        }
    button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::SourceBrowser),
            "Source Browser"
        }
    }
}
