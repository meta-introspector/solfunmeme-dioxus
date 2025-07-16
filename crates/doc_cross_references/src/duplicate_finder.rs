use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
use syn::{File, Item, ItemFn, ItemStruct, ItemImpl, ItemTrait, parse_file};
use quote::ToTokens;
use std::fs;

/// Canonical representation of code for duplicate detection
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalCode {
    pub kind: CodeKind,
    pub signature: String,
    pub normalized_body: String,
    pub hash: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CodeKind {
    Function,
    Struct,
    Trait,
    Impl,
    Module,
}

/// Location where duplicate code was found
#[derive(Debug, Clone)]
pub struct CodeLocation {
    pub file_path: String,
    pub line_start: usize,
    pub line_end: usize,
    pub function_name: Option<String>,
}

/// Duplicate code group
#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    pub canonical: CanonicalCode,
    pub locations: Vec<CodeLocation>,
    pub similarity_score: f32,
}

/// Duplicate finder that creates canonical representations
pub struct DuplicateFinder {
    code_index: HashMap<u64, Vec<CodeLocation>>,
    canonical_index: HashMap<String, CanonicalCode>,
}

impl DuplicateFinder {
    pub fn new() -> Self {
        Self {
            code_index: HashMap::new(),
            canonical_index: HashMap::new(),
        }
    }

