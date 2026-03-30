use solfunmeme_tools::doc_test_generation::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = DocTestGeneratorArgs::from_env()?;
    
    println!("📚 Documentation Test Generator");
    println!("Generating tests from {}...", args.input_file.display());
    
    let readme_content = fs::read_to_string(&args.input_file)?;
    let examples = extract_code_examples(&readme_content)?;
    
    generate_doc_tests(&examples, &args.output_dir)?;
    
    println!("✅ Generated {} documentation tests in {}", examples.len(), args.output_dir.display());
    Ok(())
}
