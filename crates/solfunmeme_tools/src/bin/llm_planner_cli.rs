use anyhow::{Result, anyhow};
use clap::Parser;
use std::path::PathBuf;
use solfunmeme_search_tantivy::SearchIndex;
use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_models::{LlmProvider, LlmAccount, UsageVector};
use candle_core::Tensor;
use std::fs;
use toml;
use serde::Serialize;
use solfunmeme_models::LlmTaskGroup;
use solfunmeme_clifford::{BertCliffordEncoder, SolMultivector, BertConfig as CliffordBertConfig};

#[derive(Parser)]
#[command(name = "llm_planner")]
#[command(about = "Plans LLM reflection tasks by grouping code chunks based on embeddings.", long_about = None)]
struct Cli {
    #[arg(long, help = "Path to the Tantivy index directory.")]
    index_path: PathBuf,
    #[arg(long, default_value = "100", help = "Number of top chunks to retrieve for analysis.")]
    limit: usize,
    #[arg(long, default_value = "0.7", help = "Cosine similarity threshold for grouping chunks.")]
    similarity_threshold: f32,
    #[arg(long, help = "Path to the LLM providers configuration TOML file.")]
    llm_config_path: PathBuf,
    #[arg(long, default_value = "text", help = "Output format: text or json.")]
    output_format: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct LlmConfig {
    providers: Vec<LlmProvider>,
}

fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    if v1.is_empty() || v2.is_empty() || v1.len() != v2.len() {
        return 0.0;
    }

    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(&a, &b)| a * b).sum();
    let magnitude1: f32 = v1.iter().map(|&a| a * a).sum::<f32>().sqrt();
    let magnitude2: f32 = v2.iter().map(|&b| b * b).sum::<f32>().sqrt();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        0.0
    } else {
        dot_product / (magnitude1 * magnitude2)
    }
}

