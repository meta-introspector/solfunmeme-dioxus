use anyhow::Result;
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use solfunmeme_function_analysis::AnalyzedDocument;
use solfunmeme_language_processing::{LanguageProcessor, markdown_processor::MarkdownProcessor};
use indicatif::ProgressBar;

pub fn analyze_markdown_files(project_root: &Path, _ontology_path: &Path, _use_gpu: bool, pb: &ProgressBar) -> Result<Vec<AnalyzedDocument>> {
    let mut analyzed_documents = Vec::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            if ext == "md" || ext == "markdown" {
                let processor = MarkdownProcessor;
                let content = fs::read_to_string(file_path)?;
                let code_snippets = processor.process_code(&content, &file_path.to_string_lossy())?;
                let text_chunks = Vec::new();
                // Simplified text chunk extraction for now. In a full implementation,
                // the MarkdownProcessor would ideally return both code and text chunks.
                analyzed_documents.push(AnalyzedDocument {
                    file_path: file_path.to_string_lossy().into_owned(),
                    code_snippets: code_snippets.clone(),
                    text_chunks,
                    analyzed_snippets: code_snippets,
                });
            }
            pb.inc(entry.metadata()?.len());
        }
    }
    Ok(analyzed_documents)
}
