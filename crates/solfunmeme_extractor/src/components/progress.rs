use dioxus::prelude::*;

use solfunmeme_function_analysis::ProcessingFile;
//use crate::extractor::ProcessingFile;
#[component]
pub fn ProcessingIndicator(processing_file: ReadOnlySignal<Option<ProcessingFile>>) -> Element {
    if let Some(pf) = (*processing_file)() {
        rsx! {
            div { class: "processing-indicator",
                h3 { "🔄 Processing: {pf.name}" }
                if pf.total_lines > 0 {
                    div {
                        "Progress: {pf.progress} / {pf.total_lines} lines ({((pf.progress as f32 / pf.total_lines as f32) * 100.0) as i32}%)"
                    }
                    div { class: "progress-bar",
                        div {
                            class: "progress-fill",
                            style: "width: {((pf.progress as f32 / pf.total_lines as f32) * 100.0)}%"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}

// #[component]
// pub fn ProcessingIndicator(processing_file: Option<ProcessingFile>) -> Element {
//         if let Some(pf) = processing_file {
//             rsx! {
//                 div { class: "processing-indicator",
//                       h3 { "🔄 Processing: {pf.name}" }
//                       if pf.total_lines > 0 {
//                         div {
//                             "Progress: {pf.progress} / {pf.total_lines} lines ({((pf.progress as f32 / pf.total_lines as f32) * 100.0) as i32}%)"
//                         }
//                         div { class: "progress-bar",
//                             div {
//                                 class: "progress-fill",
//                                 style: "width: {((pf.progress as f32 / pf.total_lines as f32) * 100.0)}%"
//                             }
//                         }
//                     }
//                 }
//             }
//         } else {
//             rsx! { div {} }
//         }
//     }
