use dioxus::prelude::*;

#[component]
pub fn Clusters() -> Element {
    rsx! {
        div {
            class: "clusters",
            h3 { "Clusters" }
            p { "Clusters functionality will be implemented here" }
        }
    }
} 