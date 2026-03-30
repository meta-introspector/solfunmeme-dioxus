use anyhow::Result;
use solfunmeme_search_tantivy::{SearchIndex, SearchResult};
use solfunmeme_function_analysis::CodeChunk;
use std::path::Path;
use walkdir::WalkDir;
use std::fs;

/// Vibe finder that uses Tantivy to find code matching chat vibes
pub struct VibeFinder {
    search_index: SearchIndex,
}

/// Result of a vibe search
#[derive(Debug, Clone)]
pub struct VibeSearchResult {
    pub code_chunk: CodeChunk,
    pub search_result: SearchResult,
    pub vibe_similarity: f32,
    pub chat_context: String,
}

impl VibeFinder {
    pub fn new(index_path: &Path) -> Result<Self> {
        let search_index = SearchIndex::new(index_path)?;
        Ok(Self { search_index })
    }

    /// Find code that matches a chat vibe
    pub fn find_vibe_matches(&self, chat_text: &str, limit: usize) -> Result<Vec<VibeSearchResult>> {
        // Use Tantivy to search for code chunks that match the chat vibe
        let search_results = self.search_index.search(chat_text, limit)?;
        
        let mut vibe_results = Vec::new();
        
        for search_result in search_results {
            // Convert SearchResult back to CodeChunk
            let code_chunk = self.search_result_to_code_chunk(&search_result)?;
            
            // Calculate vibe similarity (could be enhanced with embeddings)
            let vibe_similarity = self.calculate_vibe_similarity(chat_text, &code_chunk);
            
            let vibe_result = VibeSearchResult {
                code_chunk,
                search_result,
                vibe_similarity,
                chat_context: chat_text.to_string(),
            };
            
            vibe_results.push(vibe_result);
        }
        
        // Sort by vibe similarity
        vibe_results.sort_by(|a, b| b.vibe_similarity.partial_cmp(&a.vibe_similarity).unwrap());
        
        Ok(vibe_results)
    }

