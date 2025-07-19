use markdown::mdast::Node;
use solfunmeme_function_analysis::CodeChunk;
use crate::model::walk_ast_modules::{code_node, inline_code_node};

/// Recursively walks the AST to find code snippets.  
pub fn walk_ast(node: &Node, snippets: &mut Vec<CodeChunk>) {
    match node {
        Node::Code(code) => {
            code_node::handle_code_node(code, snippets);
        }
        Node::InlineCode(inline_code) => {
            inline_code_node::handle_inline_code_node(inline_code, snippets);
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