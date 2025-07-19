use std::error::Error;
use std::path::PathBuf;
use std::fs;
use crate::types::IndexedDocument;

pub fn export_to_csv(documents: &[IndexedDocument], output_path: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut csv_output = String::new();
    // CSV header
    csv_output.push_str("path,content,emoji,line_start,line_end,chunk_type,language,content_hash,token_count,line_count,char_count,test_result\n");
    // CSV rows
    for doc in documents {
        csv_output.push_str(&format!("\"{}\",\"{}\",\"{}\",{},{},\"{}\",\"{}\",\"{}\",{},{},{},\"{}\"\n",
            doc.path.as_deref().unwrap_or(""),
            doc.content.as_deref().unwrap_or("").replace('"', "\""),
            doc.emoji.as_deref().unwrap_or(""),
            doc.line_start.unwrap_or(0),
            doc.line_end.unwrap_or(0),
            doc.chunk_type.as_deref().unwrap_or(""),
            doc.language.as_deref().unwrap_or(""),
            doc.content_hash.as_deref().unwrap_or(""),
            doc.token_count.unwrap_or(0),
            doc.line_count.unwrap_or(0),
            doc.char_count.unwrap_or(0),
            doc.test_result.as_deref().unwrap_or("")
        ));
    }
    match output_path {
        Some(path) => {
            fs::write(path, csv_output)?;
            println!("Exported {} documents to CSV file: {:?}", documents.len(), path);
        },
        None => {
            println!("{}", csv_output);
        }
    }
    Ok(())
}
