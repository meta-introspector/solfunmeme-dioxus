// used as embedding app MenuOption::Embedding => rsx!(EmbeddingApp {}) from src/playground/app.rs
//
use dioxus::{html::HasFileData, prelude::*};
use gloo_timers::future::TimeoutFuture;
use std::sync::Arc;

use crate::styles::STYLE;
use dioxus::html::FileEngine;
//use crate::extractor::error;
//use crate::extractor::ProcessingFile;
use solfunmeme_function_analysis::{ProcessingFile, UploadedFile, DocumentSummary, AnnotatedWord, CodeChunk};
// use solfunmeme_embedding::embed_text;
use solfunmeme_clifford::{SolMultivector, SerializableMultivector};
// use candle_core::Device;

async fn read_files(
    file_engine: Arc<dyn FileEngine>,
    currently_processing_file: &mut Signal<Option<ProcessingFile>>,
    files_uploaded: &mut Signal<Vec<UploadedFile>>,
) {
    let files = file_engine.files();
    for file_name in &files {
        currently_processing_file.set(Some(ProcessingFile {
            name: file_name.clone(),
            ..Default::default()
        }));
        TimeoutFuture::new(1).await;

        if let Some(contents) = file_engine.read_file_to_string(file_name).await {
            let lines: Vec<&str> = contents.lines().collect();
            let total_lines = lines.len();

            if let Some(p) = currently_processing_file.write().as_mut() {
                p.total_lines = total_lines;
            }

            let _snippet: Vec<CodeChunk> = [].to_vec();
            let _code_annotations: Vec<AnnotatedWord> = [].to_vec();

            // FIXME: broken code, please fix

            // if file_name.ends_with(".md") {
            //     let snippets = extract_code_snippets(&contents);
            //     for snippet in snippets {
            //         //test_code_snippet(snippet);
            //         //code_annotations.push(annotate_code_snippet(snippet));
            // 	//snippets.push(snippet);
            //     }

            // }
            //Err(e) => error!("Markdown parsing error: {}", e),
            //}
            //}

            for (i, line) in lines.iter().enumerate() {
                let _words = line.split_whitespace().collect::<Vec<_>>();
                if let Some(p) = currently_processing_file.write().as_mut() {
                    //let line_annotations = words.iter().map(|&w| annotate_word(w)).collect::<Vec<_>>();
                    //p.annotations.extend(line_annotations);
                    p.current_content.push_str(line);
                    p.current_content.push('\n');
                    p.progress = i + 1;
                }
                if i % 10 == 0 || i == total_lines - 1 {
                    TimeoutFuture::new(1).await;
                }
            }

            let summary = DocumentSummary {
                total_turns: 1,
                total_code_snippets: 0, //snippets.len(),
                total_tokens: 0,
                // FIXME
                //                    total_tokens: snippets.iter().map(|s| s.token_count).sum::<usize>(),
                //                    languages_found: snippets.iter().map(|s| s.language.clone()).collect::<std::collections::HashSet<_>>().into_iter().collect(),
                //                    content_hashes: snippets.iter().map(|s| s.content_hash.clone()).collect(),
                content_hashes: [].to_vec(),
                languages_found: [].to_vec(),
            };

            //let duplicate_report = check_duplicates(snippets.clone());
            //                let zip_url = create_zip(&duplicate_report.unique_snippets).unwrap_or_else(|e| {
            //                    error!("ZIP creation failed: {:?}", e);
            //                    String::new()
            //                });

            if let Some(mut p_file) = currently_processing_file.take() {
                p_file.summary = Some(summary);
                //p_file.duplicate_report = Some(duplicate_report);
                //p_file.code_annotations = code_annotations;
                let generated_program = "FIXME"; //generate_program(&p_file.annotations);

                files_uploaded.write().push(UploadedFile {
                    name: file_name.clone(),
                    contents: contents.clone(),
                    //annotations: p_file.annotations,
                    generated_program: generated_program.to_string(),
                    summary: p_file.summary,
                    //duplicate_report: p_file.duplicate_report,
                    zip_url: Some("FIXME".to_string()),
                    //                        zip_url: if zip_url.is_empty() { None } else { Some(zip_url) },
                    //code_annotations: p_file.code_annotations,
                });
            }
        }
    } // for filename
}

