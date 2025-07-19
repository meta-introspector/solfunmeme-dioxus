use anyhow::Result;
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use solfunmeme_function_analysis::AnalyzedToken;
use indicatif::ProgressBar;
use std::collections::HashMap;

pub fn analyze_project_tokens(project_root: &Path, _ontology_path: &Path, _use_gpu: bool, pb: &ProgressBar) -> Result<HashMap<String, AnalyzedToken>> {
    let mut analyzed_tokens: HashMap<String, AnalyzedToken> = HashMap::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            if matches!(ext.as_str(), "rs" | "md" | "json" | "ttl" | "toml" | "txt" | "js" | "ts" | "tsx" | "py" | "go" | "java" | "c" | "cpp" | "h" | "hpp") {
                let content = fs::read_to_string(file_path)?;
                for word in content.split_whitespace() {
                    let entry = analyzed_tokens.entry(word.to_string()).or_insert_with(|| AnalyzedToken {
                        token: word.to_string(),
                        count: 0,
                        orbital_path: None,
                    });
                    entry.count += 1;
                }
            }
            pb.inc(entry.metadata()?.len());
        }
    }
    Ok(analyzed_tokens)
}
