use tantivy::schema::TantivyDocument;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexWriter, IndexSettings};
use tantivy::directory::MmapDirectory;

use solfunmeme_function_analysis::CodeChunk;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub language: String,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
    pub content_hash: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: String,
    pub score: f32,
}

pub struct SearchIndex {
    index: Index,
    schema: Schema,
    writer: IndexWriter,
}

impl SearchIndex {
    pub fn new(index_path: &Path) -> Result<Self> {
        // Create schema
        let mut schema_builder = Schema::builder();
        
        // Fields for indexing
        let language_field = schema_builder.add_text_field("language", TEXT | STORED);
        let content_field = schema_builder.add_text_field("content", TEXT | STORED);
        let line_start_field = schema_builder.add_u64_field("line_start", STORED);
        let line_end_field = schema_builder.add_u64_field("line_end", STORED);
        let content_hash_field = schema_builder.add_text_field("content_hash", TEXT | STORED);
        let token_count_field = schema_builder.add_u64_field("token_count", STORED);
        let line_count_field = schema_builder.add_u64_field("line_count", STORED);
        let char_count_field = schema_builder.add_u64_field("char_count", STORED);
        let test_result_field = schema_builder.add_text_field("test_result", TEXT | STORED);
        let embedding_field = schema_builder.add_bytes_field("embedding", STORED);
        
        let schema = schema_builder.build();
        
        // Create or open index
        let index = match Index::open(MmapDirectory::open(index_path)?) {
            Ok(existing_index) => {
                if existing_index.schema() == schema {
                    eprintln!("[INFO] Opened existing Tantivy index at: {}", index_path.display());
                    existing_index
                } else {
                    eprintln!("[WARN] Existing index schema mismatch at: {}. Recreating index.", index_path.display());
                    std::fs::remove_dir_all(index_path)
                        .map_err(|e| anyhow::anyhow!("Failed to remove old index directory {}: {}", index_path.display(), e))?;
                    std::fs::create_dir_all(index_path)
                        .map_err(|e| anyhow::anyhow!("Failed to create index directory {}: {}", index_path.display(), e))?;
                    let directory = MmapDirectory::open(index_path)
                        .map_err(|e| anyhow::anyhow!("Failed to open MmapDirectory at {}: {}", index_path.display(), e))?;
                    Index::create(directory, schema.clone(), IndexSettings::default())
                        .map_err(|e| anyhow::anyhow!("Failed to create Tantivy index at {}: {}", index_path.display(), e))?
                }
            },
            Err(e) => {
                eprintln!("[WARN] Failed to open existing Tantivy index at {}: {}. Recreating index.", index_path.display(), e);
                if index_path.exists() {
                    std::fs::remove_dir_all(index_path)
                        .map_err(|e| anyhow::anyhow!("Failed to remove old index directory {}: {}", index_path.display(), e))?;
                }
                std::fs::create_dir_all(index_path)
                    .map_err(|e| anyhow::anyhow!("Failed to create index directory {}: {}", index_path.display(), e))?;
                let directory = MmapDirectory::open(index_path)
                    .map_err(|e| anyhow::anyhow!("Failed to open MmapDirectory at {}: {}", index_path.display(), e))?;
                Index::create(directory, schema.clone(), IndexSettings::default())
                    .map_err(|e| anyhow::anyhow!("Failed to create Tantivy index at {}: {}", index_path.display(), e))?
            }
        };
        
        let writer = index.writer(50_000_000)?; // 50MB buffer
        
        Ok(Self {
            index,
            schema,
            writer,
        })
    }
    
    pub fn add_chunk(&mut self, chunk: &CodeChunk) -> Result<()> {
        let language_field = self.schema.get_field("language")?;
        let content_field = self.schema.get_field("content")?;
        let line_start_field = self.schema.get_field("line_start")?;
        let line_end_field = self.schema.get_field("line_end")?;
        let content_hash_field = self.schema.get_field("content_hash")?;
        let token_count_field = self.schema.get_field("token_count")?;
        let line_count_field = self.schema.get_field("line_count")?;
        let char_count_field = self.schema.get_field("char_count")?;
        let test_result_field = self.schema.get_field("test_result")?;
        let embedding_field = self.schema.get_field("embedding")?;

        let embedding_bytes: Vec<u8> = chunk.embedding.iter().flat_map(|f| f.to_le_bytes()).collect();

        let doc = doc!(
            language_field => chunk.language.clone(),
            content_field => chunk.content.clone(),
            line_start_field => chunk.line_start as u64,
            line_end_field => chunk.line_end as u64,
            content_hash_field => chunk.content_hash.clone(),
            token_count_field => chunk.token_count as u64,
            line_count_field => chunk.line_count as u64,
            char_count_field => chunk.char_count as u64,
            test_result_field => chunk.test_result.clone(),
            embedding_field => embedding_bytes,
        );
        
        self.writer.add_document(doc)?;
        Ok(())
    }
    
    pub fn commit(&mut self) -> Result<()> {
        self.writer.commit()?;
        Ok(())
    }
    
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        
        let query_parser = QueryParser::for_index(&self.index, vec![
            self.schema.get_field("content")?,
            self.schema.get_field("path")?,
            self.schema.get_field("chunk_type")?,
        ]);
        
        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;
        
        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            
            let language = doc
                .get_first(self.schema.get_field("language")?)
                .and_then(|v| v.as_value().as_str())
                .unwrap_or("")
                .to_string();
                
            let content = doc
                .get_first(self.schema.get_field("content")?)
                .and_then(|v| v.as_value().as_str())
                .unwrap_or("")
                .to_string();
                
