use anyhow::Result;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: tantivy_analyzer <command> [options]");
        println!("Commands:");
        println!("  scan <directory>     - Scan directory for code patterns");
        println!("  word-freq <limit>    - Show most common words");
        println!("  file-types           - Show file type distribution");
        println!("  function-names       - Show function name patterns");
        println!("  struct-names         - Show struct name patterns");
        println!("  imports              - Show import patterns");
        return Ok(());
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "scan" => {
            if args.len() < 3 {
                println!("Usage: tantivy_analyzer scan <directory>");
                return Ok(());
            }
            let dir = &args[2];
            scan_directory(dir)?;
        }
        
        "word-freq" => {
            let limit = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(20);
            analyze_word_frequency("../vendor/tantivy", limit)?;
        }
        
        "file-types" => {
            analyze_file_types("../vendor/tantivy")?;
        }
        
        "function-names" => {
            analyze_function_names("../vendor/tantivy")?;
        }
        
        "struct-names" => {
            analyze_struct_names("../vendor/tantivy")?;
        }
        
        "imports" => {
            analyze_imports("../vendor/tantivy")?;
        }
        
        _ => {
            println!("Unknown command: {}", command);
            println!("Use 'tantivy_analyzer' without arguments to see available commands.");
        }
    }
    
    Ok(())
}

fn scan_directory(dir: &str) -> Result<()> {
    println!("Scanning directory: {}", dir);
    
    let mut file_count = 0;
    let mut line_count = 0;
    let mut char_count = 0;
    let mut file_types: HashMap<String, usize> = HashMap::new();
    
    scan_recursive(Path::new(dir), &mut file_count, &mut line_count, &mut char_count, &mut file_types)?;
    
    println!("\nDirectory Scan Results:");
    println!("=======================");
    println!("Total files: {}", file_count);
    println!("Total lines: {}", line_count);
    println!("Total characters: {}", char_count);
    println!("Average lines per file: {:.1}", line_count as f64 / file_count as f64);
    
    println!("\nFile Type Distribution:");
    let mut sorted_types: Vec<(String, usize)> = file_types.into_iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (ext, count) in sorted_types.iter().take(10) {
        println!("  {}: {} files", ext, count);
    }
    
    Ok(())
}

fn analyze_word_frequency(dir: &str, limit: usize) -> Result<()> {
    println!("Analyzing word frequency in: {}", dir);
    
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let word_regex = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b")?;
    
    scan_for_words(Path::new(dir), &word_regex, &mut word_counts)?;
    
    println!("\nTop {} most common words:", limit);
    println!("=========================");
    
    let mut sorted_words: Vec<(String, usize)> = word_counts.into_iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (i, (word, count)) in sorted_words.iter().take(limit).enumerate() {
        println!("{:3}. {:20} - {} occurrences", i + 1, word, count);
    }
    
    Ok(())
}

fn analyze_file_types(dir: &str) -> Result<()> {
    println!("Analyzing file types in: {}", dir);
    
    let mut file_types: HashMap<String, usize> = HashMap::new();
    let mut total_files = 0;
    
    scan_file_types(Path::new(dir), &mut file_types, &mut total_files)?;
    
    println!("\nFile Type Analysis:");
    println!("===================");
    println!("Total files: {}", total_files);
    
    let mut sorted_types: Vec<(String, usize)> = file_types.into_iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (ext, count) in sorted_types {
        let percentage = (count as f64 / total_files as f64) * 100.0;
        println!("  {:10}: {:4} files ({:.1}%)", ext, count, percentage);
    }
    
    Ok(())
}

fn analyze_function_names(dir: &str) -> Result<()> {
    println!("Analyzing function names in: {}", dir);
    
    let mut function_patterns: HashMap<String, usize> = HashMap::new();
    let function_regex = Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
    
    scan_for_patterns(Path::new(dir), &function_regex, &mut function_patterns)?;
    
    println!("\nFunction Name Patterns:");
    println!("=======================");
    
    let mut sorted_patterns: Vec<(String, usize)> = function_patterns.into_iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (i, (func_name, count)) in sorted_patterns.iter().take(20).enumerate() {
        println!("{:3}. {:30} - {} occurrences", i + 1, func_name, count);
    }
    
    Ok(())
}

