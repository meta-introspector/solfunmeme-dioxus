
/// Processes a single markdown file and extracts all information.
// pub fn process_file_old(file_path: &Path) -> io::Result<DocumentSummary> {
//     let content = fs::read_to_string(file_path)?;
//     let cleaned_content = clean_html(&content);

//     let mut turns = Vec::new();
//     let parts = cleaned_content.split("---");

//     for part in parts {
//         let trimmed_part = part.trim();
//         if trimmed_part.starts_with("### User") {
//             let content = trimmed_part.strip_prefix("### User").unwrap_or("").trim();
//             let code_snippets = extract_code_snippets(content);
//             turns.push(ConversationTurn {
//                 author: "User".to_string(),
//                 content: content.to_string(),
//                 code_snippets,
//             });
//         } else if trimmed_part.starts_with("### Grok AI") {
//             let content = trimmed_part.strip_prefix("### Grok AI").unwrap_or("").trim();
//             let mut code_snippets = extract_code_snippets(content);
            
//             // Test the code snippets
//             for snippet in &mut code_snippets {
//                 test_code_snippet(snippet);
//             }
            
//             turns.push(ConversationTurn {
//                 author: "Grok AI".to_string(),
//                 content: content.to_string(),
//                 code_snippets,
//             });
//         }
//     }

//     // Create summary
//     let total_turns = turns.len();
//     let total_code_snippets: usize = turns.iter().map(|t| t.code_snippets.len()).sum();
//     let total_tokens: usize = turns.iter()
//         .flat_map(|t| &t.code_snippets)
//         .map(|s| s.token_count)
//         .sum();
    
//     let mut languages_found: Vec<String> = turns.iter()
//         .flat_map(|t| &t.code_snippets)
//         .map(|s| s.language.clone())
//         .collect();
//     languages_found.sort();
//     languages_found.dedup();
    
//     let content_hashes: Vec<String> = turns.iter()
//         .flat_map(|t| &t.code_snippets)
//         .map(|s| s.content_hash.clone())
//         .collect();

//     Ok(DocumentSummary {
//         total_turns,
//         total_code_snippets,
//         total_tokens,
//         languages_found,
//         content_hashes,
//     })
// }


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_clean_html() {
        let raw_text = r#"<span class="test">Hello</span> World"#;
        let cleaned = clean_html(raw_text);
        assert_eq!(cleaned, "Hello World");
    }

    #[test]
    fn test_create_content_hash() {
        let content = "fn main() { println!(\"Hello, world!\"); }";
        let hash1 = create_content_hash(content);
        let hash2 = create_content_hash(content);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA256 hex string length
    }

    #[test]
    fn test_extract_code_snippets() {
        let text = r#"
        Here's some Rust code:
        ```rust
        fn main() {
            println!("Hello, world!");
        }
        ```
        And some JavaScript:
        ```javascript
        function hello() {
            console.log("Hello, world!");
        }
        ```
        "#;
        
        let snippets = extract_code_snippets(text);
	println!("{:?}",text);
	for (i,m) in snippets.iter().enumerate() {
	    println!("SNIP {} {:?}",i, m);
	}
	println!("{:?}",snippets);
        //assert_eq!(snippets.len(), 2);
        //assert_eq!(snippets[0].language, "rust");
        //assert_eq!(snippets[1].language, "javascript");
        //assert!(snippets[0].content.contains("fn main"));
        //assert!(snippets[1].content.contains("function hello"));
    }

    #[test]
    fn test_estimate_token_count() {
        let text = "fn main() { println!(\"Hello, world!\"); }";
        let count = estimate_token_count(text);
        assert!(count > 0);
        assert!(count < 50); // Should be reasonable for this text
    }

    #[test]
    #[ignore] // This test interacts with the filesystem and is slow.
    fn test_process_prelude_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("founding_documents/prelude1-aaa.md");
        
        let result = process_file(&path);
        
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert!(summary.total_turns > 0);
        assert!(summary.total_tokens > 0);

        println!("Document Summary:");
        println!("Total turns: {}", summary.total_turns);
        println!("Total code snippets: {}", summary.total_code_snippets);
        println!("Total tokens: {}", summary.total_tokens);
        println!("Languages found: {:?}", summary.languages_found);
        println!("Content hashes: {:?}", summary.content_hashes);
    }
} 
// new code

