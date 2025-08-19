use std::env;
use std::path::Path;
use std::fs;

use prepare_sources::project_analyzer::{analyze_project, analyze_markdown_files, analyze_project_tokens, create_theme_nodes_from_emoji_matrix, AnalyzedFunction, AnalyzedDocument, AnalyzedToken, ClosestEmojiInfo};
use rdf_processing_lib::ontology_generator::{generate_ontology, generate_token_ontology};
use indicatif::{ProgressBar, ProgressStyle};
use orbital_sim_lib::{simulate_orbit, ThemeNode};
use emoji_matrix_lib::{parse_summary_total, parse_summary_root, rollup_emoji_matrix};
use core_data_lib::EmojiMatrix;
use std::collections::HashMap;
use tracing::{info, instrument};
use tracing_subscriber::{fmt, EnvFilter};

use solana_integration_lib::solana_bootstrap::bootstrap_to_solana;
use solana_integration_lib::{
    Ontology, CodeFile, Function, ClosestEmojiInfo as SolanaClosestEmojiInfo, Term as SolanaTerm, BuyOrder, BuyOrderStatus
};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use std::str::FromStr;
use sha2::{Sha256, Digest};

#[instrument]
fn main() {
    fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: cargo run -- <command> <path> [--use-gpu] [solana_program_id] [solana_payer_keypair]");
        eprintln!("Commands: analyze-rust, analyze-markdown, generate-glossary, simulate-orbits, bootstrap-solana");
        return;
    }

    let command = &args[1];
    let project_root = Path::new(&args[2]);
    let use_gpu = args.iter().any(|arg| arg == "--use-gpu");

    let ontology_path = Path::new("../ontologies/zos/v1.ttl"); // Path to your ontology
    let output_ontology_path = Path::new("project_ontology.ttl");
    let token_ontology_path = Path::new("token_ontology.ttl");

    match command.as_str() {
        "analyze-rust" => {
            info!("Analyzing Rust project: {}", project_root.display());
            let total_size = WalkDir::new(project_root)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry| entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs"))
                .filter_map(|entry| entry.metadata().ok())
                .map(|metadata| metadata.len())
                .sum();

            let pb = ProgressBar::new(total_size);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>- "));
            pb.set_message("Analyzing Rust files...");

            match analyze_project(project_root, ontology_path, use_gpu, &pb) {
                Ok(analyzed_functions) => {
                    pb.finish_with_message("Rust project analysis complete.");
                    info!("Rust project analysis complete.");
                    println!("\n--- Analysis Results ---");
                    if analyzed_functions.is_empty() {
                        println!("No functions found or analyzed in the Rust project.");
                    } else {
                        print_top_emojis_and_prompts(&analyzed_functions);
                        match generate_ontology(analyzed_functions, output_ontology_path) {
                            Ok(_) => println!("\nSuccessfully generated ontology to {}.", output_ontology_path.display()),
                            Err(e) => eprintln!("Error generating ontology: {}", e),
                        }
                    }
                }
                Err(e) => {
                    pb.finish_with_message("Rust project analysis failed.");
                    eprintln!("Error analyzing Rust project: {}", e);
                }
            }
        }
        "analyze-markdown" => {
            info!("Analyzing Markdown documents: {}", project_root.display());
            let total_size = WalkDir::new(project_root)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry| entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "md" || ext == "markdown"))
                .filter_map(|entry| entry.metadata().ok())
                .map(|metadata| metadata.len())
                .sum();

            let pb = ProgressBar::new(total_size);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>- "));
            pb.set_message("Analyzing Markdown files...");

            match analyze_markdown_files(project_root, ontology_path, use_gpu, &pb) {
                Ok(analyzed_documents) => {
                    pb.finish_with_message("Markdown document analysis complete.");
                    info!("Markdown document analysis complete.");
                    println!("\n--- Analysis Results ---");
                    if analyzed_documents.is_empty() {
                        println!("No Markdown documents found or analyzed.");
                    }
                }
                Err(e) => {
                    pb.finish_with_message("Markdown document analysis failed.");
                    eprintln!("Error analyzing Markdown documents: {}", e);
                }
            }
        }
        "generate-glossary" => {
            info!("Generating glossary for project: {}", project_root.display());
            let total_size = WalkDir::new(project_root)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry| {
                    entry.file_type().is_file() && (
                        entry.path().extension().map_or(false, |ext| ext == "rs") ||
                        entry.path().extension().map_or(false, |ext| ext == "md" || ext == "markdown")
                    )
                })
                .filter_map(|entry| entry.metadata().ok())
                .map(|metadata| metadata.len())
                .sum();

            let pb = ProgressBar::new(total_size);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>- "));
            pb.set_message("Analyzing tokens...");

            match analyze_project_tokens(project_root, ontology_path, use_gpu, &pb) {
                Ok(analyzed_tokens) => {
                    pb.finish_with_message("Token analysis complete.");
                    info!("Token analysis complete.");
                    println!("\n--- Token Glossary ---");
                    for (token, data) in analyzed_tokens.iter() {
                        println!("Token: \"{}\", Count: {}, Multivector: {}", token, data.count, data.multivector_str);
                    }
                    match generate_token_ontology(analyzed_tokens, token_ontology_path) {
                        Ok(_) => println!("\nSuccessfully generated token ontology to {}.", token_ontology_path.display()),
                        Err(e) => eprintln!("Error generating token ontology: {}", e),
                    }
                }
                Err(e) => {
                    pb.finish_with_message("Token analysis failed.");
                    eprintln!("Error analyzing tokens: {}", e);
                }
            }
        }
        "simulate-orbits" => {
            info!("Simulating orbits for project: {}", project_root.display());
            let pb = ProgressBar::new_spinner();
            pb.set_style(ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap());
            pb.set_message("Calculating orbits...");
            pb.enable_steady_tick(std::time::Duration::from_millis(100));

            let total_matrix = parse_summary_total();
            let root_matrix = parse_summary_root();
            let mut combined_entries = total_matrix.entries;
            combined_entries.extend(root_matrix.entries);
            let final_matrix = rollup_emoji_matrix(EmojiMatrix { entries: combined_entries });
            let nodes = create_theme_nodes_from_emoji_matrix(final_matrix);

            let k = 1.0; // Force constant
            let t_span = (0.0, 10.0);
            let n_steps = 1000;

            let orbits: Vec<Vec<(f64, f64)>> = nodes
                .iter()
                .map(|node| {
                    let initial_state = nalgebra::vector![
                        node.initial_position[0],
                        node.initial_position[1],
                        node.initial_position[2],
                        node.initial_position[3],
                        node.initial_velocity[0],
                        node.initial_velocity[1],
                        node.initial_velocity[2],
                        node.initial_velocity[3],
                    ];
                    simulate_orbit(t_span, n_steps, initial_state, k, node.mass)
                })
                .collect();

            pb.finish_with_message("Orbit simulation complete.");
            info!("Orbit simulation complete.");
            println!("\n--- Orbital Paths ---");
            for (i, orbit) in orbits.iter().enumerate() {
                println!("Orbit for node {}: {} points", i, orbit.len());
            }
        }
        "bootstrap-solana" => {
            info!("Bootstrapping to Solana...");
            if args.len() < 5 {
                eprintln!("Usage: cargo run -- bootstrap-solana <path_to_rdf_data> <solana_program_id> <solana_payer_keypair>");
                return;
            }
            let rdf_data_path = Path::new(&args[2]);
            let solana_program_id = Pubkey::from_str(&args[3]).expect("Invalid Solana Program ID");
            let solana_payer_keypair_str = &args[4];
            let solana_payer_keypair = Keypair::from_base58_string(solana_payer_keypair_str).expect("Invalid Solana Payer Keypair");
            let rpc_url = "http://localhost:8899"; // Your validator RPC

            // 1. Analyze project data
            let pb = ProgressBar::new_spinner();
            pb.set_style(ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap());
            pb.set_message("Analyzing project data for Solana...");
            pb.enable_steady_tick(std::time::Duration::from_millis(100));

            let analyzed_functions = analyze_project(project_root, ontology_path, use_gpu, &pb).expect("Failed to analyze Rust project");
            let analyzed_documents = analyze_markdown_files(project_root, ontology_path, use_gpu, &pb).expect("Failed to analyze Markdown documents");
            let analyzed_tokens = analyze_project_tokens(project_root, ontology_path, use_gpu, &pb).expect("Failed to analyze project tokens");

            pb.finish_with_message("Project data analysis complete for Solana.");

            // 2. Convert analyzed data to Solana data models
            let ontology_instance = Ontology {
                id: "prepare_sources_ontology".to_string(),
                classes: vec!["CodeFile".to_string(), "Function".to_string(), "ClosestEmojiInfo".to_string(), "Term".to_string(), "BuyOrder".to_string()],
                properties: vec!["isInFile".to_string(), "hasCodeSnippet".to_string(), "hasSemanticSummary".to_string(), "hasMultivectorEmbedding".to_string(), "hasSieveAddress".to_string(), "hasClosestEmojiInfo".to_string(), "emojiChar".to_string(), "emojiCategory".to_string(), "emojiDistance".to_string(), "hasOrbitalPath".to_string(), "token".to_string(), "count".to_string(), "multivectorEmbedding".to_string(), "hasSourceChunk".to_string(), "hasVectorizedResult".to_string(), "hasBuyOrderStatus".to_string()],
                creator: solana_payer_keypair.pubkey(),
                timestamp: 0,
            };

            let (solana_functions, solana_code_files, solana_emojis, solana_buy_orders) = 
                convert_analyzed_functions_to_solana_functions(
                    analyzed_functions,
                    &ontology_instance,
                    &solana_payer_keypair,
                    solana_program_id,
                );
            let solana_terms = 
                convert_analyzed_tokens_to_solana_terms(
                    analyzed_tokens,
                    &ontology_instance,
                    &solana_payer_keypair,
                    solana_program_id,
                );

            // 3. Bootstrap to Solana
            match bootstrap_to_solana(
                ontology_instance,
                solana_code_files.into_values().collect(),
                solana_functions,
                solana_emojis.into_values().collect(),
                solana_terms,
                solana_buy_orders,
                solana_program_id,
                &solana_payer_keypair,
                rpc_url,
            ) {
                Ok(_) => info!("Successfully bootstrapped to Solana."),
                Err(e) => eprintln!("Error bootstrapping to Solana: {}", e),
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Usage: cargo run -- <command> <path> [--use-gpu] [solana_program_id] [solana_payer_keypair]");
            eprintln!("Commands: analyze-rust, analyze-markdown, generate-glossary, simulate-orbits, bootstrap-solana");
        }
    }
}