fn select_llm_provider<'a>(providers: &'a [LlmProvider]) -> Option<(&'a LlmProvider, &'a LlmAccount)> {
    let mut best_provider: Option<(&LlmProvider, &LlmAccount)> = None;

    for provider in providers {
        for account in &provider.accounts {
            // Simple strategy: prioritize accounts with available credits
            // Then, lowest cost per token (input + output)
            // Then, highest rate limit
            if account.usage_vector.available_credits > 0.0 {
                let current_cost = account.usage_vector.cost_per_token_input + account.usage_vector.cost_per_token_output;
                match best_provider {
                    Some((_, best_account)) => {
                        let best_cost = best_account.usage_vector.cost_per_token_input + best_account.usage_vector.cost_per_token_output;
                        if current_cost < best_cost {
                            best_provider = Some((provider, account));
                        } else if current_cost == best_cost && account.usage_vector.rate_limit_per_minute > best_account.usage_vector.rate_limit_per_minute {
                            best_provider = Some((provider, account));
                        }
                    }
                    None => {
                        best_provider = Some((provider, account));
                    }
                }
            }
        }
    }
    best_provider
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let index_path = &cli.index_path;
    if !index_path.exists() {
        return Err(anyhow!("Index path does not exist: {}", index_path.display()));
    }

    // Load LLM configuration
    let llm_config_content = fs::read_to_string(&cli.llm_config_path)
        .map_err(|e| anyhow!("Failed to read LLM config file {}: {}", cli.llm_config_path.display(), e))?;
    let llm_config: LlmConfig = toml::from_str(&llm_config_content)
        .map_err(|e| anyhow!("Failed to parse LLM config file {}: {}", cli.llm_config_path.display(), e))?;

    eprintln!("Loaded LLM Providers: {:#?}", llm_config.providers);

    if let Some((provider, account)) = select_llm_provider(&llm_config.providers) {
        eprintln!("\nSelected LLM Provider: {}", provider.name);
        eprintln!("  Account ID: {}", account.id);
        eprintln!("  Available Credits: {}", account.usage_vector.available_credits);
        eprintln!("  Cost per token (input/output): {} / {}", account.usage_vector.cost_per_token_input, account.usage_vector.cost_per_token_output);
        eprintln!("  Rate Limit (per minute): {}", account.usage_vector.rate_limit_per_minute);
    } else {
        eprintln!("No suitable LLM provider found with available credits.");
    }

    let search_index = SearchIndex::new(index_path)?;

    let reader = search_index.index.reader()?;
    let searcher = reader.searcher();
    let all_docs = searcher.search(&tantivy::query::AllQuery, &tantivy::collector::TopDocs::with_limit(cli.limit))?;

    eprintln!("Retrieved {} chunks from the index.", all_docs.len());

    let schema = search_index.schema;
    let content_field = schema.get_field("content")?;
    let embedding_field = schema.get_field("embedding")?;

    let mut chunks_with_embeddings: Vec<(String, Vec<f32>)> = Vec::new();

    for (_score, doc_address) in all_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        let content = retrieved_doc
            .get_first(content_field)
            .and_then(|v| v.as_value().as_str())
            .unwrap_or("")
            .to_string();

        let embedding_bytes = retrieved_doc
            .get_first(embedding_field)
            .and_then(|v| v.as_bytes());

        if let Some(bytes) = embedding_bytes {
            let embedding: Vec<f32> = bytes
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
                .collect();
            chunks_with_embeddings.push((content, embedding));
        } else {
            eprintln!("[WARN] Chunk has no embedding and will be skipped for grouping: {}", &content[..std::cmp::min(content.len(), 50)]);
        }
    }

    if chunks_with_embeddings.is_empty() {
        eprintln!("No chunks with embeddings found to analyze.");
        return Ok(());
    }

    let mut processed_indices = std::collections::HashSet::new();
    let mut groups: Vec<(Vec<String>, SolMultivector)> = Vec::new();

    let bert_clifford_encoder = BertCliffordEncoder::new(CliffordBertConfig::default());

    for i in 0..chunks_with_embeddings.len() {
        if processed_indices.contains(&i) {
            continue;
        }

        let (current_content, current_embedding) = &chunks_with_embeddings[i];
        let mut current_group_contents = vec![current_content.clone()];
        let mut current_group_embedding = bert_clifford_encoder.encode_embedding(current_embedding)?;
        processed_indices.insert(i);

        for j in (i + 1)..chunks_with_embeddings.len() {
            if processed_indices.contains(&j) {
                continue;
            }

            let (other_content, other_embedding) = &chunks_with_embeddings[j];
            let similarity = cosine_similarity(current_embedding, other_embedding);

            if similarity >= cli.similarity_threshold {
                current_group_contents.push(other_content.clone());
                // For simplicity, we'll just use the first chunk's multivector for the group
                // A more sophisticated approach might average or combine multivectors
                processed_indices.insert(j);
            }
        }
        groups.push((current_group_contents, current_group_embedding));
    }

    if cli.output_format == "json" {
        let llm_task_groups: Vec<LlmTaskGroup> = if cli.task_type == "clifford_operation" {
            // Create a single Clifford operation task for demonstration
            vec![LlmTaskGroup {
                task_type: "clifford_operation".to_string(),
                payload: LlmTaskPayload::CliffordOperation(CliffordOperationRequest {
                    operation: "create_scalar_multivector".to_string(),
                    scalar_value: 42.0,
                    vector_values: Vec::new(),
                    input_multivector: None,
                }),
            }]
        } else if cli.task_type == "code_evolution" {
            let code_snippet = cli.code_snippet.ok_or_else(|| anyhow!("Code snippet is required for code_evolution task."))?;
            vec![LlmTaskGroup {
                task_type: "code_evolution".to_string(),
                payload: LlmTaskPayload::CodeEvolution(CodeEvolutionTask {
                    code_snippet,
                    meme_token: None, // MemeToken can be populated later or passed as input
                }),
            }]
        } else {
            groups.into_iter().map(|(group_contents, group_multivector)| {
                LlmTaskGroup {
                    task_type: "code_reflection".to_string(),
                    payload: LlmTaskPayload::CodeReflection(CodeReflectionTask {
                        code_chunks: group_contents,
                        multivector: group_multivector,
                    }),
                }
            }).collect()
        };
        println!("{}", serde_json::to_string_pretty(&llm_task_groups)?);
    } else {
        println!("\n--- LLM Planning Groups (Similarity Threshold: {}) ---", cli.similarity_threshold);
        for (i, (group_contents, group_multivector)) in groups.iter().enumerate() {
            println!("\nGroup {}: ({} chunks)", i + 1, group_contents.len());
            println!("  Multivector: {:?}", group_multivector);
            for chunk_content in group_contents {
                println!("  - {}", &chunk_content[..std::cmp::min(chunk_content.len(), 100)].replace("\n", " "));
            }
        }
    }

    Ok(())
}
