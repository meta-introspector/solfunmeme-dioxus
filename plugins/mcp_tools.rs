use super::*;
use serde_json::{json, Value};

pub struct McpPlugin;

impl ZOSPlugin for McpPlugin {
    fn name(&self) -> &'static str { "mcp-tools" }
    fn version(&self) -> &'static str { "0.2.0" }
    fn commands(&self) -> Vec<&'static str> { vec!["list-tools", "invoke", "schema"] }

    fn execute(&self, command: &str, args: Vec<String>) -> Result<Value, String> {
        match command {
            "list-tools" => Ok(json!(["lean4-check", "rust-parse", "stego-encode", "zkperf-witness"])),
            "schema" => Ok(json!({"type": "mcp-tool-registry", "version": "0.2.0"})),
            "invoke" => Ok(json!({"status": "ok", "tool": args.first().unwrap_or(&"none".into())})),
            _ => Err(format!("unknown: {}", command)),
        }
    }

    fn render(&self) -> Vec<GuiComponent> {
        vec![
            GuiComponent::Heading { level: 2, text: "🔧 MCP Tools".into() },
            GuiComponent::Table {
                headers: vec!["Tool".into(), "Status".into()],
                rows: vec![
                    vec!["lean4-check".into(), "✅".into()],
                    vec!["rust-parse".into(), "✅".into()],
                    vec!["stego-encode".into(), "✅".into()],
                    vec!["zkperf-witness".into(), "✅".into()],
                ],
            },
            GuiComponent::Button { label: "Invoke Tool".into(), command: "invoke".into() },
        ]
    }

    fn state(&self) -> Vec<(u64, u32)> { vec![(29, 1), (41, 1)] } // monitor + test
}
