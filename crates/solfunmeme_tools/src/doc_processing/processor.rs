use std::fs;
use std::path::PathBuf;
use crate::utils;

pub fn process_documentation_files(target_path: &PathBuf, output_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Find all markdown files
    let md_files = utils::find_files_with_pattern(target_path, ".*", "md", None);
    
    println!("[INFO] Found {} markdown files to process", md_files.len());
    
    for file_path in md_files {
        println!("[INFO] Processing: {}", file_path.display());
        
        // Read the file content
        let content = fs::read_to_string(&file_path)?;
        
        // Clean the content (remove HTML tags, normalize whitespace)
        let cleaned_content = process_document(&content);
        
        // Create output filename
        let output_filename = utils::create_output_filename(&file_path, "cleaned");
        let output_path = output_dir.join(output_filename);
        
        // Save the cleaned content
        utils::save_content_to_file(&cleaned_content, &output_path)?;
        
        println!("[INFO] Saved cleaned file to: {}", output_path.display());
    }
    
    Ok(())
}

pub fn process_document(content: &str) -> String {
    // Remove HTML tags
    let html_cleaned = content
        .lines()
        .map(|line| {
            // Simple HTML tag removal
            let mut cleaned = line.to_string();
            while let Some(start) = cleaned.find('<') {
                if let Some(end) = cleaned[start..].find('>') {
                    cleaned.remove(start);
                    cleaned.remove(start + end - 1);
                } else {
                    break;
                }
            }
            cleaned
        })
        .collect::<Vec<_>>()
        .join("\n");
    
    // Normalize whitespace
    html_cleaned
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}