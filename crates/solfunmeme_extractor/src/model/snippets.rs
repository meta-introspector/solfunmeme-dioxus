use markdown::{to_mdast, ParseOptions};
use crate::model::walk_ast::walk_ast;
use shared_analysis_types::CodeSnippet;

pub fn extract_markdown_snippets(text: &str) -> Result<Vec<CodeSnippet>, markdown::message::Message> {
    let mut snippets = Vec::new();
    let ast = to_mdast(text, &ParseOptions::default())?;
    walk_ast(&ast, &mut snippets);
    Ok(snippets)
}
