use std::fs;
use std::path::{Path, PathBuf};

pub fn save_processed_document(
    processed_content: &str,
    original_file_path: &Path,
    output_dir: &PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    // Create output file path
    let file_name = original_file_path.file_name().unwrap().to_string_lossy();
    let output_name = format!("{}_processed.md", file_name.trim_end_matches(".md"));
    let output_path = output_dir.join(output_name);

    // Write processed content
    fs::write(&output_path, processed_content)?;
    
    Ok(output_path)
}
