use anyhow::{Result, anyhow};
use solfunmeme_search_tantivy::SearchIndex;
use solfunmeme_models::{LlmTaskGroup, LlmTaskPayload};

mod cli_args;
mod llm_config_loader;
mod embedding_processing;
mod task_grouping;
mod task_generation;

use cli_args::Cli;
use llm_config_loader::{load_llm_config, select_llm_provider};
use embedding_processing::{retrieve_and_process_embeddings};
use task_grouping::group_chunks_by_similarity;
use task_generation::generate_llm_task_groups;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let index_path = &cli.index_path;
    if !index_path.exists() {
        return Err(anyhow!("Index path does not exist: {}", index_path.display()));
    }

    // Load LLM configuration
    let llm_config = load_llm_config(&cli.llm_config_path)?;
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

    let chunks_with_embeddings = retrieve_and_process_embeddings(&search_index, cli.limit)?;

    if chunks_with_embeddings.is_empty() {
        eprintln!("No chunks with embeddings found to analyze.");
        return Ok(());
    }

    let groups = group_chunks_by_similarity(chunks_with_embeddings, cli.similarity_threshold)?;

    let llm_task_groups = generate_llm_task_groups(
        &cli.task_type,
        cli.code_snippet,
        groups,
    )?;

    if cli.output_format == "json" {
        println!("{}", serde_json::to_string_pretty(&llm_task_groups)?);
    } else {
        println!("\n--- LLM Planning Groups (Similarity Threshold: {}) ---", cli.similarity_threshold);
        for (i, group) in llm_task_groups.iter().enumerate() {
            println!("\nGroup {}: ({} chunks)", i + 1, group.payload.code_chunks.len()); // Assuming CodeReflection for text output
            println!("  Task Type: {}", group.task_type);
            if let LlmTaskPayload::CodeReflection(code_reflection_task) = &group.payload {
                println!("  Multivector: {:?}", code_reflection_task.multivector);
                for chunk_content in &code_reflection_task.code_chunks {
                    println!("  - {}", &chunk_content[..std::cmp::min(chunk_content.len(), 100)].replace("\n", " "));
                }
            } else if let LlmTaskPayload::CliffordOperation(clifford_op_request) = &group.payload {
                println!("  Clifford Operation: {}", clifford_op_request.operation);
                println!("  Scalar Value: {}", clifford_op_request.scalar_value);
                println!("  Vector Values: {:?}", clifford_op_request.vector_values);
            } else if let LlmTaskPayload::CodeEvolution(code_evolution_task) = &group.payload {
                println!("  Code Snippet: {}", &code_evolution_task.code_snippet[..std::cmp::min(code_evolution_task.code_snippet.len(), 100)].replace("\n", " "));
                println!("  Meme Token: {:?}", code_evolution_task.meme_token);
            }
        }
    }

    Ok(())
}