fn print_top_emojis_and_prompts(analyzed_functions: &Vec<AnalyzedFunction>) {
    let mut all_emojis: Vec<ClosestEmojiInfo> = Vec::new();
    for func in analyzed_functions {
        all_emojis.extend(func.closest_emojis.clone());
    }

    all_emojis.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));
    all_emojis.dedup_by_key(|e| e.emoji.clone());

    println!("\n--- Top 10 Closest Emojis and LLM Prompts ---");
    for (i, emoji_info) in all_emojis.iter().take(10).enumerate() {
        println!("\n{}. Emoji: {} ({})", i + 1, emoji_info.emoji, emoji_info.category);
        println!("   Distance: {:.4}", emoji_info.distance);
        println!("   LLM Prompt: \"Given the semantic summary '{}', is '{}' ({}) a fitting emoji representation? Explain why or why not.\"", 
            analyzed_functions.iter().find(|f| f.closest_emojis.iter().any(|e| e.emoji == emoji_info.emoji)).map_or("N/A", |f| f.semantic_summary.as_str()),
            emoji_info.emoji,
            emoji_info.category
        );
    }
}

fn print_top_emojis_and_prompts_for_docs(analyzed_documents: &Vec<AnalyzedDocument>) {
    let mut all_emojis: Vec<ClosestEmojiInfo> = Vec::new();
    for doc in analyzed_documents {
        for snippet in &doc.analyzed_snippets {
            all_emojis.extend(snippet.closest_emojis.clone());
        }
    }

    all_emojis.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));
    all_emojis.dedup_by_key(|e| e.emoji.clone());

    println!("\n--- Top 10 Closest Emojis for Documents and LLM Prompts ---");
    for (i, emoji_info) in all_emojis.iter().take(10).enumerate() {
        println!("\n{}. Emoji: {} ({})", i + 1, emoji_info.emoji, emoji_info.category);
        println!("   Distance: {:.4}", emoji_info.distance);
        println!("   LLM Prompt: \"Given the semantic summary '{}' from a document snippet, is '{}' ({}) a fitting emoji representation? Explain why or why not.\"", 
            analyzed_documents.iter().flat_map(|d| &d.analyzed_snippets).find(|s| s.closest_emojis.iter().any(|e| e.emoji == emoji_info.emoji)).map_or("N/A", |s| s.semantic_summary.as_str()),
            emoji_info.emoji,
            emoji_info.category
        );
    }
}

