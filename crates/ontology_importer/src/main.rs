use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let index_ttl_path = Path::new("/data/data/com.termux/files/home/storage/solfunmeme-dioxus/ontologies/index.ttl");
    let hf_import_statement = "owl:imports <http://example.org/huggingface#> .";

    let mut content = String::new();
    fs::File::open(&index_ttl_path)?.read_to_string(&mut content)?;

    // Check if the import statement already exists to avoid duplication
    if !content.contains(hf_import_statement) {
        let mut new_lines: Vec<String> = Vec::new();
        let mut inserted = false;

        for line in content.lines() {
            new_lines.push(line.to_string());
            // Simple heuristic: insert after the last existing owl:imports statement
            if line.trim().starts_with("owl:imports") {
                // Check if this is the last import line before a non-import line or end of file
                let next_line_idx = new_lines.len();
                if next_line_idx < content.lines().count() && !content.lines().nth(next_line_idx).unwrap().trim().starts_with("owl:imports") {
                    new_lines.push(hf_import_statement.to_string());
                    inserted = true;
                }
            }
        }

        // Fallback: if not inserted (e.g., no existing imports or logic missed it), append at the end of the import block section
        if !inserted {
            let mut final_content_lines = Vec::new();
            let mut in_import_block = false;
            for line in content.lines() {
                final_content_lines.push(line.to_string());
                if line.trim().starts_with("# Import individual crate ontologies") {
                    in_import_block = true;
                } else if in_import_block && !line.trim().starts_with("owl:imports") && !line.trim().is_empty() {
                    // Found end of import block
                    final_content_lines.push(hf_import_statement.to_string());
                    in_import_block = false;
                }
            }
            if in_import_block { // If import block extends to end of file
                final_content_lines.push(hf_import_statement.to_string());
            }
            content = final_content_lines.join("\n");
        } else {
            content = new_lines.join("\n");
        }

        fs::File::create(&index_ttl_path)?.write_all(content.as_bytes())?;
        println!("Successfully added {} to {}", hf_import_statement, index_ttl_path.display());
    } else {
        println!("{} already exists in {}", hf_import_statement, index_ttl_path.display());
    }

    Ok(())
}
