use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;
use solfunmeme_function_analysis::AnalyzedFunction;
use solfunmeme_language_processing::{LanguageProcessor, rust_processor::RustProcessor};
use indicatif::ProgressBar;

pub fn analyze_project(project_root: &Path, _ontology_path: &Path, _use_gpu: bool, pb: &ProgressBar) -> Result<Vec<AnalyzedFunction>> {
    let mut analyzed_functions = Vec::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            if ext == "rs" {
                let processor = RustProcessor;
                let content = std::fs::read_to_string(file_path)?;
                let chunks = processor.process_code(&content, &file_path.to_string_lossy())?;
                for chunk in chunks {
                    analyzed_functions.push(AnalyzedFunction {
                        function_name: "unknown".to_string(), // Placeholder
                        code_snippet: chunk.content,
                        semantic_summary: chunk.semantic_summary.unwrap_or_default(),
                        file_path: file_path.to_string_lossy().replace("\\", "/").to_owned(),
                        orbital_path: None,
                    });
                }
            }
            pb.inc(entry.metadata()?.len());
        }
    }
    Ok(analyzed_functions)
}
