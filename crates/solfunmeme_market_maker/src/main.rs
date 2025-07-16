use anyhow::{Result, anyhow};
use clap::Parser;
use std::io::{self, BufRead, Write, Read};
use std::fs;
use serde::{Deserialize, Serialize};
use solfunmeme_models::{LlmProvider, LlmAccount, LlmTaskGroup};
use std::process::{Command, Stdio};

mod provider_selection;
use provider_selection::select_llm_provider;

#[derive(Parser)]
#[command(name = "solfunmeme_market_maker")]
#[command(about = "Matches LLM reflection tasks to providers and dispatches them.", long_about = None)]
struct Cli {
    #[arg(long, help = "Path to the LLM providers configuration TOML file.")]
    llm_config_path: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
struct LlmConfig {
    providers: Vec<LlmProvider>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load LLM configuration
    let llm_config_content = fs::read_to_to_string(&cli.llm_config_path)
        .map_err(|e| anyhow!("Failed to read LLM config file {}: {}", cli.llm_config_path.display(), e))?;
    let llm_config: LlmConfig = toml::from_str(&llm_config_content)
        .map_err(|e| anyhow!("Failed to parse LLM config file {}: {}", cli.llm_config_path.display(), e))?;

    eprintln!("Loaded LLM Providers: {:#?}", llm_config.providers);

    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut llm_task_groups: Vec<LlmTaskGroup> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let group: LlmTaskGroup = serde_json::from_str(&line)?;
        llm_task_groups.push(group);
    }

    eprintln!("Received {} LLM task groups from stdin.", llm_task_groups.len());

    if let Some((provider, account)) = select_llm_provider(&llm_config.providers) {
        println!("\nMarket Maker selected LLM Provider: {}", provider.name);
        println!("  Account ID: {}", account.id);
        println!("  Available Credits: {}", account.usage_vector.available_credits);
        println!("  Cost per token (input/output): {} / {}", account.usage_vector.cost_per_token_input, account.usage_vector.cost_per_token_output);
        println!("  Rate Limit (per minute): {}", account.usage_vector.rate_limit_per_minute);

        let command_str = account.command.as_ref().ok_or_else(|| anyhow!("Selected LLM account has no command defined."))?;
        let parts: Vec<&str> = command_str.split_whitespace().collect();
        let program = parts[0];
        let args = &parts[1..];

        for (i, group) in llm_task_groups.iter().enumerate() {
            eprintln!("\nDispatching Task Group {}: ({} chunks) to {} ({})", i + 1, group.code_chunks.len(), provider.name, account.id);

            let mut child = Command::new(program)
                .args(args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;

            let mut stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed to open stdin for LLM provider subprocess"))?;
            let mut stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to open stdout for LLM provider subprocess"))?;

            let task_json = serde_json::to_string(&group.payload)?;
            stdin.write_all(task_json.as_bytes())?;
            drop(stdin); // Close stdin to signal end of input

            let mut response_bytes = Vec::new();
            stdout.read_to_end(&mut response_bytes)?;

            let response_str = String::from_utf8_lossy(&response_bytes);
            println!("LLM Provider Response for Group {}:\n{}", i + 1, response_str);

            let status = child.wait()?;
            if !status.success() {
                eprintln!("[ERROR] LLM provider subprocess exited with non-zero status: {:?}", status.code());
            }
        }
    } else {
        println!("No suitable LLM provider found with available credits to dispatch tasks.");
    }

    Ok(())
}