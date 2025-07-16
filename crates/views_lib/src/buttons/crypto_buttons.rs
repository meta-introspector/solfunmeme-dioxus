use dioxus::prelude::*;

#[component]
pub fn CryptoButtons() -> Element {
    rsx! {
        div {
            class: "crypto-buttons",
            h3 { "Crypto Buttons" }
            p { "Crypto button components will be implemented here" }
        }
    }
} 