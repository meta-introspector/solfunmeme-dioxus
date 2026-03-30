use core::io;
use std::path::Path;

/// Saves the processed document summary to a JSON file.
pub fn save_summary(summary: &DocumentSummary, output_path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(summary)?;
    fs::write(output_path, json)?;
    Ok(())
}

/// Main function to process documents from command line
pub fn savemain() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        eprintln!("Example: {} founding_documents/prelude1-aaa.md output/summary.json", args[0]);
        std::process::exit(1);
    }
    
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);
    
    if !input_path.exists() {
        eprintln!("Error: Input file '{}' does not exist", args[1]);
        std::process::exit(1);
    }
    
    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    println!("Processing document: {}", input_path.display());
    let summary = process_file(input_path)?;
    
    println!("Saving summary to: {}", output_path.display());
    save_summary(&summary, output_path)?;
    
    println!("Document Summary:");
    println!("  Total turns: {}", summary.total_turns);
    println!("  Total code snippets: {}", summary.total_code_snippets);
    println!("  Total tokens: {}", summary.total_tokens);
    println!("  Languages found: {:?}", summary.languages_found);
    println!("  Content hashes: {}", summary.content_hashes.len());
    
    Ok(())
}
