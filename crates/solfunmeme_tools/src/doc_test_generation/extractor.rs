use regex::Regex;

#[derive(Debug)]
pub struct CodeExample {
    pub language: String,
    pub code: String,
    pub description: Option<String>,
}

pub fn extract_code_examples(content: &str) -> Result<Vec<CodeExample>, Box<dyn std::error::Error>> {
    let mut examples = Vec::new();
    
    // Regex to match code blocks
    let code_block_regex = Regex::new(r"```(\w+)?\n(.*?)\n```")?;
    
    for captures in code_block_regex.captures_iter(content) {
        let language = captures.get(1)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "text".to_string());
        
        let code = captures.get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        
        if !code.trim().is_empty() {
            examples.push(CodeExample {
                language,
                code,
                description: None,
            });
        }
    }
    
    Ok(examples)
}

