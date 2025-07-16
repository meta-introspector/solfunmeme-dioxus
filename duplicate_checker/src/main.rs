use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

#[derive(Debug, Clone)]
struct CodeSnippet {
    file_path: String,
    line_start: usize,
    line_end: usize,
    content: String,
    normalized_content: String,
    hash: String,
}

struct DuplicateChecker {
    snippets: Vec<CodeSnippet>,
    hash_map: HashMap<String, Vec<usize>>,
}

impl DuplicateChecker {
    fn new() -> Self {
        Self {
            snippets: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    fn analyze_directory(&mut self, dir_path: &Path) -> Result<(), Box<dyn Error>> {
        println!("🔍 Analyzing directory: {:?}", dir_path);
        
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "rs" || ext == "toml" || ext == "md" {
                    self.analyze_file(path)?;
                }
            }
        }
        
        Ok(())
    }

    fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        let snippets = self.extract_snippets(&lines, file_path);
        
        for mut snippet in snippets {
            let hash = self.compute_hash(&snippet.normalized_content);
            snippet.hash = hash.clone();
            
            self.hash_map.entry(hash).or_insert_with(Vec::new).push(self.snippets.len());
            self.snippets.push(snippet);
        }
        
        Ok(())
    }

    fn extract_snippets(&self, lines: &[&str], file_path: &Path) -> Vec<CodeSnippet> {
        let mut snippets = Vec::new();
        let mut current_snippet = String::new();
        let mut start_line = 0;
        let mut in_code_block = false;
        
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Detect code blocks in markdown
            if trimmed.starts_with("```") {
                if in_code_block {
                    // End of code block
                    if !current_snippet.trim().is_empty() {
                        snippets.push(CodeSnippet {
                            file_path: file_path.to_string_lossy().to_string(),
                            line_start: start_line + 1,
                            line_end: i + 1,
                            content: current_snippet.clone(),
                            normalized_content: self.normalize_content(&current_snippet),
                            hash: String::new(),
                        });
                    }
                    current_snippet.clear();
                    in_code_block = false;
                } else {
                    // Start of code block
                    in_code_block = true;
                    start_line = i;
                }
            } else if in_code_block {
                current_snippet.push_str(line);
                current_snippet.push('\n');
            } else {
                // For Rust files, extract functions, structs, etc.
                if self.is_code_start(trimmed) {
                    if !current_snippet.trim().is_empty() {
                        snippets.push(CodeSnippet {
                            file_path: file_path.to_string_lossy().to_string(),
                            line_start: start_line + 1,
                            line_end: i + 1,
                            content: current_snippet.clone(),
                            normalized_content: self.normalize_content(&current_snippet),
                            hash: String::new(),
                        });
                    }
                    current_snippet = line.to_string();
                    current_snippet.push('\n');
                    start_line = i;
                } else if !current_snippet.is_empty() {
                    current_snippet.push_str(line);
                    current_snippet.push('\n');
                }
            }
        }
        
        // Add final snippet
        if !current_snippet.trim().is_empty() {
            snippets.push(CodeSnippet {
                file_path: file_path.to_string_lossy().to_string(),
                line_start: start_line + 1,
                line_end: lines.len(),
                content: current_snippet.clone(),
                normalized_content: self.normalize_content(&current_snippet),
                hash: String::new(),
            });
        }
        
        snippets
    }

    fn is_code_start(&self, line: &str) -> bool {
        line.starts_with("pub fn ") || 
        line.starts_with("fn ") || 
        line.starts_with("pub struct ") || 
        line.starts_with("struct ") || 
        line.starts_with("pub enum ") || 
        line.starts_with("enum ") || 
        line.starts_with("impl ") || 
        line.starts_with("pub trait ") || 
        line.starts_with("trait ") ||
        line.starts_with("mod ") ||
        line.starts_with("pub mod ")
    }

    fn normalize_content(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn compute_hash(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn prove_no_duplicates(&self, module_path: &Path) -> bool {
        let module_snippets: Vec<&CodeSnippet> = self.snippets.iter()
            .filter(|s| s.file_path.contains(module_path.to_string_lossy().as_ref()))
            .collect();
        
        if module_snippets.is_empty() {
            println!("✅ No code found in module: {:?}", module_path);
            return true;
        }
        
        // Check for exact duplicates within the module
        let mut hashes = std::collections::HashSet::new();
        for snippet in &module_snippets {
            if !hashes.insert(&snippet.hash) {
                println!("❌ Found duplicate code in module: {:?}", module_path);
                println!("   Duplicate: {}", snippet.content.lines().next().unwrap_or(""));
                return false;
            }
        }
        
        // Check for similar code within the module
        for i in 0..module_snippets.len() {
            for j in (i + 1)..module_snippets.len() {
                let similarity = self.calculate_similarity(module_snippets[i], module_snippets[j]);
                if similarity > 0.8 {
                    println!("⚠️  Found similar code in module: {:?} (similarity: {:.2})", module_path, similarity);
                    println!("   File 1: {}:{}", module_snippets[i].file_path, module_snippets[i].line_start);
                    println!("   File 2: {}:{}", module_snippets[j].file_path, module_snippets[j].line_start);
                    return false;
                }
            }
        }
        
        println!("✅ No duplicates found in module: {:?}", module_path);
        true
    }

    fn calculate_similarity(&self, snippet1: &CodeSnippet, snippet2: &CodeSnippet) -> f64 {
        let words1: Vec<&str> = snippet1.normalized_content.split_whitespace().collect();
        let words2: Vec<&str> = snippet2.normalized_content.split_whitespace().collect();
        
        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }
        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }
        
        let common_words: Vec<&&str> = words1.iter()
            .filter(|word| words2.contains(word))
            .collect();
        
        let union_size = words1.len() + words2.len() - common_words.len();
        if union_size == 0 {
            1.0
        } else {
            common_words.len() as f64 / union_size as f64
        }
    }

    fn find_all_duplicates(&self) -> Vec<Vec<&CodeSnippet>> {
        let mut duplicates = Vec::new();
        let mut processed_hashes = std::collections::HashSet::new();
        
        for (hash, indices) in &self.hash_map {
            if indices.len() > 1 && !processed_hashes.contains(hash) {
                let mut group = Vec::new();
                for &index in indices {
                    group.push(&self.snippets[index]);
                }
                duplicates.push(group);
                processed_hashes.insert(hash.clone());
            }
        }
        
        duplicates
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("🔍 Solfunmeme Duplicate Code Prover");
    println!("=====================================");
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <directory>", args[0]);
        println!("Example: {} src/bin", args[0]);
        return Ok(());
    }
    
    let dir_path = PathBuf::from(&args[1]);
    
    if !dir_path.exists() {
        println!("❌ Directory does not exist: {:?}", dir_path);
        return Ok(());
    }
    
    let mut checker = DuplicateChecker::new();
    checker.analyze_directory(&dir_path)?;
    
    println!("\n📊 Analysis Results:");
    println!("Total snippets found: {}", checker.snippets.len());
    
    // Find all duplicates across the codebase
    let all_duplicates = checker.find_all_duplicates();
    if !all_duplicates.is_empty() {
        println!("\n⚠️  Found {} duplicate groups across the codebase:", all_duplicates.len());
        for (i, group) in all_duplicates.iter().enumerate() {
            println!("\nDuplicate Group {}:", i + 1);
            for snippet in group {
                println!("  {}:{} - {}", snippet.file_path, snippet.line_start, 
                    snippet.content.lines().next().unwrap_or(""));
            }
        }
    }
    
    // Check specific modules for duplicates
    let modules_to_check = vec![
        "src/bin",
        "crates/task_manager", 
        "crates/solfunmeme_core_logic",
        "crates/solfunmeme_views",
        "crates/solfunmeme_state"
    ];
    
    println!("\n🔍 Module Duplicate Proof:");
    println!("==========================");
    
    let mut all_clean = true;
    for module in modules_to_check {
        let module_path = PathBuf::from(module);
        if module_path.exists() {
            if !checker.prove_no_duplicates(&module_path) {
                all_clean = false;
            }
        } else {
            println!("⚠️  Module not found: {:?}", module_path);
        }
    }
    
    println!("\n🎯 Final Result:");
    if all_clean {
        println!("✅ All modules are duplicate-free! The vibe is proven true.");
        println!("🎉 Your codebase maintains its unique identity and avoids redundancy.");
    } else {
        println!("❌ Duplicates found. The vibe needs work.");
        println!("💡 Consider refactoring duplicate code into shared utilities.");
    }
    
    Ok(())
} 