use std::path::Path;
use walkdir::WalkDir;

pub fn find_rust_files(project_root: &Path) -> Vec<String> {
    let mut rust_files = Vec::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "rs" {
                    rust_files.push(entry.path().to_string_lossy().into_owned());
                }
            }
        }
    }
    rust_files
}
