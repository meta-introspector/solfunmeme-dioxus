use syn::{ItemFn, ReturnType};
use std::path::Path;
use super::function_info::FunctionInfo;
use super::extract_semantic_summary::extract_semantic_summary;

pub fn analyze_rust_file(file_path: &Path) -> Vec<FunctionInfo> {
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
            functions_info.push(FunctionInfo {
                function_name,
                code_snippet,
                semantic_summary,
                file_path: file_path.to_string_lossy().replace("\\", "/").to_owned(),
                multivector_str: String::new(), // Placeholder
                sieve_address: String::new(),   // Placeholder
                closest_emoji: String::new(),   // Placeholder
                emoji_category: String::new(),  // Placeholder
                emoji_distance: 0.0,            // Placeholder
            });
        }
    }
    functions_info
}
