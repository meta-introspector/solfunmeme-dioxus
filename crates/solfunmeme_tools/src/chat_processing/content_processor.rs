use regex::Regex;

fn clean_html(content: &str) -> String {
    // Simple HTML tag removal
    let html_tag_regex = Regex::new(r"<[^>]*>").unwrap_or_else(|_| Regex::new("").unwrap());
    let cleaned = html_tag_regex.replace_all(content, "");
    
    // Decode common HTML entities
    cleaned
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .to_string()
}

pub fn process_content(content: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cleaned_content = clean_html(content);
    
    // Split into conversation turns
    let turns: Vec<String> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(turns)
}
