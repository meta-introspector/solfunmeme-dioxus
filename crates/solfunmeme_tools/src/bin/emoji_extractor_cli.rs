use anyhow::Result;
use tantivy::{Index, schema::*, doc};
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: emoji_extractor <command> [options]");
        println!("Commands:");
        println!("  scan <directory>     - Scan directory for emojis");
        println!("  index <directory>    - Scan and add to Tantivy index");
        println!("  stats                - Show emoji statistics");
        return Ok(());
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "scan" => {
            if args.len() < 3 {
                println!("Usage: emoji_extractor scan <directory>");
                return Ok(());
            }
            let dir = &args[2];
            scan_directory_for_emojis(dir)?;
        }
        
        "index" => {
            if args.len() < 3 {
                println!("Usage: emoji_extractor index <directory>");
                return Ok(());
            }
            let dir = &args[2];
            index_emojis_from_directory(dir)?;
        }
        
        "stats" => {
            show_emoji_stats()?;
        }
        
        _ => {
            println!("Unknown command: {}", command);
            println!("Use 'emoji_extractor' without arguments to see available commands.");
        }
    }
    
    Ok(())
}

fn scan_directory_for_emojis(dir: &str) -> Result<()> {
    println!("Scanning directory: {}", dir);
    
    let emoji_regex = Regex::new(r"[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]")?;
    
    let mut emoji_counts: HashMap<String, usize> = HashMap::new();
    let mut file_emojis: HashMap<String, Vec<String>> = HashMap::new();
    
    scan_recursive(Path::new(dir), &emoji_regex, &mut emoji_counts, &mut file_emojis)?;
    
    println!("\nEmoji Summary:");
    println!("==============");
    println!("Total unique emojis found: {}", emoji_counts.len());
    println!("Total files with emojis: {}", file_emojis.len());
    
    println!("\nTop 20 Emojis:");
    let mut sorted_emojis: Vec<(String, usize)> = emoji_counts.into_iter().collect();
    sorted_emojis.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (i, (emoji, count)) in sorted_emojis.iter().take(20).enumerate() {
        println!("{:3}. {:4} - {} occurrences", i + 1, emoji, count);
    }
    
    println!("\nFiles with most emojis:");
    let mut sorted_files: Vec<(String, usize)> = file_emojis
        .into_iter()
        .map(|(file, emojis)| (file, emojis.len()))
        .collect();
    sorted_files.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (i, (file, count)) in sorted_files.iter().take(10).enumerate() {
        println!("{:3}. {} - {} emojis", i + 1, file, count);
    }
    
    Ok(())
}

fn index_emojis_from_directory(dir: &str) -> Result<()> {
    println!("Indexing emojis from directory: {}", dir);
    
    // Create or open index
    let index_path = Path::new("codebase_index");
    let schema = create_schema();
    
    let index = if index_path.exists() {
        Index::open_in_dir(index_path)?
    } else {
        std::fs::create_dir_all(index_path)?;
        Index::create_in_dir(index_path, schema.clone())?
    };
    
    let mut writer = index.writer(50_000_000)?; // 50MB buffer
    
    let emoji_regex = Regex::new(r"[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]")?;
    
    let mut emoji_counts: HashMap<String, usize> = HashMap::new();
    let mut file_emojis: HashMap<String, Vec<String>> = HashMap::new();
    
    scan_recursive(Path::new(dir), &emoji_regex, &mut emoji_counts, &mut file_emojis)?;
    
    // Add documents to index
    let path_field = schema.get_field("path")?;
    let content_field = schema.get_field("content")?;
    let emoji_field = schema.get_field("emoji")?;
    let line_start_field = schema.get_field("line_start")?;
    let line_end_field = schema.get_field("line_end")?;
    
    let mut doc_count = 0;
    
    for (file_path, emojis) in file_emojis {
        if !emojis.is_empty() {
            // Read file content
            let content = fs::read_to_string(&file_path)?;
            let lines: Vec<&str> = content.lines().collect();
            
            // Find lines with emojis
            for (line_num, line) in lines.iter().enumerate() {
                let line_emojis: Vec<&str> = emoji_regex.find_iter(line)
                    .map(|m| m.as_str())
                    .collect();
                
                if !line_emojis.is_empty() {
                    let doc = doc!(
                        path_field => file_path.clone(),
                        content_field => line.to_string(),
                        emoji_field => line_emojis.join(" "),
                        line_start_field => (line_num + 1) as u64,
                        line_end_field => (line_num + 1) as u64,
                    );
                    
                    writer.add_document(doc)?;
                    doc_count += 1;
                }
            }
        }
    }
    
    writer.commit()?;
    
    println!("Indexed {} documents with emojis", doc_count);
    println!("Total unique emojis: {}", emoji_counts.len());
    
    Ok(())
}

