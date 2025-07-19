use solfunmeme_serde_utils::serde::{Deserialize, Serialize};
use solfunmeme_serde_utils::toml;


#[derive(Debug, Deserialize, Serialize)]
pub struct CliConfig {
    pub about: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VendorConfig {
    pub default_path: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IndexConfig {
    pub default_source: String,
    pub default_output: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DedupConfig {
    pub default_source: String,
    pub default_threshold: f64,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchConfig {
    pub default_limit: usize,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TasksConfig {
    pub list_message: String,
    pub run_message: String,
    pub report_message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyzeConfig {
    pub default_format: String,
    pub default_output: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatConfig {
    pub default_processed_chats: String,
    pub default_chat_index: String,
    pub index_message: String,
    pub index_error: String,
    pub index_success: String,
    pub vibe_ontology: String,
    pub vibe_output: String,
    pub vibe_message: String,
    pub vibe_error: String,
    pub vibe_success: String,
    pub search_message: String,
    pub search_found: String,
    pub search_error: String,
    pub analyze_default_type: String,
    pub analyze_default_format: String,
    pub analyze_message: String,
    pub analyze_result: String,
    pub analyze_error: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub cli: CliConfig,
    pub vendor: VendorConfig,
    pub index: IndexConfig,
    pub dedup: DedupConfig,
    pub search: SearchConfig,
    pub tasks: TasksConfig,
    pub analyze: AnalyzeConfig,
    pub chat: ChatConfig,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = std::fs::read_to_string("zos_config.toml")?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}
