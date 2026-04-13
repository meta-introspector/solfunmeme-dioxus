use crate::extractor::{
    components::{
        appheader::ExtractorAppHeader, clearbutton::ClearButton, dropzone::DropZone,
        filedisplay::FileDisplay, fileinput::FileInput, progress::ProcessingIndicator,
        welcome::WelcomeMessage,
    },
    model::extract::process_file_engine,
    styles::STYLE,
    system::clipboard::{copy_all_snippets_combined, copy_to_clipboard},
    types::{CodeSnippet, ExtractedFile, ProcessingFile},
};
use dioxus::prelude::*;
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

    // Clipboard handlers
    let copy_to_clipboard = move |(text, snippet_id): (String, String)| {
        copy_to_clipboard(text, snippet_id, copied_snippets);
    };

    let copy_all_snippets = move |snippets: Vec<CodeSnippet>| {
        copy_all_snippets_combined(snippets, copied_snippets);
    };

    // File processing handlers
    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        process_file_engine(file_engine, files, processing_file).await; // also extract snippets
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };

    // Drag and drop handlers
    let on_drop = move |evt: DragEvent| {
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
                        copy_all_snippets,
                        copy_to_clipboard
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