    /// Scan a directory for Rust files and find duplicates
    pub fn scan_directory(&mut self, path: &Path) -> Result<Vec<DuplicateGroup>> {
        let mut duplicates = Vec::new();
        
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    self.scan_file(entry.path())?;
                }
            }
        }

        // Group duplicates by canonical representation
        for (hash, locations) in &self.code_index {
            if locations.len() > 1 {
                if let Some(canonical) = self.canonical_index.get(&hash.to_string()) {
                    let group = DuplicateGroup {
                        canonical: canonical.clone(),
                        locations: locations.clone(),
                        similarity_score: 1.0, // Exact duplicates
                    };
                    duplicates.push(group);
                }
            }
        }

        Ok(duplicates)
    }

    /// Scan a single Rust file for code elements
    fn scan_file(&mut self, file_path: &Path) -> Result<()> {
        let content = fs::read_to_string(file_path)?;
        
        match parse_file(&content) {
            Ok(ast) => {
                for item in ast.items {
                    self.process_item(&item, file_path)?;
                }
            }
            Err(e) => {
                eprintln!("Failed to parse {}: {}", file_path.display(), e);
            }
        }
        
        Ok(())
    }

    /// Process a single AST item
    fn process_item(&mut self, item: &Item, file_path: &Path) -> Result<()> {
        match item {
            Item::Fn(func) => {
                let canonical = self.create_function_canonical(func);
                let location = self.create_location(file_path, func);
                self.index_canonical(canonical, location);
            }
            Item::Struct(struct_item) => {
                let canonical = self.create_struct_canonical(struct_item);
                let location = self.create_location(file_path, struct_item);
                self.index_canonical(canonical, location);
            }
            Item::Trait(trait_item) => {
                let canonical = self.create_trait_canonical(trait_item);
                let location = self.create_location(file_path, trait_item);
                self.index_canonical(canonical, location);
            }
            Item::Impl(impl_item) => {
                let canonical = self.create_impl_canonical(impl_item);
                let location = self.create_location(file_path, impl_item);
                self.index_canonical(canonical, location);
            }
            _ => {}
        }
        Ok(())
    }

    /// Create canonical representation for a function
    fn create_function_canonical(&self, func: &ItemFn) -> CanonicalCode {
        let signature = self.normalize_signature(&func.sig.to_token_stream().to_string());
        let body = self.normalize_body(&func.block.to_token_stream().to_string());
        let hash = self.compute_hash(&signature, &body);
        
        CanonicalCode {
            kind: CodeKind::Function,
            signature,
            normalized_body: body,
            hash,
        }
    }

    /// Create canonical representation for a struct
    fn create_struct_canonical(&self, struct_item: &ItemStruct) -> CanonicalCode {
        let signature = format!("struct {}", struct_item.ident);
        let body = self.normalize_body(&struct_item.fields.to_token_stream().to_string());
        let hash = self.compute_hash(&signature, &body);
        
        CanonicalCode {
            kind: CodeKind::Struct,
            signature,
            normalized_body: body,
            hash,
        }
    }

    /// Create canonical representation for a trait
    fn create_trait_canonical(&self, trait_item: &ItemTrait) -> CanonicalCode {
        let signature = format!("trait {}", trait_item.ident);
        let body = self.normalize_body(&format!("{:?}", trait_item.items));
        let hash = self.compute_hash(&signature, &body);
        
        CanonicalCode {
            kind: CodeKind::Trait,
            signature,
            normalized_body: body,
            hash,
        }
    }

    /// Create canonical representation for an impl
    fn create_impl_canonical(&self, impl_item: &ItemImpl) -> CanonicalCode {
        let signature = format!("impl {}", impl_item.self_ty.to_token_stream());
        let body = self.normalize_body(&format!("{:?}", impl_item.items));
        let hash = self.compute_hash(&signature, &body);
        
        CanonicalCode {
            kind: CodeKind::Impl,
            signature,
            normalized_body: body,
            hash,
        }
    }

    /// Normalize signature by removing whitespace and comments
    fn normalize_signature(&self, signature: &str) -> String {
        signature
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Normalize code body by removing whitespace, comments, and variable names
    fn normalize_body(&self, body: &str) -> String {
        // Remove comments
        let without_comments = body
            .lines()
            .map(|line| {
                if let Some(comment_start) = line.find("//") {
                    &line[..comment_start]
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Remove extra whitespace
        let normalized = without_comments
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        // TODO: Add more sophisticated normalization:
        // - Replace variable names with placeholders
        // - Normalize function calls
        // - Standardize formatting

        normalized
    }

    /// Compute hash for canonical representation
    fn compute_hash(&self, signature: &str, body: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        signature.hash(&mut hasher);
        body.hash(&mut hasher);
        hasher.finish()
    }

    /// Create location information for an item
    fn create_location(&self, file_path: &Path, item: &impl syn::spanned::Spanned) -> CodeLocation {
        let _span = item.span();
        // For now, use simple line counting since span methods aren't available
        let start_line = 1; // Placeholder
        let end_line = 1;   // Placeholder
        
        CodeLocation {
            file_path: file_path.to_string_lossy().to_string(),
            line_start: start_line,
            line_end: end_line,
            function_name: None, // TODO: Extract function name
        }
    }

    /// Index canonical representation
    fn index_canonical(&mut self, canonical: CanonicalCode, location: CodeLocation) {
        let hash_key = canonical.hash.to_string();
        
        // Store canonical representation
        self.canonical_index.insert(hash_key.clone(), canonical.clone());
        
        // Index location
        self.code_index.entry(canonical.hash).or_insert_with(Vec::new).push(location);
    }

    /// Find similar code (not exact duplicates)
    pub fn find_similar_code(&self, target_code: &str, similarity_threshold: f32) -> Vec<DuplicateGroup> {
        let target_normalized = self.normalize_body(target_code);
        let mut similar_groups = Vec::new();

        for (_hash, canonical) in &self.canonical_index {
            let similarity = self.compute_similarity(&target_normalized, &canonical.normalized_body);
            
            if similarity >= similarity_threshold {
                if let Some(locations) = self.code_index.get(&canonical.hash) {
                    let group = DuplicateGroup {
                        canonical: canonical.clone(),
                        locations: locations.clone(),
                        similarity_score: similarity,
                    };
                    similar_groups.push(group);
                }
            }
        }

        // Sort by similarity score
        similar_groups.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());
        similar_groups
    }

    /// Compute similarity between two code bodies
    fn compute_similarity(&self, code1: &str, code2: &str) -> f32 {
        // Simple Jaccard similarity on token sets
        let tokens1: std::collections::HashSet<&str> = code1.split_whitespace().collect();
        let tokens2: std::collections::HashSet<&str> = code2.split_whitespace().collect();
        
        let intersection = tokens1.intersection(&tokens2).count();
        let union = tokens1.union(&tokens2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    /// Generate a report of all duplicates
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("# Code Duplicate Analysis Report\n\n");
        
        let mut total_duplicates = 0;
        let mut total_files = 0;
        
        for (hash, locations) in &self.code_index {
            if locations.len() > 1 {
                total_duplicates += locations.len() - 1;
                total_files += locations.len();
                
                if let Some(canonical) = self.canonical_index.get(&hash.to_string()) {
                    report.push_str(&format!("## {} ({} instances)\n", canonical.signature, locations.len()));
                    report.push_str(&format!("**Type:** {:?}\n", canonical.kind));
                    report.push_str(&format!("**Hash:** {}\n\n", canonical.hash));
                    
                    report.push_str("**Locations:**\n");
                    for location in locations {
                        report.push_str(&format!("- {}:{}-{}\n", 
                            location.file_path, location.line_start, location.line_end));
                    }
                    report.push_str("\n");
                }
            }
        }
        
        report.push_str(&format!("## Summary\n"));
        report.push_str(&format!("- Total duplicate instances: {}\n", total_duplicates));
        report.push_str(&format!("- Total files with duplicates: {}\n", total_files));
        report.push_str(&format!("- Unique duplicate patterns: {}\n", 
            self.code_index.values().filter(|locs| locs.len() > 1).count()));
        
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_body() {
        let finder = DuplicateFinder::new();
        let code = "fn test() { let x = 1; // comment\n let y = 2; }";
        let normalized = finder.normalize_body(code);
        assert!(!normalized.contains("comment"));
        assert!(normalized.contains("let x = 1"));
    }

    #[test]
    fn test_compute_similarity() {
        let finder = DuplicateFinder::new();
        let code1 = "fn test() { let x = 1; let y = 2; }";
        let code2 = "fn test() { let a = 1; let b = 2; }";
        let similarity = finder.compute_similarity(code1, code2);
        assert!(similarity > 0.5); // Should be similar
    }
} 