fn analyze_struct_names(dir: &str) -> Result<()> {
    println!("Analyzing struct names in: {}", dir);
    
    let mut struct_patterns: HashMap<String, usize> = HashMap::new();
    let struct_regex = Regex::new(r"struct\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
    
    scan_for_patterns(Path::new(dir), &struct_regex, &mut struct_patterns)?;
    
    println!("\nStruct Name Patterns:");
    println!("=====================");
    
    let mut sorted_patterns: Vec<(String, usize)> = struct_patterns.into_iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (i, (struct_name, count)) in sorted_patterns.iter().take(20).enumerate() {
        println!("{:3}. {:30} - {} occurrences", i + 1, struct_name, count);
    }
    
    Ok(())
}

fn analyze_imports(dir: &str) -> Result<()> {
    println!("Analyzing import patterns in: {}", dir);
    
    let mut import_patterns: HashMap<String, usize> = HashMap::new();
    let use_regex = Regex::new(r"use\s+([^;]+);")?;
    
    scan_for_patterns(Path::new(dir), &use_regex, &mut import_patterns)?;
    
    println!("\nImport Patterns:");
    println!("================");
    
    let mut sorted_patterns: Vec<(String, usize)> = import_patterns.into_iter().collect();
    sorted_patterns.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (i, (import, count)) in sorted_patterns.iter().take(20).enumerate() {
        println!("{:3}. {:50} - {} occurrences", i + 1, import, count);
    }
    
    Ok(())
}

fn scan_recursive(
    dir: &Path,
    file_count: &mut usize,
    line_count: &mut usize,
    char_count: &mut usize,
    file_types: &mut HashMap<String, usize>,
) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Skip .git and target directories
            let path_str = path.to_string_lossy();
            if path_str.contains(".git") || path_str.contains("target") {
                continue;
            }
            scan_recursive(&path, file_count, line_count, char_count, file_types)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if matches!(ext_str.as_str(), "rs" | "md" | "txt" | "toml" | "json" | "yaml" | "yml") {
                    *file_count += 1;
                    *file_types.entry(ext_str.to_string()).or_insert(0) += 1;
                    
                    if let Ok(content) = fs::read_to_string(&path) {
                        *line_count += content.lines().count();
                        *char_count += content.len();
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn scan_for_words(
    dir: &Path,
    word_regex: &Regex,
    word_counts: &mut HashMap<String, usize>,
) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let path_str = path.to_string_lossy();
            if path_str.contains(".git") || path_str.contains("target") {
                continue;
            }
            scan_for_words(&path, word_regex, word_counts)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "rs" {
                    if let Ok(content) = fs::read_to_string(&path) {
                        for word_match in word_regex.find_iter(&content) {
                            let word = word_match.as_str().to_lowercase();
                            // Skip common Rust keywords and short words
                            if !is_rust_keyword(&word) && word.len() > 2 {
                                *word_counts.entry(word).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn scan_file_types(
    dir: &Path,
    file_types: &mut HashMap<String, usize>,
    total_files: &mut usize,
) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let path_str = path.to_string_lossy();
            if path_str.contains(".git") || path_str.contains("target") {
                continue;
            }
            scan_file_types(&path, file_types, total_files)?;
        } else if path.is_file() {
            *total_files += 1;
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                *file_types.entry(ext_str.to_string()).or_insert(0) += 1;
            } else {
                *file_types.entry("no_extension".to_string()).or_insert(0) += 1;
            }
        }
    }
    
    Ok(())
}

fn scan_for_patterns(
    dir: &Path,
    pattern_regex: &Regex,
    patterns: &mut HashMap<String, usize>,
) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let path_str = path.to_string_lossy();
            if path_str.contains(".git") || path_str.contains("target") {
                continue;
            }
            scan_for_patterns(&path, pattern_regex, patterns)?;
        } else if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "rs" {
                    if let Ok(content) = fs::read_to_string(&path) {
                        for cap in pattern_regex.captures_iter(&content) {
                            if let Some(matched) = cap.get(1) {
                                let pattern = matched.as_str().trim();
                                *patterns.entry(pattern.to_string()).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn is_rust_keyword(word: &str) -> bool {
    matches!(word, 
        "fn" | "let" | "mut" | "const" | "static" | "pub" | "use" | "mod" | "crate" | "extern" |
        "impl" | "trait" | "struct" | "enum" | "union" | "type" | "where" | "for" | "in" | "if" |
        "else" | "match" | "loop" | "while" | "for" | "return" | "break" | "continue" | "as" |
        "ref" | "self" | "Self" | "super" | "unsafe" | "async" | "await" | "dyn" | "move" | "box" |
        "true" | "false" | "None" | "Some" | "Ok" | "Err" | "Result" | "Option" | "String" | "str" |
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
        "f32" | "f64" | "bool" | "char" | "vec" | "Vec" | "HashMap" | "BTreeMap" | "HashSet" | "BTreeSet"
    )
} 