fn show_emoji_stats() -> Result<()> {
    let index_path = Path::new("codebase_index");
    
    if !index_path.exists() {
        println!("No index found at: {}", index_path.display());
        return Ok(());
    }
    
    let index = Index::open_in_dir(index_path)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();
    
    // Try to get emoji field, fall back to content field
    let emoji_field = schema.get_field("emoji").or_else(|_| schema.get_field("content"))?;
    let query_parser = tantivy::query::QueryParser::for_index(&index, vec![emoji_field]);
    let query = query_parser.parse_query("*")?;
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(1000))?;
    
    println!("Emoji Statistics:");
    println!("=================");
    println!("Total documents in index: {}", top_docs.len());
    
    let mut emoji_counts: HashMap<String, usize> = HashMap::new();
    
    for (_score, doc_address) in top_docs {
        let doc: tantivy::TantivyDocument = searcher.doc(doc_address)?;
        if let Some(content) = doc.get_first(emoji_field) {
            if let Some(content_str) = content.as_value().as_str() {
                // Extract emojis from content using regex
                let emoji_regex = Regex::new(r"[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]")?;
                
                for emoji_match in emoji_regex.find_iter(content_str) {
                    let emoji = emoji_match.as_str();
                    *emoji_counts.entry(emoji.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    
    println!("Unique emojis found: {}", emoji_counts.len());
    
    let mut sorted_emojis: Vec<(String, usize)> = emoji_counts.into_iter().collect();
    sorted_emojis.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\nTop 20 Emojis:");
    for (i, (emoji, count)) in sorted_emojis.iter().take(20).enumerate() {
        println!("{:3}. {:4} - {} occurrences", i + 1, emoji, count);
    }
    
    Ok(())
}

fn scan_recursive(
    dir: &Path,
    emoji_regex: &Regex,
    emoji_counts: &mut HashMap<String, usize>,
    file_emojis: &mut HashMap<String, Vec<String>>,
) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Skip vendor and target directories
            let path_str = path.to_string_lossy();
            if path_str.contains("vendor") || path_str.contains("target") || path_str.contains(".git") {
                continue;
            }
            scan_recursive(&path, emoji_regex, emoji_counts, file_emojis)?;
        } else if path.is_file() {
            // Only process text files
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if matches!(ext_str.as_str(), "rs" | "md" | "txt" | "toml" | "json" | "yaml" | "yml") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let mut file_emoji_list = Vec::new();
                        
                        for emoji_match in emoji_regex.find_iter(&content) {
                            let emoji = emoji_match.as_str();
                            *emoji_counts.entry(emoji.to_string()).or_insert(0) += 1;
                            file_emoji_list.push(emoji.to_string());
                        }
                        
                        if !file_emoji_list.is_empty() {
                            file_emojis.insert(path.to_string_lossy().to_string(), file_emoji_list);
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_text_field("content", TEXT | STORED);
    schema_builder.add_text_field("emoji", TEXT | STORED);
    schema_builder.add_u64_field("line_start", STORED);
    schema_builder.add_u64_field("line_end", STORED);
    
    schema_builder.build()
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        matches!(self,
            '\u{1F600}'..='\u{1F64F}' | // Emoticons
            '\u{1F300}'..='\u{1F5FF}' | // Miscellaneous Symbols and Pictographs
            '\u{1F680}'..='\u{1F6FF}' | // Transport and Map Symbols
            '\u{1F1E0}'..='\u{1F1FF}' | // Regional Indicator Symbols
            '\u{2600}'..='\u{26FF}'   | // Miscellaneous Symbols
            '\u{2700}'..='\u{27BF}'     // Dingbats
        )
    }
} 