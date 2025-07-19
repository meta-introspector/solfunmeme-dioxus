use solfunmeme_dioxus::zos_modules::config::*;
use std::path::PathBuf;

#[test]
fn test_config_serialization_deserialization() {
    let config = Config {
        cli: CliConfig { about: "Test CLI".to_string() },
        vendor: VendorConfig { default_path: "test_vendor".to_string(), message: "Vendor message: {}".to_string() },
        index: IndexConfig { default_source: "test_src".to_string(), default_output: "test_output".to_string(), message: "Index message: {} {}".to_string() },
        dedup: DedupConfig { default_source: "test_dedup_src".to_string(), default_threshold: 0.9, message: "Dedup message: {} {}".to_string() },
        search: SearchConfig { default_limit: 20, message: "Search message: {} {}".to_string() },
        tasks: TasksConfig { list_message: "List tasks".to_string(), run_message: "Run task: {}".to_string(), report_message: "Report format: {}".to_string() },
        analyze: AnalyzeConfig { default_format: "xml".to_string(), default_output: "test_reports".to_string(), message: "Analyze message: {} {}".to_string() },
        chat: ChatConfig {
            default_processed_chats: "test_processed_chats".to_string(),
            default_chat_index: "test_chat_index".to_string(),
            index_message: "Chat index message: {} {}".to_string(),
            index_error: "Chat index error: {}".to_string(),
            index_success: "Chat index success".to_string(),
            vibe_ontology: "0,1,2".to_string(),
            vibe_output: "test_vibe_proof.json".to_string(),
            vibe_message: "Chat vibe message: {} {}".to_string(),
            vibe_error: "Chat vibe error: {}".to_string(),
            vibe_success: "Chat vibe success".to_string(),
            search_message: "Chat search message: {} {}".to_string(),
            search_found: "Chat search found: {}".to_string(),
            search_error: "Chat search error".to_string(),
            analyze_default_type: "sentiment".to_string(),
            analyze_default_format: "csv".to_string(),
            analyze_message: "Chat analyze message: {} {}".to_string(),
            analyze_result: "Chat analyze result: {}".to_string(),
            analyze_error: "Chat analyze error".to_string(),
        },
    };

    let serialized = toml::to_string(&config).unwrap();
    let deserialized: Config = toml::from_str(&serialized).unwrap();

    assert_eq!(config.cli.about, deserialized.cli.about);
    assert_eq!(config.vendor.default_path, deserialized.vendor.default_path);
    assert_eq!(config.index.default_source, deserialized.index.default_source);
    assert_eq!(config.dedup.default_threshold, deserialized.dedup.default_threshold);
    assert_eq!(config.search.default_limit, deserialized.search.default_limit);
    assert_eq!(config.tasks.list_message, deserialized.tasks.list_message);
    assert_eq!(config.analyze.default_format, deserialized.analyze.default_format);
    assert_eq!(config.chat.vibe_ontology, deserialized.chat.vibe_ontology);
}

#[test]
fn test_config_load() {
    // Create a dummy zos_config.toml for testing
    let toml_content = r#"
[cli]
about = "Test CLI"

[vendor]
default_path = "test_vendor"
message = "Vendor message: {}"

[index]
default_source = "test_src"
default_output = "test_output"
message = "Index message: {} {}"

[dedup]
default_source = "test_dedup_src"
default_threshold = 0.9
message = "Dedup message: {} {}"

[search]
default_limit = 20
message = "Search message: {} {}"

[tasks]
list_message = "List tasks"
run_message = "Run task: {}"
report_message = "Report format: {}"

[analyze]
default_format = "xml"
default_output = "test_reports"
message = "Analyze message: {} {}"

[chat]
default_processed_chats = "test_processed_chats"
default_chat_index = "test_chat_index"
index_message = "Chat index message: {} {}"
index_error = "Chat index error: {}"
index_success = "Chat index success"
vibe_ontology = "0,1,2"
vibe_output = "test_vibe_proof.json"
vibe_message = "Chat vibe message: {} {}"
vibe_error = "Chat vibe error: {}"
vibe_success = "Chat vibe success"
search_message = "Chat search message: {} {}"
search_found = "Chat search found: {}"
search_error = "Chat search error"
analyze_default_type = "sentiment"
analyze_default_format = "csv"
analyze_message = "Chat analyze message: {} {}"
analyze_result = "Chat analyze result: {}"
analyze_error = "Chat analyze error"
    "#;

    std::fs::write("zos_config.toml", toml_content).unwrap();

    let loaded_config = Config::load().unwrap();

    assert_eq!(loaded_config.cli.about, "Test CLI");
    assert_eq!(loaded_config.vendor.default_path, "test_vendor");
    assert_eq!(loaded_config.index.default_source, "test_src");
    assert_eq!(loaded_config.dedup.default_threshold, 0.9);
    assert_eq!(loaded_config.search.default_limit, 20);
    assert_eq!(loaded_config.tasks.list_message, "List tasks");
    assert_eq!(loaded_config.analyze.default_format, "xml");
    assert_eq!(loaded_config.chat.vibe_ontology, "0,1,2");

    // Clean up the dummy config file
    std::fs::remove_file("zos_config.toml").unwrap();
}
