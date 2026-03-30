use crate::{model::lean::style::Styles, playground::MenuOption};
use dioxus::prelude::*;

#[component]
pub fn ConnectionButtons(on_menu_change: EventHandler<MenuOption>) -> Element {
    rsx! {
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::ConnectionManagement),
            "Connection Management"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::ConnectionList),
            "Connection List"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Connections),
            "Connections"
        }
    }
}
