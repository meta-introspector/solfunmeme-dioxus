use dioxus::prelude::*;

#[component]
pub fn WorkflowMemes() -> Element {
    rsx! {
        div {
            class: "workflow-memes",
            h3 { "Workflow Memes" }
            p { "Workflow memes functionality will be implemented here" }
        }
    }
} 