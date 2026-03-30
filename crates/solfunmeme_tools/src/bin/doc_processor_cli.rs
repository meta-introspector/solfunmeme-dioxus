use solfunmeme_tools::doc_processing::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = DocProcessorArgs::from_env()?;
    
    let doc_files = find_doc_files(&args.target_path, args.limit, &args.include_pattern);
    println!("[INFO] Processing {} document files:", doc_files.len());

    for doc_file in doc_files {
        println!("[INFO] Processing file: {}", doc_file.display());
        
        let content = fs::read_to_string(&doc_file)?;
        let processed_content = process_document(&content);

        let output_path = save_processed_document(&processed_content, &doc_file, &args.output_dir)?;
        
        println!("[INFO] Saved processed file to: {}", output_path.display());
    }

    Ok(())
}
