use crate::{model::lean::style::Styles, playground::MenuOption};
use dioxus::prelude::*;

#[component]
pub fn CryptoButtons(on_menu_change: EventHandler<MenuOption>) -> Element {
    rsx! {
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::CryptoFrontend),
            "Crypto Frontend"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Memes),
            "Memes"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Lean),
            "Lean"
        }
    }
}
