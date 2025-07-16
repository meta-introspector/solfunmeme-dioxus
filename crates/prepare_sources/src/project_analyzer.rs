use anyhow::Result;
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use solfunmeme_function_analysis::CodeChunk;



pub fn analyze_project(project_root: &Path) -> Result<Vec<CodeChunk>> {
    let mut code_chunks = Vec::new();

    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

            // Only process text/code files for now
            if matches!(ext.as_str(), "rs" | "md" | "json" | "ttl" | "toml" | "txt" | "js" | "ts" | "tsx" | "py" | "go" | "java" | "c" | "cpp" | "h" | "hpp") {
                match fs::read_to_string(file_path) {
                    Ok(content) => {
                        // Simple chunking for now
                        let lines: Vec<&str> = content.lines().collect();
                        let chunk_size = 50; // Lines per chunk

                        for (i, chunk_lines) in lines.chunks(chunk_size).enumerate() {
                            let start_line = i * chunk_size + 1;
                            let end_line = start_line + chunk_lines.len() - 1;

                            let chunk = CodeChunk {
                                language: ext.clone(),
                                content: chunk_lines.join("\n"),
                                line_start: start_line,
                                line_end: end_line,
                                content_hash: format!("{:x}", md5::compute(chunk_lines.join("\n"))),
                                token_count: chunk_lines.join(" ").split_whitespace().count(),
                                line_count: chunk_lines.len(),
                                char_count: chunk_lines.join("\n").chars().count(),
                                test_result: "Untested".to_string(),
                            };
                            code_chunks.push(chunk);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading file {}: {}", file_path.display(), e);
                    }
                }
            }
        }
    }
    Ok(code_chunks)
}