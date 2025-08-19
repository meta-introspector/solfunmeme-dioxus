use anyhow::Result;
use tantivy::{Index, query::QueryParser, collector::TopDocs, TantivyDocument, schema::Value};
use tantivy::directory::MmapDirectory;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: codebase_analyzer <command> [options]");
        println!("Commands:");
        println!("  word-freq [limit]     - Show top words by frequency");
        println!("  emoji-freq [limit]    - Show top emojis by frequency");
        println!("  file-types [limit]    - Show file types by count");
        println!("  search <query> [limit] - Search codebase content");
        println!("  stats                 - Show overall statistics");
        return Ok(());
    }
    
    let command = &args[1];
    let index_path = Path::new("codebase_index");
    
    // Check if index exists
    if !index_path.exists() {
        println!("Error: codebase_index directory not found!");
        println!("Please ensure the Tantivy index exists at: {}", index_path.display());
        return Ok(());
    }
    
    // Open the existing index
    let index = Index::open(MmapDirectory::open(index_path)?)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();
    
    match command.as_str() {
        "word-freq" => {
            let limit = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(50);
            println!("Top {} words in codebase:", limit);
            
            // Get the content field
            let content_field = schema.get_field("content")?;
            let query_parser = QueryParser::for_index(&index, vec![content_field]);
            let query = query_parser.parse_query("*")?;
            let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;
            
            println!("Found {} documents", top_docs.len());
            for (i, (_score, doc_address)) in top_docs.iter().enumerate() {
                let doc: TantivyDocument = searcher.doc(*doc_address)?;
                if let Some(content) = doc.get_first(content_field) {
                    if let Some(content_str) = content.as_value().as_str() {
                        let words: Vec<&str> = content_str.split_whitespace().take(5).collect();
                        println!("{:3}. {}...", i + 1, words.join(" "));
                    }
                }
            }
        }
        
        "emoji-freq" => {
            let limit = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(20);
            analyze_emoji_frequency(limit)?;
        }
        
        "file-types" => {
            let limit = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(20);
            println!("Top {} file types in codebase:", limit);
            
            // Get the path field
            let path_field = schema.get_field("path")?;
            let query_parser = QueryParser::for_index(&index, vec![path_field]);
            let query = query_parser.parse_query("*")?;
            let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;
            
            let mut extensions = std::collections::HashMap::new();
            for (_score, doc_address) in top_docs {
                let doc: TantivyDocument = searcher.doc(doc_address)?;
                if let Some(path) = doc.get_first(path_field) {
                    if let Some(path_str) = path.as_value().as_str() {
                        let extension = path_str.split('.').last().unwrap_or("no-ext");
                        *extensions.entry(extension.to_string()).or_insert(0) += 1;
                    }
                }
            }
            
            let mut sorted_extensions: Vec<(String, usize)> = extensions.into_iter().collect();
            sorted_extensions.sort_by(|a, b| b.1.cmp(&a.1));
            sorted_extensions.truncate(limit);
            
            for (i, (ext, count)) in sorted_extensions.iter().enumerate() {
                println!("{:3}. {:10} - {}", i + 1, ext, count);
            }
        }
        
        "search" => {
            if args.len() < 3 {
                println!("Usage: codebase_analyzer search <query> [limit]");
                return Ok(());
            }
            let query = &args[2];
            let limit = args.get(3).and_then(|s| s.parse::<usize>().ok()).unwrap_or(10);
            
            let content_field = schema.get_field("content")?;
            let path_field = schema.get_field("path")?;
            let query_parser = QueryParser::for_index(&index, vec![content_field, path_field]);
            let parsed_query = query_parser.parse_query(query)?;
            let top_docs = searcher.search(&parsed_query, &TopDocs::with_limit(limit))?;
            
            println!("Search results for '{}':", query);
            for (i, (score, doc_address)) in top_docs.iter().enumerate() {
                let doc: TantivyDocument = searcher.doc(*doc_address)?;
                println!("{:3}. Score: {:.3}", i + 1, score);
                
                if let Some(path) = doc.get_first(path_field) {
                    if let Some(path_str) = path.as_value().as_str() {
                        println!("     Path: {}", path_str);
                    }
                }
                
                if let Some(content) = doc.get_first(content_field) {
                    if let Some(content_str) = content.as_value().as_str() {
                        println!("     Content: {}", content_str);
                    }
                }
                println!();
            }
        }
        
        "stats" => {
            show_stats()?;
        }
        
        _ => {
            println!("Unknown command: {}", command);
            println!("Use 'codebase_analyzer' without arguments to see available commands.");
        }
    }
    
    Ok(())
}

