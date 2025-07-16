use std::path::PathBuf;
use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};
use tantivy::{Index, IndexReader};
use tantivy::directory::MmapDirectory;
use tantivy::schema::TantivyDocument;
use tantivy::DocId;
use clap::Parser;

#[derive(Parser)]
#[command(name = "index_exporter")]
#[command(about = "Export Tantivy index to readable formats")]
struct Cli {
    /// Path to the Tantivy index directory
    #[arg(short, long, default_value = "tmp/tantivy_index")]
    index_path: PathBuf,
    
    /// Output format: json, csv, markdown, or stats
    #[arg(short, long, default_value = "json")]
    format: String,
    
    /// Output file path (defaults to stdout if not specified)
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Maximum number of documents to export (0 = all)
    #[arg(short, long, default_value = "0")]
    limit: usize,
    
    /// Filter by emoji
    #[arg(long)]
    emoji: Option<String>,
    
    /// Filter by path pattern
    #[arg(long)]
    path_pattern: Option<String>,
    
    /// Show only statistics
    #[arg(long)]
    stats_only: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexedDocument {
    pub path: Option<String>,
    pub content: Option<String>,
    pub emoji: Option<String>,
    pub line_start: Option<u64>,
    pub line_end: Option<u64>,
    pub chunk_type: Option<String>,
    pub language: Option<String>,
    pub content_hash: Option<String>,
    pub token_count: Option<u64>,
    pub line_count: Option<u64>,
    pub char_count: Option<u64>,
    pub test_result: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexStats {
    pub total_documents: usize,
    pub fields: Vec<FieldStats>,
    pub emoji_distribution: std::collections::HashMap<String, usize>,
    pub language_distribution: std::collections::HashMap<String, usize>,
    pub chunk_type_distribution: std::collections::HashMap<String, usize>,
    pub path_distribution: std::collections::HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FieldStats {
    pub field_name: String,
    pub non_null_count: usize,
    pub unique_values: usize,
}

struct IndexExporter {
    index: Index,
    reader: IndexReader,
}

impl IndexExporter {
    fn new(index_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let index = Index::open(MmapDirectory::open(index_path)?)?;
        let reader = index.reader()?;
        
        Ok(Self { index, reader })
    }
    
    fn export_all_documents(&self, limit: usize) -> Result<Vec<IndexedDocument>, Box<dyn Error>> {
        let searcher = self.reader.searcher();
        let schema = self.index.schema();
        
        let total_docs = searcher.num_docs();
        let export_limit = if limit == 0 { total_docs } else { limit.min(total_docs) };
        
        println!("Exporting {} documents from index (total: {})", export_limit, total_docs);
        
        let mut documents = Vec::new();
        
        for doc_id in 0..export_limit {
            if let Ok(doc) = searcher.doc(DocId::from(doc_id)) {
                if let Ok(indexed_doc) = self.document_to_indexed_document(&doc, &schema) {
                    documents.push(indexed_doc);
                }
            }
        }
        
        Ok(documents)
    }
    
    fn document_to_indexed_document(&self, doc: &TantivyDocument, schema: &tantivy::schema::Schema) -> Result<IndexedDocument, Box<dyn Error>> {
        let mut indexed_doc = IndexedDocument {
            path: None,
            content: None,
            emoji: None,
            line_start: None,
            line_end: None,
            chunk_type: None,
            language: None,
            content_hash: None,
            token_count: None,
            line_count: None,
            char_count: None,
            test_result: None,
        };
        
        for (field, values) in doc.iter() {
            let field_name = schema.get_field_name(field);
            
            match field_name {
                "path" => {
                    if let Some(value) = values.first() {
                        indexed_doc.path = Some(value.as_text().unwrap_or("").to_string());
                    }
                },
                "content" => {
                    if let Some(value) = values.first() {
                        indexed_doc.content = Some(value.as_text().unwrap_or("").to_string());
                    }
                },
                "emoji" => {
                    if let Some(value) = values.first() {
                        indexed_doc.emoji = Some(value.as_text().unwrap_or("").to_string());
                    }
                },
                "line_start" => {
                    if let Some(value) = values.first() {
                        indexed_doc.line_start = Some(value.as_u64().unwrap_or(0));
                    }
                },
                "line_end" => {
                    if let Some(value) = values.first() {
                        indexed_doc.line_end = Some(value.as_u64().unwrap_or(0));
                    }
                },
                "chunk_type" => {
                    if let Some(value) = values.first() {
                        indexed_doc.chunk_type = Some(value.as_text().unwrap_or("").to_string());
                    }
                },
                "language" => {
                    if let Some(value) = values.first() {
                        indexed_doc.language = Some(value.as_text().unwrap_or("").to_string());
                    }
                },
                "content_hash" => {
                    if let Some(value) = values.first() {
                        indexed_doc.content_hash = Some(value.as_text().unwrap_or("").to_string());
                    }
                },
                "token_count" => {
                    if let Some(value) = values.first() {
                        indexed_doc.token_count = Some(value.as_u64().unwrap_or(0));
                    }
                },
                "line_count" => {
                    if let Some(value) = values.first() {
                        indexed_doc.line_count = Some(value.as_u64().unwrap_or(0));
                    }
                },
                "char_count" => {
                    if let Some(value) = values.first() {
                        indexed_doc.char_count = Some(value.as_u64().unwrap_or(0));
                    }
                },
                "test_result" => {
                    if let Some(value) = values.first() {
                        indexed_doc.test_result = Some(value.as_text().unwrap_or("").to_string());
                    }
                },
                _ => {}
            }
        }
        
        Ok(indexed_doc)
    }
    
    fn get_stats(&self) -> Result<IndexStats, Box<dyn Error>> {
        let searcher = self.reader.searcher();
        let schema = self.index.schema();
        let total_docs = searcher.num_docs();
        
        let mut emoji_distribution = std::collections::HashMap::new();
        let mut language_distribution = std::collections::HashMap::new();
        let mut chunk_type_distribution = std::collections::HashMap::new();
        let mut path_distribution = std::collections::HashMap::new();
        
        // Collect statistics from all documents
        for doc_id in 0..total_docs {
            if let Ok(doc) = searcher.doc(DocId::from(doc_id)) {
                if let Ok(indexed_doc) = self.document_to_indexed_document(&doc, &schema) {
                    // Count emojis
                    if let Some(emoji) = indexed_doc.emoji {
                        *emoji_distribution.entry(emoji).or_insert(0) += 1;
                    }
                    
                    // Count languages
                    if let Some(language) = indexed_doc.language {
                        *language_distribution.entry(language).or_insert(0) += 1;
                    }
                    
                    // Count chunk types
                    if let Some(chunk_type) = indexed_doc.chunk_type {
                        *chunk_type_distribution.entry(chunk_type).or_insert(0) += 1;
                    }
                    
                    // Count paths
                    if let Some(path) = indexed_doc.path {
                        *path_distribution.entry(path).or_insert(0) += 1;
                    }
                }
            }
        }
        
        // Create field statistics
        let fields = vec![
            FieldStats {
                field_name: "emoji".to_string(),
                non_null_count: emoji_distribution.values().sum(),
                unique_values: emoji_distribution.len(),
            },
            FieldStats {
                field_name: "language".to_string(),
                non_null_count: language_distribution.values().sum(),
                unique_values: language_distribution.len(),
            },
            FieldStats {
                field_name: "chunk_type".to_string(),
                non_null_count: chunk_type_distribution.values().sum(),
                unique_values: chunk_type_distribution.len(),
            },
            FieldStats {
                field_name: "path".to_string(),
                non_null_count: path_distribution.values().sum(),
                unique_values: path_distribution.len(),
            },
        ];
        
        Ok(IndexStats {
            total_documents: total_docs,
            fields,
            emoji_distribution,
            language_distribution,
            chunk_type_distribution,
            path_distribution,
        })
    }
}

fn export_to_json(documents: &[IndexedDocument], output_path: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
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

fn export_to_csv(documents: &[IndexedDocument], output_path: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut csv_output = String::new();
    
    // CSV header
    csv_output.push_str("path,content,emoji,line_start,line_end,chunk_type,language,content_hash,token_count,line_count,char_count,test_result\n");
    
    // CSV rows
    for doc in documents {
        csv_output.push_str(&format!("\"{}\",\"{}\",\"{}\",{},{},\"{}\",\"{}\",\"{}\",{},{},{},\"{}\"\n",
            doc.path.as_deref().unwrap_or(""),
            doc.content.as_deref().unwrap_or("").replace("\"", "\"\""),
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

fn export_to_markdown(documents: &[IndexedDocument], output_path: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
    let mut md_output = String::new();
    
    md_output.push_str("# Tantivy Index Export\n\n");
    md_output.push_str(&format!("Total documents: {}\n\n", documents.len()));
    
    for (i, doc) in documents.iter().enumerate() {
        md_output.push_str(&format!("## Document {}\n\n", i + 1));
        
        if let Some(path) = &doc.path {
            md_output.push_str(&format!("**Path:** `{}`\n\n", path));
        }
        
        if let Some(emoji) = &doc.emoji {
            md_output.push_str(&format!("**Emoji:** {}\n\n", emoji));
        }
        
        if let Some(language) = &doc.language {
            md_output.push_str(&format!("**Language:** {}\n\n", language));
        }
        
        if let Some(chunk_type) = &doc.chunk_type {
            md_output.push_str(&format!("**Type:** {}\n\n", chunk_type));
        }
        
        if let Some(line_start) = doc.line_start {
            if let Some(line_end) = doc.line_end {
                md_output.push_str(&format!("**Lines:** {} - {}\n\n", line_start, line_end));
            }
        }
        
        if let Some(content) = &doc.content {
            md_output.push_str("**Content:**\n```");
            if let Some(lang) = &doc.language {
                md_output.push_str(lang);
            }
            md_output.push_str("\n");
            md_output.push_str(content);
            md_output.push_str("\n```\n\n");
        }
        
        if let Some(hash) = &doc.content_hash {
            md_output.push_str(&format!("**Hash:** `{}`\n\n", hash));
        }
        
        md_output.push_str(&format!("**Metrics:** {} tokens, {} lines, {} chars\n\n", 
            doc.token_count.unwrap_or(0),
            doc.line_count.unwrap_or(0),
            doc.char_count.unwrap_or(0)
        ));
        
        if let Some(test_result) = &doc.test_result {
            md_output.push_str(&format!("**Test Result:** {}\n\n", test_result));
        }
        
        md_output.push_str("---\n\n");
    }
    
    match output_path {
        Some(path) => {
            fs::write(path, md_output)?;
            println!("Exported {} documents to Markdown file: {:?}", documents.len(), path);
        },
        None => {
            println!("{}", md_output);
        }
    }
    
    Ok(())
}

fn export_stats(stats: &IndexStats, output_path: &Option<PathBuf>) -> Result<(), Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    println!("🔍 Tantivy Index Exporter");
    println!("=========================");
    
    if !cli.index_path.exists() {
        eprintln!("❌ Index path does not exist: {:?}", cli.index_path);
        return Ok(());
    }
    
    let exporter = IndexExporter::new(&cli.index_path)?;
    
    if cli.stats_only {
        println!("📊 Generating index statistics...");
        let stats = exporter.get_stats()?;
        export_stats(&stats, &cli.output)?;
    } else {
        println!("📄 Exporting documents...");
        let documents = exporter.export_all_documents(cli.limit)?;
        
        match cli.format.as_str() {
            "json" => export_to_json(&documents, &cli.output)?,
            "csv" => export_to_csv(&documents, &cli.output)?,
            "markdown" => export_to_markdown(&documents, &cli.output)?,
            _ => {
                eprintln!("❌ Unsupported format: {}. Use: json, csv, markdown", cli.format);
                return Ok(());
            }
        }
    }
    
    println!("✅ Export completed successfully!");
    
    Ok(())
} 