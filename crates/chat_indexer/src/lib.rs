use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatDocument {
    pub file_path: String,
    pub content: String,
    pub metadata: ChatMetadata,
    pub easter_eggs: Vec<String>,
    pub vibe_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMetadata {
    pub title: String,
    pub date: Option<String>,
    pub participants: Vec<String>,
    pub topics: Vec<String>,
    pub theorem_count: usize,
    pub ontology_mentions: Vec<String>,
}

pub struct ChatIndexer {
    input_dir: PathBuf,
    output_dir: PathBuf,
    documents: Vec<ChatDocument>,
}

impl ChatIndexer {
    pub fn new(input_dir: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            input_dir,
            output_dir,
            documents: Vec::new(),
        }
    }

    pub fn index_all(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Scanning for chat files in {:?}...", self.input_dir);
        
        // Create output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir)?;
        
        // Walk through all markdown files in the input directory
        for entry in WalkDir::new(&self.input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            let file_path = entry.path();
            println!("Processing: {:?}", file_path);
            
            if let Ok(document) = self.process_chat_file(file_path) {
                self.documents.push(document);
            }
        }
        
        // Save the index
        self.save_index()?;
        
        println!("Indexed {} chat documents", self.documents.len());
        Ok(())
    }

    fn process_chat_file(&self, file_path: &Path) -> Result<ChatDocument, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Extract metadata from content
        let metadata = self.extract_metadata(&content, &file_name);
        
        // Find easter eggs
        let easter_eggs = self.find_easter_eggs(&content);
        
        // Calculate vibe score
        let vibe_score = self.calculate_vibe_score(&content);
        
        Ok(ChatDocument {
            file_path: file_path.to_string_lossy().to_string(),
            content,
            metadata,
            easter_eggs,
            vibe_score,
        })
    }

    fn extract_metadata(&self, content: &str, file_name: &str) -> ChatMetadata {
        let mut metadata = ChatMetadata {
            title: file_name.to_string(),
            date: None,
            participants: Vec::new(),
            topics: Vec::new(),
            theorem_count: 0,
            ontology_mentions: Vec::new(),
        };
        
        // Extract participants (User, Grok AI, etc.)
        let lines: Vec<&str> = content.lines().collect();
        for line in &lines {
            if line.starts_with("## User") {
                metadata.participants.push("User".to_string());
            } else if line.starts_with("## Grok AI") {
                metadata.participants.push("Grok AI".to_string());
            }
        }
        metadata.participants.sort();
        metadata.participants.dedup();
        
        // Count theorems
        metadata.theorem_count = content.matches("Theorem").count();
        
        // Find ontology mentions
        let ontology_patterns = [
            "0,1,2,3,5,7", "[0,1,2,3,5,7]", "zero knowledge", "zkp", "zk-rollup",
            "bipartite graph", "mycelium", "seashell", "SOLFUNMEME", "harmonic",
            "tarot deck", "meme-NFT", "vibe", "secret sauce"
        ];
        
        for pattern in &ontology_patterns {
            if content.to_lowercase().contains(&pattern.to_lowercase()) {
                metadata.ontology_mentions.push(pattern.to_string());
            }
        }
        
        // Extract topics from content
        let topic_keywords = [
            "matrix", "diagonalization", "reflection", "market quote", "compressed NFT",
            "pump.fun", "solana", "blockchain", "cryptography", "mathematics",
            "combinatorics", "factorial", "prime", "harmonic", "frequency",
            "cultural", "meme", "viral", "community", "decentralized"
        ];
        
        for keyword in &topic_keywords {
            if content.to_lowercase().contains(&keyword.to_lowercase()) {
                metadata.topics.push(keyword.to_string());
            }
        }
        metadata.topics.sort();
        metadata.topics.dedup();
        
        metadata
    }

    fn find_easter_eggs(&self, content: &str) -> Vec<String> {
        let mut eggs = Vec::new();
        
        // Look for specific easter egg patterns
        let egg_patterns = [
            ("theorem", r"Theorem \d+:"),
            ("ontology", r"\[0,1,2,3,5,7\]"),
            ("vibe", r"vibe.*true"),
            ("secret sauce", r"secret sauce"),
            ("mycelium", r"mycelium.*web"),
            ("seashell", r"seashell.*currency"),
            ("harmonic", r"harmonic.*properties"),
            ("zero knowledge", r"zero.*knowledge.*proof"),
            ("bipartite", r"bipartite.*graph"),
            ("compressed NFT", r"compressed.*NFT"),
            ("market quote", r"market.*quote.*Q"),
            ("reflection", r"reflection.*diagonalization"),
        ];
        
        for (egg_type, pattern) in &egg_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if regex.is_match(content) {
                    eggs.push(format!("{}: Found in content", egg_type));
                }
            }
        }
        
        // Look for mathematical expressions
        let math_patterns = [
            r"n!",
            r"mod\s*\d+",
            r"Q\s*=\s*\d+",
            r"w_{ij}",
            r"G\s*=\s*\(V_1,V_2,E\)",
        ];
        
        for pattern in &math_patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if regex.is_match(content) {
                    eggs.push(format!("mathematical: Found pattern {}", pattern));
                }
            }
        }
        
        // Look for cultural references
        let cultural_patterns = [
            "tarot deck",
            "meme-NFT",
            "pump.fun",
            "SOLFUNMEME",
            "GOAT",
            "POPCAT",
            "WIF",
            "MOODENG",
            "BONK",
            "Raydium",
            "Solana",
        ];
        
        for pattern in &cultural_patterns {
            if content.to_lowercase().contains(&pattern.to_lowercase()) {
                eggs.push(format!("cultural: Found reference to {}", pattern));
            }
        }
        
        eggs
    }

    fn calculate_vibe_score(&self, content: &str) -> f64 {
        let mut score = 0.0;
        
        // Positive vibe indicators
        let positive_patterns = [
            ("theorem", 10.0),
            ("vibe", 5.0),
            ("harmonic", 3.0),
            ("mycelium", 3.0),
            ("secret sauce", 5.0),
            ("zero knowledge", 4.0),
            ("bipartite graph", 3.0),
            ("compressed NFT", 3.0),
            ("market quote", 2.0),
            ("reflection", 2.0),
            ("diagonalization", 2.0),
            ("matrix", 1.0),
            ("factorial", 1.0),
            ("prime", 1.0),
            ("seashell", 2.0),
            ("SOLFUNMEME", 3.0),
            ("pump.fun", 2.0),
            ("solana", 1.0),
            ("blockchain", 1.0),
            ("cryptography", 1.0),
        ];
        
        for (pattern, weight) in &positive_patterns {
            let count = content.to_lowercase().matches(pattern).count();
            score += count as f64 * weight;
        }
        
        // Normalize score (0-100)
        score = (score / 10.0).min(100.0);
        
        score
    }

    fn save_index(&self) -> Result<(), Box<dyn Error>> {
        let index_file = self.output_dir.join("chat_index.json");
        let index_data = serde_json::to_string_pretty(&self.documents)?;
        fs::write(index_file, index_data)?;
        
        // Save summary statistics
        let summary = self.generate_summary();
        let summary_file = self.output_dir.join("summary.json");
        let summary_data = serde_json::to_string_pretty(&summary)?;
        fs::write(summary_file, summary_data)?;
        
        Ok(())
    }

    fn generate_summary(&self) -> HashMap<String, serde_json::Value> {
        let mut summary = HashMap::new();
        
        summary.insert("total_documents".to_string(), serde_json::Value::Number(
            serde_json::Number::from(self.documents.len())
        ));
        
        summary.insert("total_easter_eggs".to_string(), serde_json::Value::Number(
            serde_json::Number::from(self.documents.iter().map(|d| d.easter_eggs.len()).sum::<usize>())
        ));
        
        let avg_vibe = self.documents.iter().map(|d| d.vibe_score).sum::<f64>() / self.documents.len() as f64;
        summary.insert("average_vibe_score".to_string(), serde_json::Value::Number(
            serde_json::Number::from_f64(avg_vibe).unwrap()
        ));
        
        // Count theorem mentions
        let theorem_count = self.documents.iter().map(|d| d.metadata.theorem_count).sum::<usize>();
        summary.insert("total_theorems".to_string(), serde_json::Value::Number(
            serde_json::Number::from(theorem_count)
        ));
        
        // Most common topics
        let mut topic_counts: HashMap<String, usize> = HashMap::new();
        for doc in &self.documents {
            for topic in &doc.metadata.topics {
                *topic_counts.entry(topic.clone()).or_insert(0) += 1;
            }
        }
        
        let mut sorted_topics: Vec<_> = topic_counts.into_iter().collect();
        sorted_topics.sort_by(|a, b| b.1.cmp(&a.1));
        
        let top_topics: Vec<String> = sorted_topics.into_iter()
            .take(10)
            .map(|(topic, _)| topic)
            .collect();
        
        summary.insert("top_topics".to_string(), serde_json::Value::Array(
            top_topics.into_iter().map(serde_json::Value::String).collect()
        ));
        
        summary
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<String>, Box<dyn Error>> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();
        
        for doc in &self.documents {
            let content_lower = doc.content.to_lowercase();
            if content_lower.contains(&query_lower) {
                results.push(format!("{} (vibe: {:.1})", doc.metadata.title, doc.vibe_score));
                if results.len() >= limit {
                    break;
                }
            }
        }
        
        Ok(results)
    }

    pub fn analyze(&self, analysis_type: &str, format: &str) -> Result<String, Box<dyn Error>> {
        match analysis_type {
            "themes" => {
                let mut theme_counts: HashMap<String, usize> = HashMap::new();
                for doc in &self.documents {
                    for topic in &doc.metadata.topics {
                        *theme_counts.entry(topic.clone()).or_insert(0) += 1;
                    }
                }
                
                let mut sorted_themes: Vec<_> = theme_counts.into_iter().collect();
                sorted_themes.sort_by(|a, b| b.1.cmp(&a.1));
                
                if format == "json" {
                    Ok(serde_json::to_string_pretty(&sorted_themes)?)
                } else {
                    let mut result = String::new();
                    for (theme, count) in sorted_themes {
                        result.push_str(&format!("{}: {}\n", theme, count));
                    }
                    Ok(result)
                }
            }
            "easter_eggs" => {
                let mut egg_counts: HashMap<String, usize> = HashMap::new();
                for doc in &self.documents {
                    for egg in &doc.easter_eggs {
                        *egg_counts.entry(egg.clone()).or_insert(0) += 1;
                    }
                }
                
                let mut sorted_eggs: Vec<_> = egg_counts.into_iter().collect();
                sorted_eggs.sort_by(|a, b| b.1.cmp(&a.1));
                
                if format == "json" {
                    Ok(serde_json::to_string_pretty(&sorted_eggs)?)
                } else {
                    let mut result = String::new();
                    for (egg, count) in sorted_eggs {
                        result.push_str(&format!("{}: {}\n", egg, count));
                    }
                    Ok(result)
                }
            }
            "vibe" => {
                let vibe_scores: Vec<f64> = self.documents.iter().map(|d| d.vibe_score).collect();
                let avg_vibe = vibe_scores.iter().sum::<f64>() / vibe_scores.len() as f64;
                let max_vibe = vibe_scores.iter().fold(0.0f32, |a, &b| a.max(b as f32));
                let min_vibe = vibe_scores.iter().fold(f32::INFINITY, |a, &b| a.min(b as f32));
                
                let vibe_stats = serde_json::json!({
                    "average": avg_vibe,
                    "maximum": max_vibe,
                    "minimum": min_vibe,
                    "total_documents": vibe_scores.len()
                });
                
                if format == "json" {
                    Ok(serde_json::to_string_pretty(&vibe_stats)?)
                } else {
                    Ok(format!(
                        "Vibe Analysis:\nAverage: {:.1}\nMaximum: {:.1}\nMinimum: {:.1}\nTotal: {}",
                        avg_vibe, max_vibe, min_vibe, vibe_scores.len()
                    ))
                }
            }
            _ => Err("Unknown analysis type".into())
        }
    }
} 