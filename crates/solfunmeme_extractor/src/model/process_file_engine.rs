use dioxus::{html::FileEngine, prelude::*};
use gloo_timers::future::TimeoutFuture;
use std::sync::Arc;
use crate::types::{ExtractedFile, ProcessingFile, DocumentSummary};
use crate::model::extract::extract_code_snippets;
use crate::model::extract_html::extract_code_snippets_from_html;

/// Process files from FileEngine and extract code snippets
pub async fn process_file_engine(
    file_engine: Arc<dyn FileEngine>,
    mut files: Signal<Vec<ExtractedFile>>,
    mut processing_file: Signal<Option<ProcessingFile>>,
) {
    let file_names = file_engine.files();

    for file_name in &file_names {
        let summary = Some(DocumentSummary {
            total_turns: 9,
            total_code_snippets: 0,
            total_tokens: 0,
            languages_found: [].to_vec(),
            content_hashes: [].to_vec(),
        });
        // Start processing this file
        processing_file.set(Some(ProcessingFile {
            name: file_name.clone(),
            progress: 0,
            total_lines: 0,
            current_content: String::new(),
            summary,
        }));

        // Small delay for UI responsiveness
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
            let progress_steps = (total_lines / 100).max(1);
            for i in (0..=total_lines).step_by(progress_steps) {
                if let Some(pf) = processing_file.write().as_mut() {
                    pf.progress = i.min(total_lines);
                }
                TimeoutFuture::new(10).await;
            }

            // Extract code snippets
            // FIXME why are we doing the same thing twice?
            let snippets = extract_code_snippets::extract_code_snippets(&content);
            // …then append any HTML‐based snippets
            let mut all_snippets = snippets;
            all_snippets.extend(extract_code_snippets_from_html(&content));
            // Finally push one combined ExtractedFile
            files.write().push(ExtractedFile {
                name: file_name.clone(),
                snippets: all_snippets,
                total_lines,
            });
        }
    }

    // Clear processing state
    processing_file.set(None);
}