fn print_top_emojis_and_prompts_for_tokens(analyzed_tokens: &HashMap<String, AnalyzedToken>) {
    let mut all_emojis: Vec<ClosestEmojiInfo> = Vec::new();
    // For tokens, we don't have closest_emojis directly, but we can generate a dummy one for reporting
    for (token_str, token_data) in analyzed_tokens {
        // This is a simplified representation for reporting purposes
        // In a real scenario, you might want to calculate closest emojis for tokens too
        all_emojis.push(ClosestEmojiInfo {
            emoji: "❓".to_string(), // Placeholder emoji for tokens
            category: "Token".to_string(),
            distance: 0.0, // Placeholder distance
        });
    }

    all_emojis.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));
    all_emojis.dedup_by_key(|e| e.emoji.clone());

    println!("\n--- Top 10 Closest Emojis for Tokens and LLM Prompts ---");
    for (i, emoji_info) in all_emojis.iter().take(10).enumerate() {
        println!("\n{}. Emoji: {} ({})", i + 1, emoji_info.emoji, emoji_info.category);
        println!("   Distance: {:.4}", emoji_info.distance);
        println!("   LLM Prompt: \"Given the token '{}', is '{}' ({}) a fitting emoji representation? Explain why or why not.\"", 
            analyzed_tokens.iter().find(|(_, t)| t.multivector_str == emoji_info.emoji).map_or("N/A", |(token_str, _)| token_str.as_str()),
            emoji_info.emoji,
            emoji_info.category
        );
    }
}

