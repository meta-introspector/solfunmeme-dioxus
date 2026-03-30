use dioxus::{html::HasFileData, prelude::*};
use std::sync::Arc;
//use crate::extractor::Arc;
//use crate::extractor::FileEngine;
use crate::components::dropzone::dioxus_elements::FileEngine;
#[component]
pub fn DropZone(hovered: Signal<bool>, read_files: EventHandler<Arc<dyn FileEngine>>) -> Element {
    rsx! {
        div {
            class: if hovered() { "upload-area drag-over" } else { "upload-area" },
            ondragover: move |evt| {
                evt.prevent_default();
                hovered.set(true);
            },
            ondragleave: move |_| hovered.set(false),
            ondrop: move |evt| async move {
                evt.prevent_default();
                hovered.set(false);
                if let Some(file_engine) = evt.files() {
                    read_files.call(file_engine);
                }
            },
            "🎯 Drop markdown files here or click above to select"
        }
    }
}
