use super::*;
use serde_json::{json, Value};

pub struct RustParserPlugin;

impl ZOSPlugin for RustParserPlugin {
    fn name(&self) -> &'static str { "rust-parser" }
    fn version(&self) -> &'static str { "0.2.0" }
    fn commands(&self) -> Vec<&'static str> { vec!["parse", "functions", "structs"] }

    fn execute(&self, command: &str, args: Vec<String>) -> Result<Value, String> {
        let src = args.first().map(|s| s.as_str()).unwrap_or("");
        match command {
            "parse" => Ok(json!({"status": "parsed", "len": src.len()})),
            "functions" => Ok(json!({"count": src.matches("fn ").count()})),
            "structs" => Ok(json!({"count": src.matches("struct ").count()})),
            _ => Err(format!("unknown: {}", command)),
        }
    }

    fn render(&self) -> Vec<GuiComponent> {
        vec![
            GuiComponent::Heading { level: 2, text: "🦀 Rust Parser".into() },
            GuiComponent::Code { language: "rust".into(), source: "// paste code here".into() },
            GuiComponent::Button { label: "Parse".into(), command: "parse".into() },
        ]
    }

    fn state(&self) -> Vec<(u64, u32)> { vec![(23, 1), (31, 1)] } // cargo + build
}
