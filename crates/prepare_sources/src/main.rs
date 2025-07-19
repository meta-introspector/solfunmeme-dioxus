use anyhow::Result;
use std::io::{self, Write};
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use clap::Parser;

mod cli;
mod code_processing;

use cli::Cli;
use code_processing::process_code_chunks;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use rdf_processing_lib::ontology_generator::{generate_ontology, generate_token_ontology};
use rdf_processing_lib::ontology_generator::process_function::{AnalyzedFunction, AnalyzedToken};
use rdf_processing_lib::ontology_generator::closest_emoji_info::ClosestEmojiInfo;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let target_path = cli.path.map(|p| p.to_string_lossy().into_owned());
    let limit = cli.limit;

    let mut output_writer: Box<dyn Write> = if let Some(output_path) = cli.output {
        Box::new(File::create(&output_path)?)
    } else {
        Box::new(io::stdout())
    };

    let all_code_chunks = process_code_chunks(target_path, limit)?;

    // Prepare data for ontology generation
    let mut analyzed_functions: Vec<AnalyzedFunction> = Vec::new();
    let mut analyzed_tokens: HashMap<String, AnalyzedToken> = HashMap::new();

    for chunk in all_code_chunks {
        // Simplified mapping from CodeChunk to AnalyzedFunction
        let func_name = format!("{}_{}_{}", chunk.file_path.replace("/", "_").replace(".", "_"), chunk.line_start, chunk.line_end);
        let multivector_str = chunk.clifford_vector.map_or_else(|| "".to_string(), |mv| format!("{:?}", mv.0));

        analyzed_functions.push(AnalyzedFunction {
            function_name: func_name.clone(),
            file_path: chunk.file_path.clone(),
            code_snippet: chunk.content.clone(),
            semantic_summary: "".to_string(), // Placeholder
            multivector_str: multivector_str.clone(),
            sieve_address: "".to_string(), // Placeholder
            closest_emojis: Vec::new(), // Placeholder
            orbital_path: None, // Placeholder
        });

        // Simplified token analysis for AnalyzedToken
        // This is a very basic tokenization; a real implementation would use a proper tokenizer
        for token_str in chunk.content.split_whitespace() {
            let entry = analyzed_tokens.entry(token_str.to_string()).or_insert_with(|| AnalyzedToken {
                token: token_str.to_string(),
                count: 0,
                multivector_str: multivector_str.clone(),
                orbital_path: None,
            });
            entry.count += 1;
        }
    }

    // Generate ontologies
    let ontology_output_path = PathBuf::from("project_ontology.ttl");
    generate_ontology(analyzed_functions, &ontology_output_path)?;
    eprintln!("[INFO] Generated function ontology at {:?}", ontology_output_path);

    let token_ontology_output_path = PathBuf::from("token_ontology.ttl");
    generate_token_ontology(analyzed_tokens, &token_ontology_output_path)?;
    eprintln!("[INFO] Generated token ontology at {:?}", token_ontology_output_path);

    Ok(())
}
