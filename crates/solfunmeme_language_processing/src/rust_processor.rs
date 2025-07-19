use super::LanguageProcessor;
use solfunmeme_function_analysis::{CodeChunk, AnalyzedFunction};
use std::path::Path;
use md5;
use syn::{self, ReturnType};
use quote;
use walkdir::WalkDir;
use anyhow::Result;

pub struct RustProcessor;

impl LanguageProcessor for RustProcessor {
    fn process_code(&self, content: &str, file_path: &str) -> Result<Vec<CodeChunk>> {
        println!("Processing Rust code from: {}", file_path);
        let mut chunks = Vec::new();

        // Analyze Rust file for functions
        let analyzed_functions = analyze_rust_file(Path::new(file_path));
        for func in analyzed_functions {
            chunks.push(CodeChunk {
                content: func.code_snippet.clone(),
                language: "rust".to_string(),
                line_start: 0, // TODO: Get actual line numbers
                line_end: 0,   // TODO: Get actual line numbers
                content_hash: format!("{:x}", md5::compute(&func.code_snippet)),
                token_count: func.code_snippet.split_whitespace().count(),
                line_count: func.code_snippet.lines().count(),
                char_count: func.code_snippet.chars().count(),
                test_result: None,
                semantic_summary: Some(func.semantic_summary),
                embedding: Vec::new(),
                clifford_vector: None,
            });
        }

        // Also extract general code snippets (e.g., markdown code blocks within Rust comments)
        // This part might need more sophisticated parsing if we want to extract code blocks
        // from comments in Rust files, but for now, we'll just treat the whole file as one chunk
        // if no functions are found, or if we want to capture top-level code.
        if chunks.is_empty() {
            chunks.push(CodeChunk {
                content: content.to_string(),
                language: "rust".to_string(),
                line_start: 1,
                line_end: content.lines().count(),
                content_hash: format!("{:x}", md5::compute(&content)),
                token_count: content.split_whitespace().count(),
                line_count: content.lines().count(),
                char_count: content.chars().count(),
                test_result: None,
                semantic_summary: None,
                embedding: Vec::new(),
                clifford_vector: None,
            });
        }

        Ok(chunks)
    }
}

pub fn analyze_rust_file(file_path: &Path) -> Vec<AnalyzedFunction> {
    let mut functions_info = Vec::new();
    let code = match std::fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", file_path.display(), e);
            return functions_info;
        }
    };

    let syntax = match syn::parse_file(&code) {
        Ok(syntax) => syntax,
        Err(e) => {
            eprintln!("Failed to parse file {}: {}", file_path.display(), e);
            return functions_info;
        }
    };

    for item in syntax.items {
        if let syn::Item::Fn(item_fn) = item {
            let function_name = item_fn.sig.ident.to_string();
            let code_snippet = quote::quote! { #item_fn }.to_string();
            let semantic_summary = extract_semantic_summary(&item_fn);
            functions_info.push(AnalyzedFunction {
                function_name,
                code_snippet,
                semantic_summary,
                file_path: file_path.to_string_lossy().replace("\\", "/").to_owned(),
                orbital_path: None,
            });
        }
    }
    functions_info
}

fn extract_semantic_summary(item_fn: &syn::ItemFn) -> String {
    let mut summary = String::new();
    summary.push_str(&item_fn.sig.ident.to_string());
    for input in &item_fn.sig.inputs {
        summary.push_str(&format!("{:?}", input));
    }
    if let ReturnType::Type(_, ty) = &item_fn.sig.output {
        summary.push_str(&format!("{:?}", ty));
    }
    summary
}

pub fn find_rust_files(project_root: &Path) -> Vec<String> {
    let mut rust_files = Vec::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "rs" {
                    rust_files.push(entry.path().to_string_lossy().into_owned());
                }
            }
        }
    }
    rust_files
}
