use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use git2::{Repository, ObjectType, Oid, Tree, Blob, Commit};
use sha2::{Sha256, Digest};
use emojis;

#[derive(Parser)]
#[command(name = "git_object_indexer")]
#[command(about = "Index Git objects directly using content-addressed hashes", long_about = None)]
struct Cli {
    #[arg(help = "Path to Git repository (default: current directory)")]
    repo_path: Option<PathBuf>,

    #[arg(long, help = "Output file for the Git object index (default: git_object_index.json)")]
    output: Option<String>,

    #[arg(long, help = "Generate emoji hashes for content-addressed identifiers")]
    emoji_hashes: bool,

    #[arg(long, help = "Extract function signatures from source files")]
    extract_functions: bool,

    #[arg(long, help = "Merge identical functions by hash")]
    merge_functions: bool,

    #[arg(long, default_value_t = 0, help = "Verbosity level (0-3)")]
    verbose: u8,

    #[arg(long, help = "Specific commit/branch to analyze (default: HEAD)")]
    reference: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GitObjectIndex {
    repository_path: String,
    total_objects: usize,
    total_blobs: usize,
    total_trees: usize,
    total_commits: usize,
    content_hashes: HashMap<String, ContentHashEntry>,
    function_signatures: HashMap<String, FunctionSignature>,
    emoji_identifiers: HashMap<String, String>,
    git_objects: Vec<GitObjectEntry>,
    indexing_stats: IndexingStats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContentHashEntry {
    git_hash: String,
    content_hash: String,
    emoji_hash: Option<String>,
    object_type: String,
    size: usize,
    path: Option<String>,
    language: Option<String>,
    function_count: Option<usize>,
    duplicate_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FunctionSignature {
    name: String,
    content_hash: String,
    git_hash: String,
    path: String,
    line_start: usize,
    line_end: usize,
    signature: String,
    emoji_identifier: Option<String>,
    duplicate_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GitObjectEntry {
    oid: String,
    object_type: String,
    size: usize,
    content_hash: String,
    emoji_hash: Option<String>,
    path: Option<String>,
    parent_oids: Vec<String>,
    tree_entries: Option<Vec<TreeEntry>>,
    commit_info: Option<CommitInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TreeEntry {
    name: String,
    oid: String,
    object_type: String,
    mode: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommitInfo {
    message: String,
    author: String,
    committer: String,
    timestamp: i64,
    tree_oid: String,
    parent_oids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexingStats {
    start_time: String,
    end_time: String,
    duration_seconds: f64,
    objects_processed: usize,
    functions_extracted: usize,
    duplicates_found: usize,
}

fn generate_content_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

fn generate_emoji_hash(content: &[u8], length: usize) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let hash_bytes = hasher.finalize();
    
    // Use hash bytes to select emojis
    let emoji_list: Vec<&str> = emojis::iter().collect();
    let mut emoji_hash = String::new();
    
    for i in 0..length {
        let byte_index = (hash_bytes[i % 32] as usize) % emoji_list.len();
        emoji_hash.push_str(emoji_list[byte_index]);
    }
    
    emoji_hash
}

fn extract_functions_from_content(content: &str, path: &str) -> Vec<FunctionSignature> {
    let mut functions = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    // Simple Rust function detection (can be enhanced)
    for (line_num, line) in lines.iter().enumerate() {
        let line = line.trim();
        
        // Look for function definitions
        if line.starts_with("fn ") || line.starts_with("pub fn ") || line.starts_with("async fn ") {
            if let Some(func_name) = extract_function_name(line) {
                let function_content = extract_function_content(&lines, line_num);
                let content_hash = generate_content_hash(function_content.as_bytes());
                
                functions.push(FunctionSignature {
                    name: func_name.to_string(),
                    content_hash: content_hash.clone(),
                    git_hash: "".to_string(), // Will be set later
                    path: path.to_string(),
                    line_start: line_num + 1,
                    line_end: line_num + 1, // Will be updated
                    signature: line.to_string(),
                    emoji_identifier: Some(generate_emoji_hash(function_content.as_bytes(), 3)),
                    duplicate_count: 0,
                });
            }
        }
    }
    
    functions
}

fn extract_function_name(line: &str) -> Option<&str> {
    // Simple function name extraction
    if let Some(start) = line.find("fn ") {
        let after_fn = &line[start + 3..];
        if let Some(end) = after_fn.find('(') {
            return Some(&after_fn[..end].trim());
        }
    }
    None
}

fn extract_function_content(lines: &[&str], start_line: usize) -> String {
    let mut content = String::new();
    let mut brace_count = 0;
    let mut in_function = false;
    
    for (i, line) in lines.iter().skip(start_line).enumerate() {
        let line = line.trim();
        
        if !in_function {
            if line.contains("fn ") {
                in_function = true;
                content.push_str(line);
                content.push('\n');
            }
            continue;
        }
        
        content.push_str(line);
        content.push('\n');
        
        // Count braces to find function end
        for ch in line.chars() {
            match ch {
                '{' => brace_count += 1,
                '}' => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        return content;
                    }
                }
                _ => {}
            }
        }
    }
    
    content
}

fn detect_language_from_path(path: &str) -> Option<String> {
    if let Some(ext) = Path::new(path).extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        match ext_str.as_str() {
            "rs" => Some("rust".to_string()),
            "md" => Some("markdown".to_string()),
            "json" => Some("json".to_string()),
            "toml" => Some("toml".to_string()),
            "js" => Some("javascript".to_string()),
            "ts" => Some("typescript".to_string()),
            "py" => Some("python".to_string()),
            "go" => Some("go".to_string()),
            "java" => Some("java".to_string()),
            "c" => Some("c".to_string()),
            "cpp" | "cc" | "cxx" => Some("cpp".to_string()),
            "h" | "hpp" => Some("cpp-header".to_string()),
            _ => None,
        }
    } else {
        None
    }
}

fn process_git_object(
    repo: &Repository,
    oid: Oid,
    path: Option<String>,
    content_hashes: &mut HashMap<String, ContentHashEntry>,
    function_signatures: &mut HashMap<String, FunctionSignature>,
    emoji_identifiers: &mut HashMap<String, String>,
    extract_functions: bool,
    verbose: u8,
) -> Result<GitObjectEntry> {
    let obj = repo.find_object(oid, None)?;
    let content = obj.as_bytes();
    let content_hash = generate_content_hash(content);
    let emoji_hash = if !emoji_identifiers.is_empty() {
        Some(generate_emoji_hash(content, 3))
    } else {
        None
    };
    
    let object_type = match obj.kind() {
        Some(ObjectType::Blob) => "blob".to_string(),
        Some(ObjectType::Tree) => "tree".to_string(),
        Some(ObjectType::Commit) => "commit".to_string(),
        Some(ObjectType::Tag) => "tag".to_string(),
        _ => "unknown".to_string(),
    };
    
    let mut git_entry = GitObjectEntry {
        oid: oid.to_string(),
        object_type: object_type.clone(),
        size: content.len(),
        content_hash: content_hash.clone(),
        emoji_hash: emoji_hash.clone(),
        path,
        parent_oids: Vec::new(),
        tree_entries: None,
        commit_info: None,
    };
    
    // Process based on object type
    match obj.kind() {
        Some(ObjectType::Blob) => {
            let blob = obj.as_blob().unwrap();
            let content_str = String::from_utf8_lossy(blob.content());
            let language = git_entry.path.as_ref().and_then(|p| detect_language_from_path(p));
            
            // Extract functions if requested
            let function_count = if extract_functions && language.as_deref() == Some("rust") {
                let functions = extract_functions_from_content(&content_str, git_entry.path.as_ref().unwrap_or(&"".to_string()));
                for mut func in functions {
                    func.git_hash = oid.to_string();
                    let func_key = format!("{}:{}", func.content_hash, func.name);
                    
                    if let Some(existing) = function_signatures.get_mut(&func_key) {
                        existing.duplicate_count += 1;
                    } else {
                        function_signatures.insert(func_key, func);
                    }
                }
                Some(function_signatures.len())
            } else {
                None
            };
            
            // Update content hash entry
            let entry = content_hashes.entry(content_hash.clone()).or_insert_with(|| ContentHashEntry {
                git_hash: oid.to_string(),
                content_hash: content_hash.clone(),
                emoji_hash: emoji_hash.clone(),
                object_type: object_type.clone(),
                size: content.len(),
                path: git_entry.path.clone(),
                language,
                function_count,
                duplicate_count: 0,
            });
            entry.duplicate_count += 1;
        }
        
        Some(ObjectType::Tree) => {
            let tree = obj.as_tree().unwrap();
            let mut tree_entries = Vec::new();
            
            for entry in tree.iter() {
                let entry_oid = entry.id().to_string();
                let entry_type = match entry.kind() {
                    Some(ObjectType::Blob) => "blob".to_string(),
                    Some(ObjectType::Tree) => "tree".to_string(),
                    _ => "unknown".to_string(),
                };
                
                tree_entries.push(TreeEntry {
                    name: entry.name().unwrap_or("").to_string(),
                    oid: entry_oid,
                    object_type: entry_type,
                    mode: entry.filemode(),
                });
            }
            
            git_entry.tree_entries = Some(tree_entries);
        }
        
        Some(ObjectType::Commit) => {
            let commit = obj.as_commit().unwrap();
            let parents: Vec<String> = commit.parent_ids().map(|id| id.to_string()).collect();
            git_entry.parent_oids = parents.clone();
            
            git_entry.commit_info = Some(CommitInfo {
                message: commit.message().unwrap_or("").to_string(),
                author: format!("{} <{}>", 
                    commit.author().name().unwrap_or(""), 
                    commit.author().email().unwrap_or("")),
                committer: format!("{} <{}>", 
                    commit.committer().name().unwrap_or(""), 
                    commit.committer().email().unwrap_or("")),
                timestamp: commit.time().seconds(),
                tree_oid: commit.tree_id().to_string(),
                parent_oids: parents,
            });
        }
        
        _ => {}
    }
    
    if verbose >= 2 {
        println!("Processed {} object: {} ({} bytes)", object_type, oid, content.len());
    }
    
    Ok(git_entry)
}

fn traverse_git_tree(
    repo: &Repository,
    tree: &Tree,
    base_path: &str,
    content_hashes: &mut HashMap<String, ContentHashEntry>,
    function_signatures: &mut HashMap<String, FunctionSignature>,
    emoji_identifiers: &mut HashMap<String, String>,
    git_objects: &mut Vec<GitObjectEntry>,
    extract_functions: bool,
    verbose: u8,
) -> Result<()> {
    for entry in tree.iter() {
        let entry_name = entry.name().unwrap_or("");
        let current_path = if base_path.is_empty() {
            entry_name.to_string()
        } else {
            format!("{}/{}", base_path, entry_name)
        };
        
        let obj = repo.find_object(entry.id(), None)?;
        let git_entry = process_git_object(
            repo, entry.id(), Some(current_path.clone()),
            content_hashes, function_signatures, emoji_identifiers,
            extract_functions, verbose
        )?;
        
        git_objects.push(git_entry);
        
        // Recursively traverse trees
        if let Some(ObjectType::Tree) = obj.kind() {
            let subtree = obj.as_tree().unwrap();
            traverse_git_tree(
                repo, subtree, &current_path,
                content_hashes, function_signatures, emoji_identifiers,
                git_objects, extract_functions, verbose
            )?;
        }
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let start_time = Instant::now();
    
    let repo_path = cli.repo_path.unwrap_or_else(|| PathBuf::from("."));
    let repo = Repository::open(&repo_path)?;
    
    if cli.verbose >= 1 {
        println!("=== Git Object Indexer ===");
        println!("Repository: {}", repo_path.display());
        println!("Emoji hashes: {}", cli.emoji_hashes);
        println!("Extract functions: {}", cli.extract_functions);
        println!("Merge functions: {}", cli.merge_functions);
    }
    
    let mut content_hashes: HashMap<String, ContentHashEntry> = HashMap::new();
    let mut function_signatures: HashMap<String, FunctionSignature> = HashMap::new();
    let mut emoji_identifiers: HashMap<String, String> = HashMap::new();
    let mut git_objects: Vec<GitObjectEntry> = Vec::new();
    
    // Get the reference to analyze
    let reference = cli.reference.as_deref().unwrap_or("HEAD");
    let rev = repo.revparse_single(reference)?;
    let commit = repo.find_commit(rev.id())?;
    let tree = commit.tree()?;
    
    if cli.verbose >= 1 {
        println!("Analyzing commit: {} ({})", commit.id(), commit.message().unwrap_or("").trim());
    }
    
    // Process the commit object itself
    let commit_entry = process_git_object(
        &repo, commit.id(), None,
        &mut content_hashes, &mut function_signatures, &mut emoji_identifiers,
        cli.extract_functions, cli.verbose
    )?;
    git_objects.push(commit_entry);
    
    // Traverse the tree recursively
    traverse_git_tree(
        &repo, &tree, "",
        &mut content_hashes, &mut function_signatures, &mut emoji_identifiers,
        &mut git_objects, cli.extract_functions, cli.verbose
    )?;
    
    // Generate emoji identifiers for content hashes
    if cli.emoji_hashes {
        for (content_hash, entry) in &mut content_hashes {
            let emoji_id = generate_emoji_hash(content_hash.as_bytes(), 4);
            emoji_identifiers.insert(content_hash.clone(), emoji_id.clone());
            entry.emoji_hash = Some(emoji_id);
        }
    }
    
    // Merge identical functions
    if cli.merge_functions {
        let mut function_groups: HashMap<String, Vec<FunctionSignature>> = HashMap::new();
        
        for (_, func) in &function_signatures {
            function_groups.entry(func.content_hash.clone())
                .or_insert_with(Vec::new)
                .push(func.clone());
        }
        
        if cli.verbose >= 1 {
            println!("Function groups by content hash: {}", function_groups.len());
            for (hash, group) in &function_groups {
                if group.len() > 1 {
                    println!("  {}: {} duplicates", hash, group.len());
                }
            }
        }
    }
    
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    
    let index = GitObjectIndex {
        repository_path: repo_path.to_string_lossy().to_string(),
        total_objects: git_objects.len(),
        total_blobs: git_objects.iter().filter(|obj| obj.object_type == "blob").count(),
        total_trees: git_objects.iter().filter(|obj| obj.object_type == "tree").count(),
        total_commits: git_objects.iter().filter(|obj| obj.object_type == "commit").count(),
        content_hashes,
        function_signatures,
        emoji_identifiers,
        git_objects,
        indexing_stats: IndexingStats {
            start_time: start_time.elapsed().as_secs_f64().to_string(),
            end_time: end_time.elapsed().as_secs_f64().to_string(),
            duration_seconds: duration.as_secs_f64(),
            objects_processed: git_objects.len(),
            functions_extracted: function_signatures.len(),
            duplicates_found: content_hashes.values().map(|e| e.duplicate_count).sum(),
        },
    };
    
    // Print summary
    println!("\n=== Git Object Index Summary ===");
    println!("Total objects: {}", index.total_objects);
    println!("Blobs: {}", index.total_blobs);
    println!("Trees: {}", index.total_trees);
    println!("Commits: {}", index.total_commits);
    println!("Content hashes: {}", index.content_hashes.len());
    println!("Functions extracted: {}", index.function_signatures.len());
    println!("Emoji identifiers: {}", index.emoji_identifiers.len());
    println!("Duplicates found: {}", index.indexing_stats.duplicates_found);
    
    if cli.verbose >= 1 {
        println!("\n=== Content Hash Analysis ===");
        let mut sorted_hashes: Vec<_> = index.content_hashes.iter().collect();
        sorted_hashes.sort_by(|a, b| b.1.duplicate_count.cmp(&a.1.duplicate_count));
        
        for (hash, entry) in sorted_hashes.iter().take(10) {
            println!("{}: {} duplicates, {} bytes, {:?}", 
                &hash[..16], entry.duplicate_count, entry.size, entry.language);
        }
    }
    
    // Write output
    let output_path = cli.output.unwrap_or_else(|| "git_object_index.json".to_string());
    let json_output = serde_json::to_string_pretty(&index)?;
    fs::write(&output_path, json_output)?;
    
    println!("\nGit object index written to: {}", output_path);
    println!("Indexing completed in {:.2} seconds", duration.as_secs_f64());
    
    Ok(())
} 