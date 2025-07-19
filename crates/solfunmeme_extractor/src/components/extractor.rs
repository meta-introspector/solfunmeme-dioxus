use crate::{
    components::{
        appheader::ExtractorAppHeader, clearbutton::ClearButton, dropzone::DropZone,
        filedisplay::FileDisplay, fileinput::FileInput, progress::ProcessingIndicator,
        welcome::WelcomeMessage,
    },
    model::files::process_files,
    styles::STYLE,
};
use solfunmeme_function_analysis::{ExtractedFile, ProcessingFile};
use dioxus::prelude::*;
use dioxus::html::FileEngine;
use dioxus_html::HasFileData;
use std::sync::Arc;

//src/playground/app.rs
// 37:use crate::extractor::components::extractor::MarkdownCodeExtractor;
//174:		    MenuOption::Extractor => rsx!(MarkdownCodeExtractor {}),

#[component]
pub fn MarkdownCodeExtractor() -> Element {
    let files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
    let processing_file = use_signal::<Option<ProcessingFile>>(|| None);
    let hovered = use_signal(|| false);
    let copied_snippets =
        use_signal(|| std::collections::HashSet::new() as std::collections::HashSet<String>);

    

    // File processing handlers
    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        process_files(file_engine, files, processing_file).await;
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };

    // Drag and drop handlers
    let _on_drop = move |evt: DragEvent| {
        let read_files_clone = read_files.clone();
        spawn(async move {
            if let Some(file_engine) = evt.files() {
                read_files_clone(file_engine).await;
            }
        });
    };

    files.with(|files_vec| {
        rsx! {
            style { "{STYLE}" }

            div { class: "code-extractor",
                ExtractorAppHeader {}

                ClearButton {
                    files: files,
                    copied_snippets: copied_snippets
                }

                FileInput {
                    upload_files: upload_files
                }

                DropZone {
                    hovered: hovered,
                    read_files: read_files
                }

                ProcessingIndicator {
                    processing_file: processing_file()
                }

                // Results section
                for file in files_vec.iter() {
                    FileDisplay {
                        file: file.clone(),
                        copied_snippets,
                    }
                }

                // Welcome message when no files are loaded
                if files_vec.is_empty() && processing_file().is_none() {
                    WelcomeMessage { show: true }
                }
            }
        }
    })
}