fn analyze_emoji_frequency(limit: usize) -> Result<()> {
    let index_path = Path::new("codebase_index");

    if !index_path.exists() {
        println!("No index found at: {}", index_path.display());
        return Ok(());
    }

    let index = Index::open(MmapDirectory::open(index_path)?)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();

    let content_field = schema.get_field("content")?;
    let emoji_field_opt = schema.get_field("emoji"); // Check if emoji field exists

    println!("Top {} emojis in codebase:", limit);
    println!("==========================");

    let mut emoji_counts: HashMap<String, usize> = HashMap::new();

    for segment_reader in searcher.segment_readers() {
        let store_reader = segment_reader.get_store_reader(100)?;
        for doc_id in 0..segment_reader.num_docs() {
            let doc: TantivyDocument = store_reader.get(doc_id)?;
            
            let content_str = if let Ok(emoji_field) = emoji_field_opt {
                // Try to get from emoji field first
                doc.get_first(emoji_field)
                    .and_then(|v| v.as_value().as_str())
                    .unwrap_or("")
                    .to_string()
            } else {
                // Fallback to content field if emoji field doesn't exist
                doc.get_first(content_field)
                    .and_then(|v| v.as_value().as_str())
                    .unwrap_or("")
                    .to_string()
            };

            // Extract emojis from content using regex
            let emoji_regex = Regex::new(r"[\u{1F600}-\u{1F64F}]|[\u{1F300}-\u{1F5FF}]|[\u{1F680}-\u{1F6FF}]|[\u{1F1E0}-\u{1F1FF}]|[\u{2600}-\u{26FF}]|[\u{2700}-\u{27BF}]")?;

            for emoji_match in emoji_regex.find_iter(&content_str) {
                let emoji = emoji_match.as_str();
                *emoji_counts.entry(emoji.to_string()).or_insert(0) += 1;
            }
        }
    }

    let mut sorted_emojis: Vec<(String, usize)> = emoji_counts.into_iter().collect();
    sorted_emojis.sort_by(|a, b| b.1.cmp(&a.1));

    for (i, (emoji, count)) in sorted_emojis.iter().take(limit).enumerate() {
        let emoji_name = emojis::get(emoji).map_or("", |e| e.name());
        println!("{:3}. {:4} - {:<30} - {} occurrences", i + 1, emoji, emoji_name, count);
    }

    Ok(())
}

fn show_stats() -> Result<()> {
    let index_path = Path::new("codebase_index");
    
    if !index_path.exists() {
        println!("No index found at: {}", index_path.display());
        return Ok(());
    }
    
    let index = Index::open(MmapDirectory::open(index_path)?)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();
    
    println!("Codebase Statistics:");
    println!("===================");
    println!("Total documents in index: {}", searcher.num_docs());
    
    // Check what fields are available
    let field_names: Vec<String> = schema.fields().map(|(field, entry)| {
        schema.get_field_name(field).to_string()
    }).collect();
    println!("Available fields: {:?}", field_names);
    
    // Try to get sample documents based on available fields
    let query_parser = tantivy::query::QueryParser::for_index(&index, vec![]);
    let query = query_parser.parse_query("*")?;
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(5))?;
    
    println!("\nSample documents:");
    for (i, (_score, doc_address)) in top_docs.iter().enumerate() {
        let doc: tantivy::TantivyDocument = searcher.doc(*doc_address)?;
        println!("Document {}: {:?}", i + 1, doc);
    }
    
    Ok(())
} 