use dioxus::prelude::*;

#[component]
pub fn Accounts() -> Element {
    rsx! {
        div {
            class: "accounts",
            h3 { "Accounts" }
            p { "Accounts functionality will be implemented here" }
        }
    }
} 