    /// Index a directory of Rust files for vibe searching
    pub fn index_directory(&mut self, path: &Path) -> Result<()> {
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    self.index_file(entry.path())?;
                }
            }
        }
        
        self.search_index.commit()?;
        Ok(())
    }

    /// Index a single Rust file
    fn index_file(&mut self, file_path: &Path) -> Result<()> {
        let content = fs::read_to_string(file_path)?;
        
        // Parse the file into code chunks
        let code_chunks = self.parse_file_to_chunks(file_path, &content)?;
        
        // Add each chunk to the search index
        for chunk in code_chunks {
            self.search_index.add_chunk(&chunk)?;
        }
        
        Ok(())
    }

    /// Parse a file into code chunks
    fn parse_file_to_chunks(&self, file_path: &Path, content: &str) -> Result<Vec<CodeChunk>> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        // Simple chunking by function boundaries
        let mut current_chunk_start = 0;
        let mut in_function = false;
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Detect function start
            if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                // Save previous chunk if we were in one
                if in_function && current_chunk_start < line_num {
                    let chunk = self.create_code_chunk(
                        file_path,
                        &lines[current_chunk_start..line_num],
                        current_chunk_start + 1,
                        line_num,
                    )?;
                    chunks.push(chunk);
                }
                
                current_chunk_start = line_num;
                in_function = true;
            }
            // Detect function end (simple heuristic)
            else if in_function && trimmed == "}" && line_num > current_chunk_start {
                let chunk = self.create_code_chunk(
                    file_path,
                    &lines[current_chunk_start..=line_num],
                    current_chunk_start + 1,
                    line_num + 1,
                )?;
                chunks.push(chunk);
                in_function = false;
            }
        }
        
        // Add any remaining chunk
        if in_function && current_chunk_start < lines.len() {
            let chunk = self.create_code_chunk(
                file_path,
                &lines[current_chunk_start..],
                current_chunk_start + 1,
                lines.len(),
            )?;
            chunks.push(chunk);
        }
        
        Ok(chunks)
    }

    /// Create a CodeChunk from lines
    fn create_code_chunk(&self, _file_path: &Path, lines: &[&str], start: usize, end: usize) -> Result<CodeChunk> {
        let content = lines.join("\n");
        let content_hash = self.compute_hash(&content);
        let token_count = content.split_whitespace().count();
        let char_count = content.len();
        
        Ok(CodeChunk {
            language: "rust".to_string(),
            content,
            line_start: start,
            line_end: end,
            content_hash,
            token_count,
            line_count: lines.len(),
            char_count,
            test_result: "".to_string(), // Could be enhanced with actual test results
        })
    }

    /// Convert SearchResult back to CodeChunk
    fn search_result_to_code_chunk(&self, search_result: &SearchResult) -> Result<CodeChunk> {
        Ok(CodeChunk {
            language: search_result.language.clone(),
            content: search_result.content.clone(),
            line_start: search_result.line_start,
            line_end: search_result.line_end,
            content_hash: search_result.content_hash.clone(),
            token_count: search_result.token_count,
            line_count: search_result.line_count,
            char_count: search_result.char_count,
            test_result: search_result.test_result.clone(),
        })
    }

    /// Calculate vibe similarity between chat text and code chunk
    fn calculate_vibe_similarity(&self, chat_text: &str, code_chunk: &CodeChunk) -> f32 {
        // Simple Jaccard similarity on token sets
        let chat_tokens_vec: Vec<String> = chat_text
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();
        let chat_tokens: std::collections::HashSet<&str> = chat_tokens_vec
            .iter()
            .map(|s| s.as_str())
            .collect();
            
        let code_tokens_vec: Vec<String> = code_chunk.content
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();
        let code_tokens: std::collections::HashSet<&str> = code_tokens_vec
            .iter()
            .map(|s| s.as_str())
            .collect();
        
        let intersection = chat_tokens.intersection(&code_tokens).count();
        let union = chat_tokens.union(&code_tokens).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    /// Compute hash for content
    fn compute_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Generate a vibe search report
    pub fn generate_vibe_report(&self, chat_text: &str, results: &[VibeSearchResult]) -> String {
        let mut report = String::new();
        report.push_str("# Vibe Search Report\n\n");
        report.push_str(&format!("**Chat Context:** {}\n\n", chat_text));
        report.push_str(&format!("**Found {} matching code chunks:**\n\n", results.len()));
        
        for (i, result) in results.iter().enumerate() {
            report.push_str(&format!("## {}. {} (similarity: {:.3})\n", 
                i + 1, 
                result.code_chunk.content.lines().next().unwrap_or("Unknown"),
                result.vibe_similarity
            ));
            
            report.push_str(&format!("**File:** {}\n", result.search_result.content_hash));
            report.push_str(&format!("**Lines:** {}-{}\n", result.code_chunk.line_start, result.code_chunk.line_end));
            report.push_str(&format!("**Language:** {}\n", result.code_chunk.language));
            report.push_str(&format!("**Score:** {:.3}\n\n", result.search_result.score));
            
            // Show code snippet
            report.push_str("**Code:**\n```rust\n");
            let lines: Vec<&str> = result.code_chunk.content.lines().collect();
            let snippet_lines = if lines.len() > 10 {
                &lines[..10]
            } else {
                &lines
            };
            report.push_str(&snippet_lines.join("\n"));
            if lines.len() > 10 {
                report.push_str("\n// ... (truncated)");
            }
            report.push_str("\n```\n\n");
        }
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_vibe_similarity() {
        let temp_dir = TempDir::new().unwrap();
        let finder = VibeFinder::new(temp_dir.path()).unwrap();
        
        let chat_text = "I want to implement geometric attention";
        let code_content = "fn geometric_attention() { /* implementation */ }";
        
        let code_chunk = CodeChunk {
            language: "rust".to_string(),
            content: code_content.to_string(),
            line_start: 1,
            line_end: 1,
            content_hash: "test".to_string(),
            token_count: 5,
            line_count: 1,
            char_count: code_content.len(),
            test_result: "".to_string(),
        };
        
        let similarity = finder.calculate_vibe_similarity(chat_text, &code_chunk);
        assert!(similarity > 0.0);
    }
} 