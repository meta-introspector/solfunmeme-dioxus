use walkdir::WalkDir;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

pub fn read_code_chunks(target_path: Option<String>, limit: Option<usize>) -> Result<Vec<(PathBuf, String)>> {
    let mut discovered_files = Vec::new();
    if let Some(ref path) = target_path {
        let path = PathBuf::from(path);
        if path.is_file() {
            discovered_files.push(path);
        } else if path.is_dir() {
            for entry in WalkDir::new(&path).into_iter().filter_map(Result::ok) {
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

    let mut file_contents = Vec::new();
    for path in discovered_files {
        match fs::read_to_string(&path) {
            Ok(content) => {
                file_contents.push((path, content));
            },
            Err(e) => {
                eprintln!("[ERROR] Failed to read file {} (possibly non-UTF-8 or binary): {}", path.display(), e);
            }
        }
    }
    Ok(file_contents)
}