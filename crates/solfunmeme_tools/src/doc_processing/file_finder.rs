use std::path::PathBuf;
use walkdir::WalkDir;
use glob::Pattern;

pub fn find_doc_files(target_path: &PathBuf, limit: Option<usize>, include_pattern: &Option<String>) -> Vec<PathBuf> {
    let mut doc_files: Vec<PathBuf> = WalkDir::new(target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && 
            if let Some(pattern_str) = include_pattern {
                if let Ok(pattern) = Pattern::new(pattern_str) {
                    pattern.matches_path(e.path())
                } else {
                    false
                }
            } else {
                true // No include pattern, so include all files
            }
        })
        .map(|e| e.path().to_owned())
        .collect();

    if let Some(limit) = limit {
        doc_files.truncate(limit);
    }

    doc_files
}