            let line_start = doc
                .get_first(self.schema.get_field("line_start")?)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;
                
            let line_end = doc
                .get_first(self.schema.get_field("line_end")?)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            let content_hash = doc
                .get_first(self.schema.get_field("content_hash")?)
                .and_then(|v| v.as_value().as_str())
                .unwrap_or("")
                .to_string();

            let token_count = doc
                .get_first(self.schema.get_field("token_count")?)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            let line_count = doc
                .get_first(self.schema.get_field("line_count")?)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            let char_count = doc
                .get_first(self.schema.get_field("char_count")?)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            let test_result = doc
                .get_first(self.schema.get_field("test_result")?)
                .and_then(|v| v.as_value().as_str())
                .unwrap_or("")
                .to_string();
            
            results.push(SearchResult {
                language,
                content,
                line_start,
                line_end,
                content_hash,
                token_count,
                line_count,
                char_count,
                test_result,
                score,
            });
        }
        
        Ok(results)
    }
    
    pub fn search_by_emoji(&self, emoji: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query = format!("emoji:{}", emoji);
        self.search(&query, limit)
    }
    
    pub fn search_by_path(&self, path_pattern: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let query = format!("path:*{}*", path_pattern);
        self.search(&query, limit)
    }
    
    pub fn get_stats_by_field(&self, field_name: &str) -> Result<HashMap<String, usize>> {
        let reader = self.index.reader()?;
        let mut stats = HashMap::new();
        let field = self.schema.get_field(field_name)?;

        let searcher = reader.searcher();
        for segment_reader in searcher.segment_readers() {
            let inverted_index = segment_reader.inverted_index(field)?;
            let term_dict = inverted_index.terms();
            let mut term_stream = term_dict.stream()?;
            while let Some((term, _doc_freq)) = term_stream.next() {
                let term_str = String::from_utf8_lossy(term).to_string();
                *stats.entry(term_str.to_string()).or_insert(0) += 1;
            }
        }

        Ok(stats)
    }

    pub fn get_stats_by_emojis(&self) -> Result<HashMap<String, usize>> {
        self.get_stats_by_field("emoji")
    }

    pub fn get_stats_by_terms(&self) -> Result<HashMap<String, usize>> {
        self.get_stats_by_field("content")
    }

    pub fn get_stats_by_hex_codes(&self) -> Result<HashMap<String, usize>> {
        // This is a placeholder. Actual implementation would involve extracting hex codes.
        Ok(HashMap::new())
    }
}

// Bag of words functionality
pub struct BagOfWords {
    word_counts: HashMap<String, usize>,
    total_words: usize,
}

impl BagOfWords {
    pub fn new() -> Self {
        Self {
            word_counts: HashMap::new(),
            total_words: 0,
        }
    }
    
    pub fn add_text(&mut self, text: &str) {
        // Simple tokenization - split on whitespace and punctuation
        let words: Vec<&str> = text
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|word| !word.is_empty())
            .collect();
            
        for word in words {
            let word = word.to_lowercase();
            *self.word_counts.entry(word).or_insert(0) += 1;
            self.total_words += 1;
        }
    }
    
    pub fn get_word_frequency(&self, word: &str) -> f64 {
        let word = word.to_lowercase();
        let count = self.word_counts.get(&word).unwrap_or(&0);
        if self.total_words > 0 {
            *count as f64 / self.total_words as f64
        } else {
            0.0
        }
    }
    
    pub fn get_top_words(&self, limit: usize) -> Vec<(String, usize)> {
        let mut words: Vec<(String, usize)> = self.word_counts
            .iter()
            .map(|(word, count)| (word.clone(), *count))
            .collect();
            
        words.sort_by(|a, b| b.1.cmp(&a.1));
        words.truncate(limit);
        words
    }
    
    pub fn similarity(&self, other: &BagOfWords) -> f64 {
        let mut similarity = 0.0;
        let mut total_weight = 0.0;
        
        for (word, count) in &self.word_counts {
            let other_count = other.word_counts.get(word).unwrap_or(&0);
            let weight = (*count + *other_count) as f64;
            similarity += weight * (1.0 - (*count as f64 - *other_count as f64).abs() / weight);
            total_weight += weight;
        }
        
        if total_weight > 0.0 {
            similarity / total_weight
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_bag_of_words() {
        let mut bow = BagOfWords::new();
        bow.add_text("hello world hello rust");
        
        assert_eq!(bow.get_word_frequency("hello"), 0.5);
        assert_eq!(bow.get_word_frequency("world"), 0.25);
        assert_eq!(bow.get_word_frequency("rust"), 0.25);
        
        let top_words = bow.get_top_words(2);
        assert_eq!(top_words[0], ("hello".to_string(), 2));
    }
    
    #[test]
    fn test_search_index() -> Result<()> {
        let temp_dir = tempdir()?;
        let mut index = SearchIndex::new(temp_dir.path())?;
        
        // Add a test chunk
        let chunk = CodeChunk {
            path: "test.rs".to_string(),
            content: "fn hello() { println!(\"world\"); }".to_string(),
            emoji: "🦀⚙️".to_string(),
            line_start: 1,
            line_end: 1,
            chunk_type: "function".to_string(),
        };
        
        index.add_chunk(&chunk)?;
        index.commit()?;
        
        // Search
        let results = index.search("hello", 10)?;
        assert!(!results.is_empty());
        assert_eq!(results[0].content, chunk.content);
        
        Ok(())
    }
}