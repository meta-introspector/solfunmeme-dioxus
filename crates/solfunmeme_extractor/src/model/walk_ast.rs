

use crate::model::content_hash::create_content_hash;
use markdown::mdast::Node;
use crate::model::estimate_token_count::estimate_token_count;
//use crate::extractor::model::token_count::estimate_token_count;
use shared_analysis_types::CodeSnippet;

/// Recursively walks the AST to find code snippets.  
pub fn walk_ast(node: &Node, snippets: &mut Vec<CodeSnippet>) {
    match node {
        Node::Code(code) => {
            if !code.value.trim().is_empty() {
                let content_hash = create_content_hash(&code.value);
                let token_count = estimate_token_count(&code.value);
                let line_count = code.value.lines().count();
                let char_count = code.value.chars().count();
                let language = code.lang.as_deref().unwrap_or("text").to_string();

                snippets.push(CodeSnippet {
                    content: code.value.clone(),
                    content_hash,
                    language,
                    token_count,
                    line_count,
                    char_count,
                    test_result: None,
                    line_start: 0,
                    line_end: 0,
                });
            }
        }
        Node::InlineCode(inline_code) => {
            if inline_code.value.len() > 10 {
                // Only consider substantial inline code
                let content_hash = crate::model::content_hash::create_content_hash(&inline_code.value);
                let token_count = estimate_token_count(&inline_code.value);
                let line_count = inline_code.value.lines().count();
                let char_count = inline_code.value.chars().count();

                snippets.push(CodeSnippet {
                    content: inline_code.value.clone(),
                    content_hash,
                    language: "inline".to_string(),
                    token_count,
                    line_count,
                    char_count,
                    test_result: None,
                    line_start: 0,
                    line_end: 0,
                });
            }
        }
        _ => {
            // Recursively process child nodes
            if let Some(children) = node.children() {
                for child in children {
                    walk_ast(child, snippets);
                }
            }
        }
    }
}

// fn walk_ast(node: &Node, snippets: &mut Vec<CodeSnippet>) {
//     match node {
//         Node::Code(code) => {
//             let trimmed_content = code.value.trim();
//             if !trimmed_content.is_empty() {
//                 let content_hash = create_content_hash(trimmed_content);
//                 let token_count = estimate_token_count(trimmed_content);
//                 let line_count = trimmed_content.lines().count();
//                 let char_count = trimmed_content.chars().count();
//                 let language = code.lang.as_deref().unwrap_or("text").to_string();

//                 snippets.push(CodeSnippet {
//                     content: trimmed_content.to_string(),
//                     content_hash,
//                     language,
//                     token_count,
//                     line_count,
//                     char_count,
//                     test_result: None,
//                 });
//             }
//         }
//         Node::InlineCode(inline_code) => {
//             let trimmed_content = inline_code.value.trim();
//             if trimmed_content.len() >= 5 {
//                 let content_hash = create_content_hash(trimmed_content);
//                 let token_count = estimate_token_count(trimmed_content);
//                 let line_count = trimmed_content.lines().count();
//                 let char_count = trimmed_content.chars().count();

//                 snippets.push(CodeSnippet {
//                     content: trimmed_content.to_string(),
//                     content_hash,
//                     language: "inline".to_string(),
//                     token_count,
//                     line_count,
//                     char_count,
//                     test_result: None,
//                 });
//             }
//         }
//         _ => {
//             if let Some(children) = node.children() {
//                 for child in children {
//                     walk_ast(child, snippets);
//                 }
//             }
//         }
//     }
// }
