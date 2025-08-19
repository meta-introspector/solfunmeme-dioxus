/// Validate if content appears to be a markdown file
pub fn is_markdown_file(file_name: &str) -> bool {
    let lower_name = file_name.to_lowercase();
    lower_name.ends_with(".md")
        || lower_name.ends_with(".markdown")
        || lower_name.ends_with(".mdown")
}
