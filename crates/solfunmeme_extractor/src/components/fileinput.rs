use dioxus::prelude::*;
#[component]
pub fn FileInput(upload_files: EventHandler<FormEvent>) -> Element {
    rsx! {
        div { class: "file-input",
            label { r#for: "file-upload", "📁 Select Markdown Files:" }
            input {
                id: "file-upload",
                r#type: "file",
                accept: ".md,.txt",
                multiple: true,
                onchange: upload_files,
            }
        }
    }
}
