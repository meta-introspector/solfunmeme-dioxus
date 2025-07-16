/// Estimate token count (rough approximation)
pub fn estimate_token_count(content: &str) -> usize {
    // Simple estimation: split by whitespace and common delimiters
    content
        .split_whitespace()
        .map(|word| {
            // Count punctuation and operators as separate tokens
            let punctuation_count = word.chars().filter(|c| c.is_ascii_punctuation()).count();
            1 + punctuation_count
        })
        .sum()
}
