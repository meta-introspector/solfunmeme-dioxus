use dioxus::prelude::*;

#[component]
pub fn Airdrop() -> Element {
    rsx! {
        div {
            class: "airdrop",
            h3 { "Airdrop" }
            p { "Airdrop functionality will be implemented here" }
        }
    }
} 