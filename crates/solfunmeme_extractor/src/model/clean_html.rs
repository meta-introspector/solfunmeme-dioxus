use regex::Regex;

/// Cleans HTML tags from a string.
pub fn clean_html(text: &str) -> String {
    let html_tag_re = Regex::new(r"<[^>]*>").unwrap();
    html_tag_re.replace_all(text, "").to_string()
}
