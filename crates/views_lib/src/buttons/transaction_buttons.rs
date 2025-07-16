use dioxus::prelude::*;

#[component]
pub fn TransactionButtons() -> Element {
    rsx! {
        div {
            class: "transaction-buttons",
            h3 { "Transaction Buttons" }
            p { "Transaction button components will be implemented here" }
        }
    }
} 