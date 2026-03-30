// FIXME
//use std::io::ErrorKind;
//use std::error::Error;
//token_count.rs
// Processes a single markdown file and extracts all information.
// pub fn process_file(file_path: &Path) -> Result<DocumentSummary,Error> {
//     let content = fs::read_to_string(file_path)?;

//     let mut turns = Vec::new();
//     let parts = content.split("---");

//     for part in parts {
//         let trimmed_part = part.trim();
//         if trimmed_part.starts_with("### User") {
//             let content = trimmed_part.strip_prefix("### User").unwrap_or("").trim();
//             let code_snippets = extract_code_snippets(content)
//                 .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
//             turns.push(ConversationTurn {
//                 author: "User".to_string(),
//                 content: content.to_string(),
//                 code_snippets,
//             });
//         } else if trimmed_part.starts_with("### Grok AI") {
//             let content = trimmed_part.strip_prefix("### Grok AI").unwrap_or("").trim();
//             let mut code_snippets = extract_code_snippets(content)
//                 .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;

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
