use crate::model::lean::style::Styles;
use dioxus::prelude::*;

#[component]
pub fn Encryption() -> Element {
    rsx! {
        div {
            class: "{Styles::section()}",
            h2 { class: "{Styles::h2()}", "Encryption" }
            p { class: "{Styles::p()}", "This section demonstrates encryption functionality." }
        }
    }
}
