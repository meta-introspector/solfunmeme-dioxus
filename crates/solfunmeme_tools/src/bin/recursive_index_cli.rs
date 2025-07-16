use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::io::BufRead;

#[derive(Parser)]
#[command(name = "recursive_index")]
#[command(about = "Recursively index codebase following Git object model structure", long_about = None)]
struct Cli {
    #[arg(help = "Root paths to analyze recursively")]
    paths: Vec<PathBuf>,

    #[arg(long, help = "Comma-separated list of file extensions to skip")]
    skip_ext: Option<String>,

    #[arg(long, help = "Maximum file size in bytes to index")]
    max_size: Option<u64>,

    #[arg(long, help = "Maximum depth for recursion (0 = unlimited)")]
    max_depth: Option<usize>,

    #[arg(long, help = "Output file for the recursive index (default: recursive_index.json)")]
    output: Option<String>,

    #[arg(long, help = "Parse .gitmodules files to understand submodule structure")]
    parse_gitmodules: bool,

    #[arg(long, default_value_t = false, help = "Show detailed tree structure")]
    tree: bool,

    #[arg(long, default_value_t = 0, help = "Verbosity level (0-3)")]
    verbose: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileNode {
    path: String,
    name: String,
    size: u64,
    extension: String,
    is_file: bool,
    is_directory: bool,
    depth: usize,
    children: Option<Vec<FileNode>>,
    metadata: FileMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileMetadata {
    lines: Option<usize>,
    language: Option<String>,
    is_binary: bool,
    is_text: bool,
    magic_header: Option<String>,
    git_submodule: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RecursiveIndex {
    root_paths: Vec<String>,
    total_files: usize,
    total_directories: usize,
    total_size: u64,
    file_types: HashMap<String, FileTypeStats>,
    directory_tree: Vec<FileNode>,
    git_submodules: Vec<GitSubmodule>,
    skip_rules: SkipRules,
    indexing_stats: IndexingStats,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileTypeStats {
    count: usize,
    total_size: u64,
    avg_size: f64,
    max_size: u64,
    min_size: u64,
    extensions: HashSet<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitSubmodule {
    path: String,
    url: String,
    branch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SkipRules {
    extensions: Vec<String>,
    max_size: Option<u64>,
    max_depth: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexingStats {
    start_time: String,
    end_time: String,
    duration_seconds: f64,
    files_processed: usize,
    files_skipped: usize,
    directories_processed: usize,
}

fn is_text_file(path: &Path, content: &[u8]) -> bool {
    // Check file extension first
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        let text_extensions = [
            "rs", "md", "json", "toml", "txt", "js", "ts", "tsx", "py", "go", "java", 
            "c", "cpp", "h", "hpp", "scm", "lisp", "clj", "hs", "ml", "f90", "f95",
            "sql", "xml", "html", "css", "scss", "yaml", "yml", "ini", "cfg", "conf",
            "sh", "bash", "zsh", "fish", "ps1", "bat", "cmd", "makefile", "dockerfile",
            "gitignore", "gitattributes", "gitmodules", "license", "readme", "changelog"
        ];
        if text_extensions.contains(&ext_str.as_str()) {
            return true;
        }
    }

    // Check magic headers for common text formats
    if content.len() >= 4 {
        let header = &content[..4];
        if header == b"#!sh" || header == b"#!ba" || header == b"#!py" || 
           header == b"#!go" || header == b"#!js" || header == b"#!ts" {
            return true;
        }
    }

    // Check if content is mostly printable ASCII/UTF-8
    if content.len() > 0 {
        let printable_ratio = content.iter()
            .filter(|&&b| b.is_ascii_graphic() || b.is_ascii_whitespace())
            .count() as f64 / content.len() as f64;
        return printable_ratio > 0.8;
    }

    false
}

fn get_magic_header(content: &[u8], n: usize) -> String {
    let n_read = std::cmp::min(n, content.len());
    content[..n_read].iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn detect_language(path: &Path, content: &[u8]) -> Option<String> {
    // Check file extension
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        match ext_str.as_str() {
            "rs" => return Some("rust".to_string()),
            "md" => return Some("markdown".to_string()),
            "json" => return Some("json".to_string()),
            "toml" => return Some("toml".to_string()),
            "js" => return Some("javascript".to_string()),
            "ts" => return Some("typescript".to_string()),
            "tsx" => return Some("typescript-react".to_string()),
            "py" => return Some("python".to_string()),
            "go" => return Some("go".to_string()),
            "java" => return Some("java".to_string()),
            "c" => return Some("c".to_string()),
            "cpp" | "cc" | "cxx" => return Some("cpp".to_string()),
            "h" | "hpp" => return Some("cpp-header".to_string()),
            "scm" => return Some("scheme".to_string()),
            "lisp" | "clj" => return Some("lisp".to_string()),
            "hs" => return Some("haskell".to_string()),
            "ml" => return Some("ocaml".to_string()),
            "sql" => return Some("sql".to_string()),
            "xml" => return Some("xml".to_string()),
            "html" | "htm" => return Some("html".to_string()),
            "css" => return Some("css".to_string()),
            "sh" | "bash" => return Some("bash".to_string()),
            "ps1" => return Some("powershell".to_string()),
            "makefile" | "mk" => return Some("makefile".to_string()),
            "dockerfile" => return Some("dockerfile".to_string()),
            _ => {}
        }
    }

    // Check shebang
    if content.len() > 2 && content[0] == b'#' && content[1] == b'!' {
        let shebang = String::from_utf8_lossy(&content[2..]);
        if shebang.contains("python") {
            return Some("python".to_string());
        } else if shebang.contains("bash") || shebang.contains("sh") {
            return Some("bash".to_string());
        } else if shebang.contains("node") {
            return Some("javascript".to_string());
        }
    }

    None
}

fn parse_gitmodules_file(path: &Path) -> Result<Vec<GitSubmodule>> {
    let content = fs::read_to_string(path)?;
    let mut submodules = Vec::new();
    let mut current_submodule: Option<GitSubmodule> = None;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("[submodule") && line.ends_with("]") {
            // Save previous submodule if exists
            if let Some(submodule) = current_submodule.take() {
                submodules.push(submodule);
            }
            // Start new submodule
            let name = line[10..line.len()-1].trim_matches('"');
            current_submodule = Some(GitSubmodule {
                path: name.to_string(),
                url: String::new(),
                branch: None,
            });
        } else if let Some(submodule) = &mut current_submodule {
            if line.starts_with("path = ") {
                submodule.path = line[7..].trim().to_string();
            } else if line.starts_with("url = ") {
                submodule.url = line[6..].trim().to_string();
            } else if line.starts_with("branch = ") {
                submodule.branch = Some(line[9..].trim().to_string());
            }
        }
    }

    // Add the last submodule
    if let Some(submodule) = current_submodule {
        submodules.push(submodule);
    }

    Ok(submodules)
}

fn build_recursive_tree(
    path: &Path,
    depth: usize,
    max_depth: Option<usize>,
    skip_exts: &HashSet<String>,
    max_size: Option<u64>,
    verbose: u8,
) -> Result<Option<FileNode>> {
    if let Some(max_d) = max_depth {
        if depth > max_d {
            return Ok(None);
        }
    }

    let metadata = fs::metadata(path)?;
    let is_file = metadata.is_file();
    let is_directory = metadata.is_dir();
    let size = metadata.len();
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Skip files based on rules
    if is_file {
        if !skip_exts.is_empty() && skip_exts.contains(&extension) {
            if verbose >= 2 {
                println!("Skipping file (extension): {}", path.display());
            }
            return Ok(None);
        }
        if let Some(max) = max_size {
            if size > max {
                if verbose >= 2 {
                    println!("Skipping file (size): {} ({} bytes)", path.display(), size);
                }
                return Ok(None);
            }
        }
    }

    let mut file_metadata = FileMetadata {
        lines: None,
        language: None,
        is_binary: false,
        is_text: false,
        magic_header: None,
        git_submodule: None,
    };

    if is_file {
        match fs::read(path) {
            Ok(content) => {
                file_metadata.is_text = is_text_file(path, &content);
                file_metadata.is_binary = !file_metadata.is_text;
                file_metadata.magic_header = Some(get_magic_header(&content, 16));
                
                if file_metadata.is_text {
                    file_metadata.lines = Some(String::from_utf8_lossy(&content).lines().count());
                    file_metadata.language = detect_language(path, &content);
                }
            }
            Err(_) => {
                // File couldn't be read, mark as binary
                file_metadata.is_binary = true;
            }
        }
    }

    let mut children = None;
    if is_directory {
        let mut child_nodes = Vec::new();
        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries.filter_map(|e| e.ok()) {
                    let child_path = entry.path();
                    if let Some(child_node) = build_recursive_tree(
                        &child_path, depth + 1, max_depth, skip_exts, max_size, verbose
                    )? {
                        child_nodes.push(child_node);
                    }
                }
            }
            Err(_) => {
                // Directory couldn't be read, skip
                return Ok(None);
            }
        }
        if !child_nodes.is_empty() {
            children = Some(child_nodes);
        }
    }

    Ok(Some(FileNode {
        path: path.to_string_lossy().to_string(),
        name,
        size,
        extension,
        is_file,
        is_directory,
        depth,
        children,
        metadata: file_metadata,
    }))
}

fn collect_stats(nodes: &[FileNode]) -> (usize, usize, u64, HashMap<String, FileTypeStats>) {
    let mut total_files = 0;
    let mut total_directories = 0;
    let mut total_size = 0;
    let mut file_types: HashMap<String, FileTypeStats> = HashMap::new();

    fn traverse(nodes: &[FileNode], stats: &mut (usize, usize, u64, &mut HashMap<String, FileTypeStats>)) {
        for node in nodes {
            if node.is_file {
                stats.0 += 1;
                stats.2 += node.size;
                
                let ext = if node.extension.is_empty() { "no-extension".to_string() } else { node.extension.clone() };
                let type_stats = stats.3.entry(ext.clone()).or_insert_with(|| FileTypeStats {
                    count: 0,
                    total_size: 0,
                    avg_size: 0.0,
                    max_size: 0,
                    min_size: u64::MAX,
                    extensions: HashSet::new(),
                });
                
                type_stats.count += 1;
                type_stats.total_size += node.size;
                type_stats.max_size = type_stats.max_size.max(node.size);
                type_stats.min_size = type_stats.min_size.min(node.size);
                type_stats.extensions.insert(ext);
            } else if node.is_directory {
                stats.1 += 1;
            }

            if let Some(ref children) = node.children {
                traverse(children, stats);
            }
        }
    }

    traverse(nodes, &mut (total_files, total_directories, total_size, &mut file_types));

    // Calculate averages
    for stats in file_types.values_mut() {
        if stats.count > 0 {
            stats.avg_size = stats.total_size as f64 / stats.count as f64;
        }
    }

    (total_files, total_directories, total_size, file_types)
}

fn print_tree(node: &FileNode, prefix: &str, is_last: bool, max_depth: Option<usize>) {
    let connector = if is_last { "└── " } else { "├── " };
    let size_str = if node.is_file {
        if node.size > 1024 * 1024 {
            format!("({:.1}MB)", node.size as f64 / 1024.0 / 1024.0)
        } else if node.size > 1024 {
            format!("({:.1}KB)", node.size as f64 / 1024.0)
        } else {
            format!("({}B)", node.size)
        }
    } else {
        "".to_string()
    };

    let ext_str = if !node.extension.is_empty() { format!(".{}", node.extension) } else { "".to_string() };
    let language_str = node.metadata.language.as_ref().map(|l| format!(" [{}]", l)).unwrap_or_default();
    
    println!("{}{}{}{}{}{}", prefix, connector, node.name, ext_str, size_str, language_str);

    if let Some(ref children) = node.children {
        let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };
        let mut iter = children.iter().peekable();
        
        while let Some(child) = iter.next() {
            let is_last_child = iter.peek().is_none();
            if let Some(max_d) = max_depth {
                if child.depth >= max_d {
                    println!("{}└── ... (max depth reached)", new_prefix);
                    break;
                }
            }
            print_tree(child, &new_prefix, is_last_child, max_depth);
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let start_time = Instant::now();

    if cli.paths.is_empty() {
        println!("Usage: recursive_index <path1> [path2 ...] [options]");
        println!("Example: recursive_index vendor/ --parse-gitmodules --tree --max-depth 3");
        return Ok(());
    }

    let skip_exts: HashSet<String> = cli
        .skip_ext
        .as_deref()
        .unwrap_or("")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_lowercase())
        .collect();

    if cli.verbose >= 1 {
        println!("=== Recursive Indexing ===");
        println!("Root paths: {:?}", cli.paths);
        println!("Skip extensions: {:?}", skip_exts);
        println!("Max size: {:?}", cli.max_size);
        println!("Max depth: {:?}", cli.max_depth);
        println!("Parse gitmodules: {}", cli.parse_gitmodules);
    }

    // Parse gitmodules if requested
    let mut git_submodules = Vec::new();
    if cli.parse_gitmodules {
        for root_path in &cli.paths {
            let gitmodules_path = root_path.join(".gitmodules");
            if gitmodules_path.exists() {
                if cli.verbose >= 1 {
                    println!("Parsing gitmodules: {}", gitmodules_path.display());
                }
                match parse_gitmodules_file(&gitmodules_path) {
                    Ok(submodules) => {
                        let submodule_count = submodules.len();
                        git_submodules.extend(submodules);
                        if cli.verbose >= 1 {
                            println!("Found {} submodules", submodule_count);
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not parse gitmodules: {}", e);
                    }
                }
            }
        }
    }

    // Build recursive tree
    let mut all_trees = Vec::new();
    let files_processed = 0;
    let files_skipped = 0;
    let directories_processed = 0;

    for root_path in &cli.paths {
        if cli.verbose >= 1 {
            println!("Processing root: {}", root_path.display());
        }

        if let Some(tree) = build_recursive_tree(
            root_path, 0, cli.max_depth, &skip_exts, cli.max_size, cli.verbose
        )? {
            all_trees.push(tree);
        }
    }

    // Collect statistics
    let (total_files, total_directories, total_size, file_types) = collect_stats(&all_trees);

    // Print tree structure if requested
    if cli.tree {
        println!("\n=== Directory Tree ===");
        for (i, tree) in all_trees.iter().enumerate() {
            let is_last = i == all_trees.len() - 1;
            print_tree(tree, "", is_last, cli.max_depth);
        }
    }

    // Print summary
    println!("\n=== Recursive Index Summary ===");
    println!("Total files: {}", total_files);
    println!("Total directories: {}", total_directories);
    println!("Total size: {:.2} MB", total_size as f64 / 1024.0 / 1024.0);
    println!("Git submodules: {}", git_submodules.len());

    if cli.verbose >= 1 {
        println!("\n=== File Types ===");
        let mut sorted_types: Vec<_> = file_types.iter().collect();
        sorted_types.sort_by(|a, b| b.1.total_size.cmp(&a.1.total_size));
        
        for (ext, stats) in sorted_types.iter().take(10) {
            println!("{}: {} files, {:.2} MB total, {:.1} KB avg", 
                ext, stats.count, stats.total_size as f64 / 1024.0 / 1024.0, 
                stats.avg_size / 1024.0);
        }
    }

    // Create index structure
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    
    let index = RecursiveIndex {
        root_paths: cli.paths.iter().map(|p| p.to_string_lossy().to_string()).collect(),
        total_files,
        total_directories,
        total_size,
        file_types,
        directory_tree: all_trees,
        git_submodules,
        skip_rules: SkipRules {
            extensions: skip_exts.iter().cloned().collect(),
            max_size: cli.max_size,
            max_depth: cli.max_depth,
        },
        indexing_stats: IndexingStats {
            start_time: start_time.elapsed().as_secs_f64().to_string(),
            end_time: end_time.elapsed().as_secs_f64().to_string(),
            duration_seconds: duration.as_secs_f64(),
            files_processed,
            files_skipped,
            directories_processed,
        },
    };

    // Write output
    let output_path = cli.output.unwrap_or_else(|| "recursive_index.json".to_string());
    let json_output = serde_json::to_string_pretty(&index)?;
    fs::write(&output_path, json_output)?;
    
    println!("\nRecursive index written to: {}", output_path);
    println!("Indexing completed in {:.2} seconds", duration.as_secs_f64());

    Ok(())
} 