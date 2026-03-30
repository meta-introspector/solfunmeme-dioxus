use dioxus::{html::HasFileData, prelude::*};
#[component]
pub fn FileUploadArea(
    hovered: bool,
    on_hover: EventHandler<bool>,
    on_upload: EventHandler<FormEvent>,
    on_drop: EventHandler<DragEvent>,
) -> Element {
    rsx! {
        div { class: "file-input",
            label { r#for: "file-upload", "📁 Select Markdown Files:" }
            input {
                id: "file-upload",
                r#type: "file",
                accept: ".md,.txt",
                multiple: true,
                onchange: on_upload,
            }
        }
        div {
            class: if hovered { "upload-area drag-over" } else { "upload-area" },
            ondragover: move |evt| {
                evt.prevent_default();
                on_hover(true);
            },
            ondragleave: move |_| on_hover(false),
            ondrop: move |evt| async move {
                evt.prevent_default();
                on_hover(false);
                if let Some(_file_engine) = evt.files() {
                    on_drop(evt);
            //FormEvent::new(evt.data().clone(),true)).await;
                }
            },
            "🎯 Drop markdown files here or click above to select"
        }
    }
}