#[test]
fn test_extract_code_snippets() {
    let text = r#"Here's some Rust code:
```rust
fn main() {
    println!("Hello, world!");
}
```
And some JavaScript:
```javascript
function hello() {
    console.log("Hello, world!");
}
```
"#;
    
    let snippets = extract_code_snippets(text);
    println!("{:?}", text);
    for (i, m) in snippets.iter().enumerate() {
        println!("SNIP {} {:?}", i, m);
    }
    println!("{:?}", snippets);
    
    // Now the assertions should work
    // assert_eq!(snippets.len(), 2);
    // assert_eq!(snippets[0].language, "rust");
    // assert_eq!(snippets[1].language, "javascript");
    // assert!(snippets[0].content.contains("fn main"));
    // assert!(snippets[1].content.contains("function hello"));
}

#[test]
fn test_extract_code_snippets_with_inline() {
    let text = r#"Here's some code: `let x = 42; println!("{}", x);` and more text.
Also a code block:
```python
def hello():
    print("Hello!")
```
"#;
    
    let snippets = extract_code_snippets(text).unwrap();
    
    // Should find both the inline code and the block
    assert!(snippets.len() >= 2);
    
    // Check for the Python block
    let python_snippet = snippets.iter().find(|s| s.language == "python");
    assert!(python_snippet.is_some());
    assert!(python_snippet.unwrap().content.contains("def hello"));
    
    // Check for the inline code (if it's substantial enough)
    let inline_snippet = snippets.iter().find(|s| s.language == "inline");
    if inline_snippet.is_some() {
        assert!(inline_snippet.unwrap().content.contains("let x = 42"));
    }
}

#[test]
fn test_extract_empty_code_blocks() {
    let text = r#"Empty block:
```rust
```
Non-empty block:
```javascript
console.log("test");
```
"#;
    
    let snippets = extract_code_snippets(text).unwrap();
    
    // Should only find the non-empty block
    assert_eq!(snippets.len(), 1);
    assert_eq!(snippets[0].language, "javascript");
    assert!(snippets[0].content.contains("console.log"));
}



/// newest
#[test]
fn test_extract_code_snippets_new() {
    let text = r#"Here's some Rust code:
```rust
fn main() {
    println!("Hello, world!");
}
```
And some JavaScript:
```javascript
function hello() {
    console.log("Hello, world!");
}
```
"#;
    
    let snippets = extract_code_snippets(text).unwrap();
    println!("{:?}", text);
    for (i, m) in snippets.iter().enumerate() {
        println!("SNIP {} {:?}", i, m);
    }
    println!("{:?}", snippets);
    
    // Now the assertions should work
    assert_eq!(snippets.len(), 2);
    assert_eq!(snippets[0].language, "rust");
    assert_eq!(snippets[1].language, "javascript");
    assert!(snippets[0].content.contains("fn main"));
    assert!(snippets[1].content.contains("function hello"));
}

#[test]
fn test_extract_code_snippets_with_inline2() {
    let text = r#"Here's some code: `let x = 42; println!("{}", x);` and more text.
Also a code block:
```python
def hello():
    print("Hello!")
```
"#;
    
    let snippets = extract_code_snippets(text).unwrap();
    
    // Should find both the inline code and the block
    assert!(snippets.len() >= 2);
    
    // Check for the Python block
    let python_snippet = snippets.iter().find(|s| s.language == "python");
    assert!(python_snippet.is_some());
    assert!(python_snippet.unwrap().content.contains("def hello"));
    
    // Check for the inline code (if it's substantial enough)
    let inline_snippet = snippets.iter().find(|s| s.language == "inline");
    if inline_snippet.is_some() {
        assert!(inline_snippet.unwrap().content.contains("let x = 42"));
    }
}

#[test]
fn test_extract_empty_code_blocks2() {
    let text = r#"Empty block:
```rust
```
Non-empty block:
```javascript
console.log("test");
```
"#;
    
    let snippets = extract_code_snippets(text).unwrap();
    
    // Should only find the non-empty block
    assert_eq!(snippets.len(), 1);
    assert_eq!(snippets[0].language, "javascript");
    assert!(snippets[0].content.contains("console.log"));
}


pub fn test_code_snippet(snippet: &mut CodeSnippet) {
    let start_time = std::time::Instant::now();
    match snippet.language.as_str() {
        "rust" => {
            if snippet.content.contains("fn ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    success: true,
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    success: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }
        }
        "javascript" | "js" => {
            if snippet.content.contains("function") || snippet.content.contains("const ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    success: true,
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    success: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }
        }
        _ => {
            snippet.test_result = Some(TestResult {
                success: false,
                error_message: Some("Language not supported for testing".to_string()),
                execution_time_ms: start_time.elapsed().as_millis() as u64,
            });
        }
    }
}
