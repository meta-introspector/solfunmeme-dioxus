use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use regex::Regex;
use crate::utils;

#[derive(Debug)]
pub struct DocumentChunk {
    pub id: String,
    pub title: String,
    pub content: String,
    pub chunk_type: ChunkType,
    pub external_files: Vec<ExternalFile>,
}

#[derive(Debug, Clone)]
pub enum ChunkType {
    Conversation,
    CodeBlock,
    Documentation,
    Analysis,
    Other,
}

#[derive(Debug)]
pub struct ExternalFile {
    pub filename: String,
    pub content: String,
    pub file_type: String,
}

pub struct ChunkProcessor {
    output_base: PathBuf,
}

impl ChunkProcessor {
    pub fn new(output_base: PathBuf) -> Self {
        Self { output_base }
    }

    pub fn process_document(&self, file_path: &Path, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Extract date from filename or use current date
        let date = self.extract_date_from_filename(file_path)?;
        
        // Create normalized filename
        let normalized_name = utils::clean_filename(
            file_path.file_stem().unwrap().to_string_lossy().as_ref()
        );
        
        // Create directory structure: processed_docs/YYYY/MM/DD/filename_normalized/
        let doc_dir = self.output_base
            .join(&date.format("%Y").to_string())
            .join(&date.format("%m").to_string())
            .join(&date.format("%d").to_string())
            .join(&normalized_name);
        
        fs::create_dir_all(&doc_dir)?;
        
        // Split content into chunks
        let chunks = self.split_into_chunks(content)?;
        
        // Process each chunk
        for (i, chunk) in chunks.iter().enumerate() {
            self.process_chunk(&doc_dir, i, chunk)?;
        }
        
        // Create main index file
        self.create_index_file(&doc_dir, file_path, &chunks)?;
        
        println!("[INFO] Processed {} into {} chunks at: {}", 
            file_path.display(), chunks.len(), doc_dir.display());
        
        Ok(())
    }

    fn extract_date_from_filename(&self, file_path: &Path) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
        let filename = file_path.file_name().unwrap().to_string_lossy();
        
        // Try to extract date from filename patterns like "2025-01-15" or "2025/01/15"
        let date_patterns = vec![
            r"(\d{4})-(\d{2})-(\d{2})",
            r"(\d{4})/(\d{2})/(\d{2})",
            r"(\d{4})_(\d{2})_(\d{2})",
        ];
        
        for pattern in date_patterns {
            if let Some(captures) = Regex::new(pattern).unwrap().captures(&filename) {
                let year = captures[1].parse::<i32>()?;
                let month = captures[2].parse::<u32>()?;
                let day = captures[3].parse::<u32>()?;
                
                return Ok(DateTime::parse_from_rfc3339(&format!("{}-{:02}-{:02}T00:00:00Z", year, month, day))?.with_timezone(&Utc));
            }
        }
        
        // If no date found, use file modification time or current time
        if let Ok(metadata) = fs::metadata(file_path) {
            if let Ok(modified) = metadata.modified() {
                return Ok(DateTime::from(modified));
            }
        }
        
