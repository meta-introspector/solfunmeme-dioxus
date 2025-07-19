use zos_cli_modules::cli::Cli;
use zos_cli_modules::commands::Commands;
use zos_cli_modules::task_commands::TaskCommands;
use zos_cli_modules::chat_commands::ChatCommands;
use zos_cli_modules::config::Config;
use clap::Parser;
use std::path::PathBuf;

#[test]
fn test_cli_serde() {
    let cli_str = "zos vendor --path test_vendor";
    let cli = Cli::parse_from(cli_str.split_whitespace());
    let serialized = solfunmeme_serde_utils::serde_json::to_string(&cli).unwrap();
    let deserialized: Cli = solfunmeme_serde_utils::serde_json::from_str(&serialized).unwrap();
    assert_eq!(cli.command.to_string(), deserialized.command.to_string());
}

#[test]
fn test_commands_serde() {
    let command = Commands::Index { source: PathBuf::from("test_src"), output: PathBuf::from("test_output") };
    let serialized = serde_json::to_string(&command).unwrap();
    let deserialized: Commands = serde_json::from_str(&serialized).unwrap();
    assert_eq!(format!("{:?}", command), format!("{:?}", deserialized));
}

#[test]
fn test_task_commands_serde() {
    let task_command = TaskCommands::Run { task_name: "test_task".to_string() };
    let serialized = serde_json::to_string(&task_command).unwrap();
    let deserialized: TaskCommands = serde_json::from_str(&serialized).unwrap();
    assert_eq!(format!("{:?}", task_command), format!("{:?}", deserialized));
}

#[test]
fn test_chat_commands_serde() {
    let chat_command = ChatCommands::Search { query: "test_query".to_string(), limit: 5 };
    let serialized = serde_json::to_string(&chat_command).unwrap();
    let deserialized: ChatCommands = serde_json::from_str(&serialized).unwrap();
    assert_eq!(format!("{:?}", chat_command), format!("{:?}", deserialized));
}

#[test]
fn test_config_serde() {
    let config_str = r#"
[cli]
about = "A CLI for the Solfunmeme project"

[vendor]
default_path = "vendor"
message = "Vendoring dependencies to {}"

[index]
default_source = "src"
default_output = "tmp/tantivy_index"
message = "Indexing {} to {}"

[dedup]
default_source = "src"
default_threshold = 0.8
message = "Deduplicating {} with threshold {}"

[search]
default_limit = 10
message = "Searching for {} with limit {}"

[tasks]
list_message = "Listing tasks"
run_message = "Running task {}"
report_message = "Generating report in {} format"

[analyze]
default_format = "json"
default_output = "reports"
message = "Analyzing {} and outputting to {}"

[chat]
default_processed_chats = "processed_chats"
default_chat_index = "tmp/chat_index"
index_message = "Indexing chat from {} to {}"
index_error = "Error indexing chat:"
index_success = "Chat indexing complete."
vibe_ontology = "0,1,2,3,5,7"
vibe_output = "vibe_proof.json"
vibe_message = "Generating vibe proof from {} to {}"
vibe_error = "Error generating vibe proof:"
vibe_success = "Vibe proof generation complete."
search_message = "Searching chat for {} with limit {}"
search_found = "Found:"
search_error = "Error searching chat."
analyze_default_type = "themes"
analyze_default_format = "json"
analyze_message = "Analyzing chat for {} in {} format"
analyze_result = "Analysis result:"
analyze_error = "Error analyzing chat."
    "#;
    let deserialized: Config = toml::from_str(config_str).unwrap();
    let serialized = toml::to_string(&deserialized).unwrap();
    let re_deserialized: Config = toml::from_str(&serialized).unwrap();
    assert_eq!(format!("{:?}", deserialized), format!("{:?}", re_deserialized));
}