// Helper function to convert AnalyzedFunction to Solana Function and CodeFile
fn convert_analyzed_functions_to_solana_functions(
    analyzed_functions: Vec<AnalyzedFunction>,
    ontology_instance: &Ontology,
    solana_payer_keypair: &Keypair,
    solana_program_id: Pubkey,
) -> (Vec<Function>, HashMap<String, CodeFile>, HashMap<String, SolanaClosestEmojiInfo>, Vec<BuyOrder>) {
    let mut solana_functions: Vec<Function> = Vec::new();
    let mut solana_code_files: HashMap<String, CodeFile> = HashMap::new();
    let mut solana_emojis: HashMap<String, SolanaClosestEmojiInfo> = HashMap::new();
    let mut solana_buy_orders: Vec<BuyOrder> = Vec::new();

    for func in analyzed_functions {
        let file_pubkey = Pubkey::find_program_address(&[b"code_file", func.file_path.as_bytes()], &solana_program_id).0;
        let func_pubkey = Pubkey::find_program_address(&[b"function", func.function_name.as_bytes()], &solana_program_id).0;

        // Create CodeFile if not exists
        solana_code_files.entry(func.file_path.clone()).or_insert_with(|| CodeFile {
            name: Path::new(&func.file_path).file_name().unwrap().to_string_lossy().into_owned(),
            path: func.file_path.clone(),
            function_refs: Vec::new(),
            buy_order_refs: Vec::new(),
            ontology: Pubkey::find_program_address(&[b"ontology", ontology_instance.id.as_bytes()], &solana_program_id).0,
            creator: solana_payer_keypair.pubkey(),
            timestamp: 0,
        });

        // Add function ref to CodeFile
        solana_code_files.get_mut(&func.file_path).unwrap().function_refs.push(func_pubkey);

        // Convert ClosestEmojiInfo to SolanaClosestEmojiInfo
        let mut solana_emoji_refs = Vec::new();
        let mut solana_emoji_distances = Vec::new();
        for emoji_info in func.closest_emojis {
            let emoji_pubkey = Pubkey::find_program_address(&[b"emoji_info", emoji_info.emoji.as_bytes()], &solana_program_id).0;
            solana_emoji_refs.push(emoji_pubkey);
            solana_emoji_distances.push(emoji_info.distance as f64);
            solana_emojis.entry(emoji_info.emoji.clone()).or_insert_with(|| SolanaClosestEmojiInfo {
                symbol: emoji_info.emoji,
                category: emoji_info.category,
                ontology: Pubkey::find_program_address(&[b"ontology", ontology_instance.id.as_bytes()], &solana_program_id).0,
                creator: solana_payer_keypair.pubkey(),
                timestamp: 0,
            });
        }

        // Hash multivector string for embedding_hash
        let mut hasher = Sha256::new();
        hasher.update(func.multivector_str.as_bytes());
        let embedding_hash: [u8; 32] = hasher.finalize().into();

        let solana_func = Function {
            name: func.function_name.clone(),
            semantic_summary: func.semantic_summary,
            code_snippet: func.code_snippet,
            sieve_address: func.sieve_address,
            embedding_hash,
            file: file_pubkey,
            emoji_refs: solana_emoji_refs,
            emoji_distances: solana_emoji_distances,
            buy_order_ref: Pubkey::default(), // Will be updated with actual buy order PDA
            ontology: Pubkey::find_program_address(&[b"ontology", ontology_instance.id.as_bytes()], &solana_program_id).0,
            creator: solana_payer_keypair.pubkey(),
            timestamp: 0,
        };

        // Create BuyOrder for this function
        let buy_order_id = format!("buyorder_{}", solana_func.name);
        let buy_order_pda = Pubkey::find_program_address(&[b"buy_order", buy_order_id.as_bytes()], &solana_program_id).0;
        solana_buy_orders.push(BuyOrder {
            id: buy_order_id,
            source_chunk: solana_func.code_snippet.clone(),
            vectorized_result: solana_func.embedding_hash,
            instance_ref: func_pubkey,
            status: BuyOrderStatus::Open,
            ontology: Pubkey::find_program_address(&[b"ontology", ontology_instance.id.as_bytes()], &solana_program_id).0,
            creator: solana_payer_keypair.pubkey(),
            timestamp: 0,
        });

        solana_functions.push(solana_func);
    }

    (solana_functions, solana_code_files, solana_emojis, solana_buy_orders)
}

