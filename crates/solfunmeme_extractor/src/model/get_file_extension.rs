/// Get language-specific file extension for downloaded snippets
pub fn get_file_extension(language: &str) -> &'static str {
    match language.to_lowercase().as_str() {
        "rust" | "rs" => "rs",
        "javascript" | "js" => "js",
        "typescript" | "ts" => "ts",
        "python" | "py" => "py",
        "java" => "java",
        "cpp" | "c++" => "cpp",
        "c" => "c",
        "html" => "html",
        "css" => "css",
        "json" => "json",
        "xml" => "xml",
        "yaml" | "yml" => "yml",
        "toml" => "toml",
        "sql" => "sql",
        "bash" | "sh" => "sh",
        "powershell" | "ps1" => "ps1",
        _ => "txt",
    }
}
