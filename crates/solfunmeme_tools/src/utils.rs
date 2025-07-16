use std::path::{Path, PathBuf};
use std::fs;
use regex::Regex;

/// Normalize paths for cross-platform compatibility
pub fn normalize_path(path: &Path) -> PathBuf {
    PathBuf::from(path.to_string_lossy().replace('\\', "/"))
}

/// Clean up filenames by removing problematic characters and normalizing
pub fn clean_filename(filename: &str) -> String {
    // Remove or replace problematic characters
    let cleaned = filename
        .replace("ΓÇª", "") // Remove encoding artifacts
        .replace("╖", "")   // Remove weird symbols
        .replace("∩╕Å", "") // Remove encoding artifacts
        .replace("₧í", "")  // Remove encoding artifacts
        .replace("≡ƒ", "")  // Remove encoding artifacts
        .replace("ô¥", "")  // Remove encoding artifacts
        .replace("º╣", "")  // Remove encoding artifacts
        .replace("9f9", "") // Remove encoding artifacts
        .replace("1f4dd", "") // Remove encoding artifacts
        .replace("27a1", "") // Remove encoding artifacts
        .replace("1f9f9", "") // Remove encoding artifacts
        .replace("(", "_")   // Replace parentheses with underscores
        .replace(")", "_")   // Replace parentheses with underscores
        .replace(" ", "_")   // Replace spaces with underscores
        .replace("__", "_")  // Replace double underscores with single
        .trim_matches('_')   // Remove leading/trailing underscores
        .to_string();
    
    // Ensure it's not empty
    if cleaned.is_empty() {
        "cleaned_file".to_string()
    } else {
        cleaned
    }
}

/// Create a safe output filename
pub fn create_output_filename(original_path: &Path, suffix: &str) -> String {
    let file_stem = original_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    
    let cleaned_stem = clean_filename(file_stem);
    format!("{}_{}.md", cleaned_stem, suffix)
}

/// Ensure directory exists, creating it if necessary
pub fn ensure_directory(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Find files with pattern matching and cleaning
pub fn find_files_with_pattern(
    target_path: &Path, 
    pattern: &str, 
    file_extension: &str,
    limit: Option<usize>
) -> Vec<PathBuf> {
    let normalized_path = normalize_path(target_path);
    let pattern_regex = Regex::new(pattern).unwrap_or_else(|_| Regex::new(".*").unwrap());
    
    let mut files: Vec<PathBuf> = walkdir::WalkDir::new(&normalized_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && 
            e.path().extension().map_or(false, |ext| ext == file_extension) &&
            e.path().file_name().map_or(false, |name| {
                pattern_regex.is_match(&name.to_string_lossy())
            })
        })
        .map(|e| normalize_path(e.path()))
        .collect();

    if let Some(limit) = limit {
        files.truncate(limit);
    }

    files
}

/// Save content to file with proper error handling
pub fn save_content_to_file(
    content: &str,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the parent directory exists
    if let Some(parent) = output_path.parent() {
        ensure_directory(parent)?;
    }
    
    // Write the content
    fs::write(output_path, content)?;
    Ok(())
} 