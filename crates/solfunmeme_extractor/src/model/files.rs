use crate::model::snippets::extract_markdown_snippets as extract_code_snippets_fn;
// use crate::model::extract_html::extract_code_snippets_from_html;
use solfunmeme_function_analysis::{ExtractedFile, ProcessingFile};
use dioxus::{
    html::FileEngine,
    signals::{Signal, Writable},
};
use gloo_timers::future::TimeoutFuture;
use std::{pin::Pin, sync::Arc};

async fn _process_file_engine_with_callbacks<F, P>(
    file_engine: Arc<dyn FileEngine>,
    mut on_file_start: F,
    mut on_progress: P,
) -> Vec<ExtractedFile>
where
    F: FnMut(&str, usize) + Send,
    P: FnMut(&str, usize, usize) + Send,
{
    let mut extracted_files = Vec::new();
    let file_names = file_engine.files();

    for file_name in &file_names {
        if let Some(content) = file_engine.read_file_to_string(file_name).await {
            let lines: Vec<&str> = content.lines().collect();
            let total_lines = lines.len();

            // Notify start of file processing
            on_file_start(file_name, total_lines);

            // Simulate progress updates
            for i in 0..=total_lines {
                on_progress(file_name, i, total_lines);
                if i % 100 == 0 || i == total_lines {
                    TimeoutFuture::new(10).await;
                }
            }
            let snippets = extract_code_snippets_fn(&content).unwrap();
            let all_snippets: Vec<solfunmeme_function_analysis::CodeChunk> = snippets.into_iter().map(|s| s.into()).collect();
            //            let html_code_snippets = extract_code_snippets_from_html(&content);
//            all_snippets.extend(html_code_snippets);
            // Finally push one combined ExtractedFile
            extracted_files.push(ExtractedFile {
                name: file_name.clone(),
                snippets: all_snippets.into_iter().map(|s| s.into()).collect(),
                total_lines,
            });
        }
    }

    extracted_files
}

// #[component]
// fn CodeExtractor() -> Element {
//     let mut files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
//     let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
//     let mut hovered = use_signal(|| false);
//     let mut copied_snippets = use_signal(|| std::collections::HashSet::new() as std::collections::HashSet<String>);

//     let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
//         process_files_with_signals(file_engine, files, processing_file).await;
//     };

//     // ... rest of component
// }

//## Process Files Function

pub async fn process_files(
    file_engine: Arc<dyn FileEngine>,
    mut files: Signal<Vec<ExtractedFile>>,
    mut processing_file: Signal<Option<ProcessingFile>>,
) {
    let file_names = file_engine.files();
    for file_name in &file_names {
        processing_file.set(Some(ProcessingFile {
            name: file_name.clone(),
            ..Default::default()
        }));

        TimeoutFuture::new(50).await;

        if let Some(content) = file_engine.read_file_to_string(file_name).await {
            let lines: Vec<&str> = content.lines().collect();
            let total_lines = lines.len();

            // Update processing status
            if let Some(pf) = processing_file.write().as_mut() {
                pf.total_lines = total_lines;
                pf.current_content = content.clone();
            }

            // Simulate progress for visual feedback
            for i in 0..=total_lines {
                if let Some(pf) = processing_file.write().as_mut() {
                    pf.progress = i;
                }
                if i % 100 == 0 || i == total_lines {
                    TimeoutFuture::new(10).await;
                }
            }

            let snippets = match extract_code_snippets_fn(&content) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error extracting markdown snippets from {}: {}", file_name, e);
                    Vec::new()
                }
            };
            files.write().push(ExtractedFile {
                name: file_name.clone(),
                snippets: snippets.into_iter().map(|s| s.into()).collect(),
                total_lines,
            });
        }
    }

    processing_file.set(None);
}

pub fn create_file_reader(
    mut processing_file: Signal<Option<ProcessingFile>>,
    mut files: Signal<Vec<ExtractedFile>>,
) -> impl Fn(Arc<dyn FileEngine>) -> Pin<Box<dyn std::future::Future<Output = ()>>> + Clone
//fn create_file_reader<T>(
//    mut    processing_file: Signal<Option<ProcessingFile>>,
//    mut files: Signal<Vec<ExtractedFile>>
//) -> impl Fn(Arc<dyn FileEngine>) -> Pin<Box<dyn std::future::Future<Output = ()>>> + Clone
{
    move |file_engine: Arc<dyn FileEngine>| {
        Box::pin(async move {
            let file_names = file_engine.files();

            for file_name in &file_names {
                processing_file.set(Some(ProcessingFile {
                    name: file_name.clone(),
                    ..Default::default()
                }));

                TimeoutFuture::new(50).await;

                if let Some(content) = file_engine.read_file_to_string(file_name).await {
                    let lines: Vec<&str> = content.lines().collect();
                    let total_lines = lines.len();

                    // Update processing status
                    if let Some(pf) = processing_file.write().as_mut() {
                        pf.total_lines = total_lines;
                        pf.current_content = content.clone();
                    }

                    // Simulate progress for visual feedback
                    for i in 0..=total_lines {
                        if let Some(pf) = processing_file.write().as_mut() {
                            pf.progress = i;
                        }
                        if i % 100 == 0 || i == total_lines {
                            TimeoutFuture::new(10).await;
                        }
                    }

                    let snippets = match extract_code_snippets_fn(&content) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Error extracting markdown snippets from {}: {}", file_name, e);
                            Vec::new()
                        }
                    };

                    files.write().push(ExtractedFile {
                        name: file_name.clone(),
                        snippets: snippets.into_iter().map(|s| s.into()).collect(),
                        total_lines,
                    });
                }
            }

            processing_file.set(None);
        })
    }
}

pub fn create_download_filename(language: &str) -> String {
    let extension = match language.to_lowercase().as_str() {
        "rust" | "rs" => "rs",
        "javascript" | "js" => "js",
        "typescript" | "ts" => "ts",
        "python" | "py" => "py",
        "java" => "java",
        "cpp" | "c++" => "cpp",
        "c" => "c",
        "html" => "html",
        "css" => "css",
        "json" => "json",
        "xml" => "xml",
        "yaml" | "yml" => "yml",
        "toml" => "toml",
        "go" => "go",
        "php" => "php",
        "ruby" | "rb" => "rb",
        "swift" => "swift",
        "kotlin" | "kt" => "kt",
        "scala" => "scala",
        "bash" | "sh" => "sh",
        "sql" => "sql",
        _ => "txt",
    };

    format!("{}_snippets.{}", language.to_lowercase(), extension)
}

// pub fn get_unique_languages(snippets: &[CodeSnippets]) -> Vec<String> {
//     snippets.iter()
//         .map(|s| s.language.clone())
//         .collect::<std::collections::HashSet<_>>()
//         .into_iter()
//         .collect()
// }
//}