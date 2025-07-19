use std::error::Error;
use std::path::PathBuf;
use std::fs;
use crate::types::IndexStats;

pub fn export_stats(stats: &IndexStats, output_path: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let json_output = serde_json::to_string_pretty(stats)?;
    match output_path {
        Some(path) => {
            fs::write(path, json_output)?;
            println!("Exported statistics to JSON file: {:?}", path);
        },
        None => {
            println!("{}", json_output);
        }
    }
    Ok(())
}
