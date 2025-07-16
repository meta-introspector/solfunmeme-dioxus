use walkdir::WalkDir;
use std::fs;
use std::path::Path;
use anyhow::Result;
use solfunmeme_function_analysis::CodeChunk;

pub fn read_code_chunks(target_path: Option<String>, limit: Option<usize>) -> Result<Vec<CodeChunk>> {
    let mut discovered_files = Vec::new();
    if let Some(ref path) = target_path {
        let path = Path::new(path);
        if path.is_file() {
            discovered_files.push(path.to_path_buf());
        } else if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() {
                    discovered_files.push(entry.path().to_path_buf());
                }
            }
        } else {
            anyhow::bail!("Path not found: {}", path.display());
        }
    } else {
        for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                discovered_files.push(entry.path().to_path_buf());
            }
        }
    }
    if let Some(lim) = limit {
        discovered_files.truncate(lim);
    }

    let mut code_chunks = Vec::new();
    for path in discovered_files {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        if !matches!(ext.as_str(), "rs" | "md") {
            // Only process .rs and .md files for now
            continue;
        }
        match fs::read(&path) {
            Ok(bytes) => {
                let content = String::from_utf8_lossy(&bytes).to_string();
                let chunk = CodeChunk {
                    language: ext.clone(), // Use file extension as language
                    content: content.clone(),
                    line_start: 1,
                    line_end: content.lines().count(),
                    content_hash: format!("{:x}", md5::compute(&content)), // Placeholder hash
                    token_count: content.split_whitespace().count(), // Placeholder token count
                    line_count: content.lines().count(),
                    char_count: content.chars().count(),
                    test_result: "Untested".to_string(), // Placeholder test result
                };
                code_chunks.push(chunk);
            },
            Err(e) => {
                eprintln!("[ERROR] Failed to read file {} (possibly non-UTF-8 or binary): {}", path.display(), e);
            }
        }
    }
    Ok(code_chunks)
}