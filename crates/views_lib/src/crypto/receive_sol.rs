use dioxus::prelude::*;

#[component]
pub fn ReceiveSol() -> Element {
    rsx! {
        div {
            class: "receive-sol",
            h3 { "Receive SOL" }
            p { "Receive SOL functionality will be implemented here" }
        }
    }
} 