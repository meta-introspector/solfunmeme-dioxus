use dioxus::prelude::*;
use shared_types_lib::MenuOption;

// Note: This component will need Styles from model_lib once we move it
// For now, we'll use a placeholder style function
fn primary_button() -> &'static str {
    "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded m-1"
}

#[component]
pub fn ManagementButtons(on_menu_change: EventHandler<MenuOption>) -> Element {
    rsx! {
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Dashboard),
            "Dashboard"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::ClustersManagement),
            "Clusters Management"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Clusters),
            "Clusters"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Airdrop),
            "Airdrop"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Accounts),
            "Accounts"
        }
        button {
            class: "{primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::EmojiMatrix),
            "Emoji Matrix"
        }
    }
} 