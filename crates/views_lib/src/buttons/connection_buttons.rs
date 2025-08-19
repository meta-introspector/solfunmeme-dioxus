use dioxus::prelude::*;

#[component]
pub fn ConnectionButtons() -> Element {
    rsx! {
        div {
            class: "connection-buttons",
            h3 { "Connection Buttons" }
            p { "Connection button components will be implemented here" }
        }
    }
} 