async fn upload_files(
    evt: FormEvent,
    currently_processing_file: &mut Signal<Option<ProcessingFile>>,
    files_uploaded: &mut Signal<Vec<UploadedFile>>,
) {
    if let Some(file_engine) = evt.files() {
        read_files(file_engine, currently_processing_file, files_uploaded).await;
    }
}

// #[mcp_component(
//      menu = "core",
//      label = "Embedding App",
//      emoji = "🔗",
//      description = "Manage Embeddings",
//      visible = true,
//      order = 1
//  )]
pub fn embedding_app() -> Element {
    let mut enable_directory_upload = use_signal(|| false);
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<UploadedFile>);
    let mut hovered = use_signal(|| false);
    let mut currently_processing_file = use_signal::<Option<ProcessingFile>>(|| None);
    let mut enable_wikidata = use_signal(|| false);
    let mut enable_sentiment = use_signal(|| false);
    let mut enable_embedding = use_signal(|| false);
    //let wikidata_data = use_signal::<Option<Value>>(|| None);

    //    use_effect(move || {
    //        if *enable_wikidata.read() {
    //            spawn(async move {
    //                match fetch_wikidata_graph().await {
    //                    Ok(data) => wikidata_data.set(Some(data)),
    //                    Err(e) => error!("Wikidata fetch failed: {}", e),
    //                }
    //            });
    //        }
    //    });

    //    let mut annotator = WikidataAnnotator::new();

    let _annotate_word = move |word: &str| -> AnnotatedWord {
        let embedding = vec![0.0; 384];
        /*if *enable_embedding.read() {
            match embed_text(word, &device) {
                Ok(emb) => emb,
                Err(_) => vec![0.0; 384],
            }
        } else {
            vec![0.0; 384]
        };*/

        let mut coeffs = [0.0; 8];
        for i in 0..std::cmp::min(embedding.len(), 8) {
            coeffs[i] = embedding[i];
        }
        let clifford_vector = Some(SerializableMultivector(SolMultivector::from_vector(coeffs.iter().cloned()).unwrap()));

        AnnotatedWord {
            word: word.to_string(),
            primary_emoji: "🌟".to_string(),
            secondary_emoji: "✨".to_string(),
            wikidata: None,
            embedding,
            clifford_vector,
        }
    };

    //     let generate_program = |annotations: &[AnnotatedWord]| -> String {
    //         let struct_defs = r#"
    // #[derive(Debug)]
    // struct Annotation {
    //     word: String,
    //     primary_emoji: String,
    //     secondary_emoji: String,
    // }
    // "#;
    //         let data = annotations.iter().map(|anno| format!(
    //             r#"Annotation {{
    //                 word: "{}".to_string(),
    //                 primary_emoji: "{}".to_string(),
    //                 secondary_emoji: "{}".to_string(),
    //             }}"#,
    //             anno.word, anno.primary_emoji, anno.secondary_emoji
    //         )).collect::<Vec<_>>().join(",\n    ");
    //         format!(
    //             //"{}\nfn main() {{\n    let annotations = vec![{}];\n    for anno in annotations {{\n        println!(\"{:?}\", anno);\n    }}\n}}",
    // 	    "debug {:?}  {:?} ",
    //             struct_defs, data
    //         )
    //     };

    // read files

    rsx! {
        style { "{STYLE}" }
        h1 { "Semantic Hyperspace File Upload" }
        p { "Upload a .md file (e.g., exported Grok chat) to extract and analyze code snippets" }
        button { onclick: move |_| files_uploaded.write().clear(), "Clear files" }

        div {
            h3 { "Processing Pipeline" }
            div {
                label { r#for: "wikidata", "Enable Wikidata Annotation (FIXME not working yet)" }
                input {
                    r#type: "checkbox",
                    id: "wikidata",
                    checked: enable_wikidata,
                    oninput: move |evt| enable_wikidata.set(evt.checked()),
                }
            }
            div {
                label { r#for: "sentiment", "Enable Sentiment Analysis (FIXME not working yet)" }
                input {
                    r#type: "checkbox",
                    id: "sentiment",
                    checked: enable_sentiment,
                    oninput: move |evt| enable_sentiment.set(evt.checked()),
                }
            }
            div {
                label { r#for: "embedding", "Enable BERT Embedding (FIXME not working yet)" }
                input {
                    r#type: "checkbox",
                    id: "embedding",
                    checked: enable_embedding,
                    oninput: move |evt| enable_embedding.set(evt.checked()),
                }
            }
        }

        div {
            label { r#for: "directory-upload", "Enable directory upload" }
            input {
                r#type: "checkbox",
                id: "directory-upload",
                checked: enable_directory_upload,
                oninput: move |evt| enable_directory_upload.set(evt.checked()),
            }
        }

        div {
            label { r#for: "textreader", "Upload markdown or text files" }
            input {
                r#type: "file",
                accept: ".md,.txt",
                multiple: true,
                name: "textreader",
                directory: enable_directory_upload,
                onchange: move |evt| {
                    let mut currently_processing_file = currently_processing_file.clone();
                    let mut files_uploaded = files_uploaded.clone();
                    spawn(async move {
                        upload_files(evt, &mut currently_processing_file, &mut files_uploaded).await;
                    });
                },
            }
        }

        div {
            id: "drop-zone",
            background_color: if hovered() { "lightblue" } else { "lightgray" },
            ondragover: move |evt| {
                evt.prevent_default();
                hovered.set(true)
            },
            ondragleave: move |_| hovered.set(false),
            ondrop: move |evt| async move {
                evt.prevent_default();
                hovered.set(false);
                if let Some(file_engine) = evt.files() {
                    read_files(file_engine, &mut currently_processing_file, &mut files_uploaded).await;
                }
            },
            "Drop files here"
        }

        if let Some(file) = currently_processing_file() {
            div {
                style: "border: 2px solid #007bff; padding: 15px; margin: 10px 0; border-radius: 5px; background-color: #f8f9fa;",
                h3 { "🔄 Processing: {file.name}" }
                if file.total_lines > 0 {
                    div {
                        div {
                            style: "margin: 10px 0;",
                            "Progress: {file.progress} / {file.total_lines} lines ({((file.progress as f32 / file.total_lines as f32) * 100.0) as i32}%)"
                        }
                        div {
                            style: "width: 100%; background-color: #e9ecef; border-radius: 10px; overflow: hidden;",
                            div {
                                style: "width: {((file.progress as f32 / file.total_lines as f32) * 100.0) as i32}%; height: 20px; background-color: #007bff; transition: width 0.3s ease;",
                            }
                        }
                    }
                }
                div {
                    style: "max-height: 200px; overflow-y: auto; border: 1px solid #dee2e6; padding: 10px; background-color: white;",
                    pre { "{file.current_content}" }
                }
            }
        }

        ul {
            for file in files_uploaded.read().iter().rev() {
                li {
                    span { "{file.name}" }
                    pre { "{file.contents}" }
                    if let Some(summary) = &file.summary {
                        div {
                            h3 { "Document Summary" }
                            p { "Total code snippets: {summary.total_code_snippets}" }
                            p { "Total tokens: {summary.total_tokens}" }
                            p { "Languages: {summary.languages_found.join(\", \")}" }
                        }
                    }
                    // if let Some(report) = &file.duplicate_report {
                    //     div {
                    //         h3 { "Duplicate Report" }
                    //         p { "Unique snippets: {report.unique_snippets.len()}" }
                    //         if !report.duplicates.is_empty() {
                    //             ul {
                    //                 for (dup, indices) in &report.duplicates {
                    //                     li { "Duplicate snippet (at indices {indices:?}): {dup.content}" }
                    //                 }
                    //             }
                    //         } else {
                    //             p { "No duplicates found." }
                    //         }
                    //     }
                    // }
                    // if !file.code_annotations.is_empty() {
                    //     div {
                    //         h3 { "Code Snippet Annotations" }
                    //         ul {
                    //             for anno in file.code_annotations.iter() {
                    //                 li {
                    //                     span { "{anno.word} {anno.primary_emoji}{anno.secondary_emoji}" }
                    //                     span { " Sentiment: {anno.sentiment:.2}" }
                    //                     if let Some(wd) = &anno.wikidata {
                    //                         span { " | Wikidata: {wd}" }
                    //                     }
                    //                     span { " | Vector: [{anno.multivector.vector[0]:.2}, {anno.multivector.vector[1]:.2}, {anno.multivector.vector[2]:.2}]" }
                    //                 }
                    //             }
                    //         }
                    //     }
                    // }
                    if let Some(zip_url) = &file.zip_url {
                        a {
                            href: "{zip_url}",
                            download: "{file.name}.zip",
                            "Download Unique Code Snippets as ZIP"
                        }
                    }
                    h3 { "Generated Program" }
                    pre { "{file.generated_program}" }
                }
            }
        }
    }
}