// Helper function to convert AnalyzedTokens to Solana Terms
fn convert_analyzed_tokens_to_solana_terms(
    analyzed_tokens: HashMap<String, AnalyzedToken>,
    ontology_instance: &Ontology,
    solana_payer_keypair: &Keypair,
    solana_program_id: Pubkey,
) -> Vec<SolanaTerm> {
    let mut solana_terms: Vec<SolanaTerm> = Vec::new();

    for (token_str, token_data) in analyzed_tokens {
        let term_pubkey = Pubkey::find_program_address(&[b"term", token_str.as_bytes()], &solana_program_id).0;
        let mut hasher = Sha256::new();
        hasher.update(token_data.multivector_str.as_bytes());
        let embedding_hash: [u8; 32] = hasher.finalize().into();

        solana_terms.push(SolanaTerm {
            text: token_str,
            description: format!("Token count: {}", token_data.count),
            emoji_refs: Vec::new(), // No direct emoji refs for tokens yet
            function_refs: Vec::new(), // No direct function refs for tokens yet
            buy_order_ref: Pubkey::default(), // No buy order for terms yet
            ontology: Pubkey::find_program_address(&[b"ontology", ontology_instance.id.as_bytes()], &solana_program_id).0,
            creator: solana_payer_keypair.pubkey(),
            timestamp: 0,
        });
    }

    solana_terms
}