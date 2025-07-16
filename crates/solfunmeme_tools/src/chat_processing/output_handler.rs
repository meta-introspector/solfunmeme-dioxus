use std::path::{Path, PathBuf};
use crate::utils;

pub fn save_processed_content(
    processed_content: &str,
    original_file_path: &Path,
    output_dir: &PathBuf,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Normalize output directory
    let normalized_output_dir = utils::normalize_path(output_dir);
    
    // Create output filename with cleaned name
    let output_filename = utils::create_output_filename(original_file_path, "processed");
    let output_path = normalized_output_dir.join(output_filename);

    // Save the content
    utils::save_content_to_file(processed_content, &output_path)?;
    
    Ok(output_path)
}
