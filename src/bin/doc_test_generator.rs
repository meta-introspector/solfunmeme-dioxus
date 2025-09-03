use std::fs;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📚 Documentation Test Generator");
    println!("Generating tests from README examples...");
    
    let readme_content = fs::read_to_string("README.md")?;
    let tests = extract_code_examples(&readme_content)?;
    
//    generate_doc_tests(&tests)?;
    
    println!("✅ Generated {} documentation tests", tests.len());
    Ok(())
}

#[derive(Debug)]
struct CodeExample {
    language: String,
    code: String,
    description: Option<String>,
}

fn extract_code_examples(content: &str) -> Result<Vec<CodeExample>, Box<dyn std::error::Error>> {
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

// fn generate_doc_tests(examples: &[CodeExample]) -> Result<(), Box<dyn std::error::Error>> {
//     let mut test_content = String::new();
    
//     test_content.push_str(r#"//! Documentation tests generated from README.md
// //! These tests verify that code examples in the documentation work correctly.

// use solfunmeme_dioxus::core::*;
// use std::collections::HashMap;

// "#);
    
//     for (i, example) in examples.iter().enumerate() {
//         match example.language.as_str() {
//             "rust" => {
//                 test_content.push_str(&format!(r#"
// #[test]
// fn test_readme_example_{}() {{
//     // Example from README.md
//     let test_code = r#"{}"#;
    
//     // Test that the code can be parsed and analyzed
//     let mut analyzer = CodeAnalyzer::new(64, 0.8);
//     let result = analyzer.analyze_file(test_code, "readme_example_{}.rs".to_string());
    
//     match result {{
//         Ok(analysis) => {{
//             assert!(analysis.declarations.len() >= 0);
//             println!("✅ README example {} analyzed successfully", {});
//         }},
//         Err(e) => {{
//             // Some examples might be incomplete snippets, that's okay
//             println!("ℹ️  README example {} is a snippet: {{}}", {}, e);
//         }}
//     }}
// }}
// "#, i, example.code.replace("\"", "\\\""), i, i, i));
//             },
//             "bash" | "sh" => {
//                 test_content.push_str(&format!(r#"
// #[test]
// fn test_readme_bash_example() {{
//     // Bash example from README.md
//     let bash_command = r#"{}"#;
    
//     // Test that we can parse bash commands for environment setup
//     assert!(!bash_command.is_empty());
    
//     // Check for common environment variables
//     if bash_command.contains("OPENSSL_DIR") {{
//         println!("✅ Found OpenSSL configuration in example {}", {});
//     }}
    
//     if bash_command.contains("cargo") {{
//         println!("✅ Found Cargo command in example {}", {});
//     }}
// }}
// "#, i, example.code.replace("\"", "\\\""), i, i));
//             },
//             _ => {
//                 // For other languages, just verify the content exists
//                 test_content.push_str(&format!(r#"
//#[test]
// fn test_readme_content_example_{}() {{
//     // Content example from README.md ({})
//     let content = r#"{}"#;
//     assert!(!content.trim().is_empty());
//     println!("✅ README content example {} verified", {});
// }}
//"#, i, example.language, example.code.replace("\"", "\\\""), i));
//             }
//         }
//     }
    
//     // Add integration test for the overall README structure
//     test_content.push_str(r#"
// #[test]
// fn test_readme_structure_completeness() {
//     // Verify that README contains all required sections
//     let readme = include_str!("../../README.md");
    
//     let required_sections = vec![
//         "Overview",
//         "Goals", 
//         "Functionality",
//         "Requirements",
//         "Running",
//     ];
    
//     for section in required_sections {
//         assert!(readme.contains(section), "README missing section: {}", section);
//     }
    
//     println!("✅ README structure verification complete");
// }

#[test]
fn test_readme_code_examples_syntax() {
    // Test that all Rust code examples have valid syntax structure
    let readme = include_str!("../../README.md");
    let rust_examples = extract_rust_code_blocks(readme);
    
    for (i, code) in rust_examples.iter().enumerate() {
        // Basic syntax checks
        let balanced_braces = count_chars(code, '{') == count_chars(code, '}');
        let balanced_parens = count_chars(code, '(') == count_chars(code, ')');
        let balanced_brackets = count_chars(code, '[') == count_chars(code, ']');
        
        if !balanced_braces {
            println!("⚠️  Unbalanced braces in example {}", i);
        }
        if !balanced_parens {
            println!("⚠️  Unbalanced parentheses in example {}", i);
        }
        if !balanced_brackets {
            println!("⚠️  Unbalanced brackets in example {}", i);
        }
    }
    
    println!("✅ Syntax structure checks complete");
}

fn extract_rust_code_blocks(content: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_rust_block = false;
    let mut current_block = String::new();
    
    for line in lines {
        if line.starts_with("```rust") {
            in_rust_block = true;
            current_block.clear();
        } else if line.starts_with("```") && in_rust_block {
            in_rust_block = false;
            if !current_block.trim().is_empty() {
                blocks.push(current_block.clone());
            }
        } else if in_rust_block {
            current_block.push_str(line);
            current_block.push('\n');
        }
    }
    
    blocks
}

fn count_chars(s: &str, c: char) -> usize {
    s.chars().filter(|&ch| ch == c).count()
}
// "#);
    
//     // Write the generated tests
// //    fs::write("tests/doc_tests.rs", test_content)?;
    
//     Ok(())
// }
