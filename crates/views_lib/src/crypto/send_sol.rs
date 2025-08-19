use dioxus::prelude::*;

#[component]
pub fn SendSol() -> Element {
    rsx! {
        div {
            class: "send-sol",
            h3 { "Send SOL" }
            p { "Send SOL functionality will be implemented here" }
        }
    }
} 