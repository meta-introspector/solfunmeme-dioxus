use std::error::Error;
use std::path::PathBuf;
use std::fs;
use crate::types::IndexedDocument;

pub fn export_to_json(documents: &[IndexedDocument], output_path: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let json_output = serde_json::to_string_pretty(documents)?;
    match output_path {
        Some(path) => {
            fs::write(path, json_output)?;
            println!("Exported {} documents to JSON file: {:?}", documents.len(), path);
        },
        None => {
            println!("{}", json_output);
        }
    }
    Ok(())
}