        Ok(Utc::now())
    }

    fn split_into_chunks(&self, content: &str) -> Result<Vec<DocumentChunk>, Box<dyn std::error::Error>> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut chunk_type = ChunkType::Other;
        let mut chunk_title = String::new();
        
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Detect chunk boundaries
            if self.is_chunk_boundary(line) {
                // Save current chunk if not empty
                if !current_chunk.trim().is_empty() {
                    chunks.push(DocumentChunk {
                        id: format!("chunk_{}", chunks.len()),
                        title: chunk_title.clone(),
                        content: current_chunk.trim().to_string(),
                        chunk_type: chunk_type.clone(),
                        external_files: Vec::new(),
                    });
                }
                
                // Start new chunk
                current_chunk.clear();
                chunk_title = self.extract_title(line);
                chunk_type = self.detect_chunk_type(line);
            } else {
                current_chunk.push_str(line);
                current_chunk.push('\n');
            }
            
            i += 1;
        }
        
        // Add final chunk
        if !current_chunk.trim().is_empty() {
            chunks.push(DocumentChunk {
                id: format!("chunk_{}", chunks.len()),
                title: chunk_title,
                content: current_chunk.trim().to_string(),
                chunk_type,
                external_files: Vec::new(),
            });
        }
        
        Ok(chunks)
    }

    fn is_chunk_boundary(&self, line: &str) -> bool {
        // Detect headers, conversation markers, etc.
        line.starts_with("# ") || 
        line.starts_with("## ") || 
        line.starts_with("### ") ||
        line.starts_with("User:") ||
        line.starts_with("Grok AI:") ||
        line.starts_with("---") ||
        line.starts_with("```") ||
        line.contains("[START PART") ||
        line.contains("[END PART")
    }

    fn extract_title(&self, line: &str) -> String {
        if line.starts_with("# ") {
            line.trim_start_matches("# ").to_string()
        } else if line.starts_with("## ") {
            line.trim_start_matches("## ").to_string()
        } else if line.starts_with("### ") {
            line.trim_start_matches("### ").to_string()
        } else {
            line.to_string()
        }
    }

    fn detect_chunk_type(&self, line: &str) -> ChunkType {
        if line.starts_with("User:") || line.starts_with("Grok AI:") {
            ChunkType::Conversation
        } else if line.starts_with("```") {
            ChunkType::CodeBlock
        } else if line.starts_with("#") {
            ChunkType::Documentation
        } else {
            ChunkType::Other
        }
    }

    fn process_chunk(&self, doc_dir: &Path, index: usize, chunk: &DocumentChunk) -> Result<(), Box<dyn std::error::Error>> {
        let chunk_dir = doc_dir.join(&chunk.id);
        fs::create_dir_all(&chunk_dir)?;
        
        // Extract external files (code blocks, large content)
        let (processed_content, external_files) = self.extract_external_files(&chunk.content)?;
        
        // Write main chunk content
        let chunk_file = chunk_dir.join("content.md");
        let mut content = String::new();
        content.push_str(&format!("# {}\n\n", chunk.title));
        content.push_str(&processed_content);
        
        fs::write(&chunk_file, content)?;
        
        // Write external files
        for (_i, ext_file) in external_files.iter().enumerate() {
            let ext_path = chunk_dir.join(&ext_file.filename);
            fs::write(&ext_path, &ext_file.content)?;
        }
        
        // Write metadata
        let metadata = serde_json::json!({
            "id": chunk.id,
            "title": chunk.title,
            "type": format!("{:?}", chunk.chunk_type),
            "external_files": external_files.iter().map(|f| f.filename.clone()).collect::<Vec<_>>(),
            "index": index
        });
        
        let metadata_file = chunk_dir.join("metadata.json");
        fs::write(&metadata_file, serde_json::to_string_pretty(&metadata)?)?;
        
        Ok(())
    }

    fn extract_external_files(&self, content: &str) -> Result<(String, Vec<ExternalFile>), Box<dyn std::error::Error>> {
        let mut external_files = Vec::new();
        let mut processed_content = content.to_string();
        
        // Extract code blocks
        let code_block_regex = Regex::new(r"```(\w+)?\n(.*?)```")?;
        let mut offset = 0;
        
        for cap in code_block_regex.captures_iter(content) {
            let language = cap.get(1).map(|m| m.as_str()).unwrap_or("text");
            let code_content = cap.get(2).unwrap().as_str();
            
            if code_content.len() > 100 { // Only externalize large code blocks
                let filename = format!("code_{}.{}", external_files.len(), self.get_file_extension(language));
                let external_file = ExternalFile {
                    filename: filename.clone(),
                    content: code_content.to_string(),
                    file_type: language.to_string(),
                };
                external_files.push(external_file);
                
                // Replace in content with link
                let link = format!("[Code Block: {}]({})", language, filename);
                let start = cap.get(0).unwrap().start() + offset;
                let end = cap.get(0).unwrap().end() + offset;
                processed_content.replace_range(start..end, &link);
                offset += link.len() - (end - start);
            }
        }
        
        Ok((processed_content, external_files))
    }

    fn get_file_extension(&self, language: &str) -> String {
        match language {
            "rust" => "rs".to_string(),
            "python" => "py".to_string(),
            "javascript" => "js".to_string(),
            "typescript" => "ts".to_string(),
            "bash" => "sh".to_string(),
            "shell" => "sh".to_string(),
            "json" => "json".to_string(),
            "toml" => "toml".to_string(),
            "yaml" => "yml".to_string(),
            "html" => "html".to_string(),
            "css" => "css".to_string(),
            "sql" => "sql".to_string(),
            _ => "txt".to_string(),
        }
    }

    fn create_index_file(&self, doc_dir: &Path, original_file: &Path, chunks: &[DocumentChunk]) -> Result<(), Box<dyn std::error::Error>> {
        let mut index_content = String::new();
        index_content.push_str(&format!("# {}\n\n", original_file.file_name().unwrap().to_string_lossy()));
        index_content.push_str("## Chunks\n\n");
        
        for chunk in chunks {
            index_content.push_str(&format!("- [{}]({}/content.md) - {}\n", 
                chunk.title, chunk.id, format!("{:?}", chunk.chunk_type)));
        }
        
        index_content.push_str("\n## Original File\n\n");
        index_content.push_str(&format!("Source: `{}`\n", original_file.display()));
        index_content.push_str(&format!("Processed: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));
        
        let index_file = doc_dir.join("index.md");
        fs::write(&index_file, index_content)?;
        
        Ok(())
    }
} 