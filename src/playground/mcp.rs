use crate::model::lean::style::Styles;
use chrono::Utc;
use dioxus::prelude::*;
use rrust_kontekst_base::{get_mcp_tools, get_mcp_tools_schema, invoke_mcp_tool, McpToolInfo};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

const AI_PLACEHOLDER: &str = "AI: invoke_tool('embedding_ops', {'query': 'hello world'})";
const AI_PREFIX: &str = "AI: ";
const TOOL_FUNCTION: &str = "invoke_tool";
const EXAMPLE_TOOL: &str = "embedding_ops";
const EXAMPLE_QUERY: &str = "hello world";

const INPUT_CLASSES: &str =
    "flex-1 px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white";

#[derive(Clone, Debug, PartialEq)]
pub enum PlaygroundMode {
    HumanUI,
    McpInterface,
    Hybrid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct McpQuery {
    pub tool_name: String,
    pub parameters: Value,
    pub result: Option<Result<Value, String>>,
    pub timestamp: String,
}

#[derive(Clone, Debug, PartialEq)]
struct McpToolView {
    pub component_name: String,
    pub tool_name: String,
    pub menu_type: String,
    pub label: String,
    pub emoji: String,
    pub description: String,
    pub visible: bool,
    pub order: i32,
    pub mcp_enabled: bool,
    pub parameters: Vec<String>,
    pub returns: String,
}

impl McpToolView {
    fn from_local(info: &McpToolInfo) -> Self {
        Self {
            component_name: info.component_name.to_string(),
            tool_name: info.tool_name.to_string(),
            menu_type: info.menu_type.to_string(),
            label: info.label.to_string(),
            emoji: info.emoji.to_string(),
            description: info.description.to_string(),
            visible: info.visible,
            order: info.order,
            mcp_enabled: info.mcp_enabled,
            parameters: info
                .parameters
                .iter()
                .map(|item| (*item).to_string())
                .collect(),
            returns: info.returns.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GatewayToolSpec {
    name: String,
    title: String,
    description: String,
    #[serde(default)]
    input_schema: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct GatewayToolsResponse {
    tools: Vec<GatewayToolSpec>,
}

fn gateway_tool_parameters(schema: &Value) -> Vec<String> {
    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        let mut params = properties
            .keys()
            .map(|name| name.to_string())
            .collect::<Vec<_>>();
        params.sort();
        params
    } else {
        Vec::new()
    }
}

fn gateway_tool_to_view(spec: GatewayToolSpec) -> McpToolView {
    McpToolView {
        component_name: "itir_mcp".to_string(),
        tool_name: spec.name,
        menu_type: "core".to_string(),
        label: spec.title.clone(),
        emoji: "🧩".to_string(),
        description: spec.description,
        visible: true,
        order: 0,
        mcp_enabled: true,
        parameters: gateway_tool_parameters(&spec.input_schema),
        returns: "ok".to_string(),
    }
}

fn local_tools() -> Vec<McpToolView> {
    get_mcp_tools("core")
        .map(|tools| {
            tools
                .into_iter()
                .map(|tool| McpToolView::from_local(&tool))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
fn tool_list_endpoint() -> String {
    format!("{}/tools", crate::mcp_gateway::gateway_base_url())
}

#[cfg(not(target_arch = "wasm32"))]
fn tool_call_endpoint() -> String {
    format!("{}/call", crate::mcp_gateway::gateway_base_url())
}

#[cfg(target_arch = "wasm32")]
fn tool_list_endpoint() -> String {
    "/api/itir-mcp/tools".to_string()
}

#[cfg(target_arch = "wasm32")]
fn tool_call_endpoint() -> String {
    "/api/itir-mcp/call".to_string()
}

#[cfg(not(target_arch = "wasm32"))]
async fn load_gateway_tools() -> Result<Vec<McpToolView>, String> {
    let response = reqwest::Client::new()
        .get(tool_list_endpoint())
        .send()
        .await
        .map_err(|error| format!("gateway list request failed: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| String::new());
        return Err(format!("gateway list request failed ({status}): {body}"));
    }

    let payload: GatewayToolsResponse = response
        .json()
        .await
        .map_err(|error| format!("gateway list response parse failed: {error}"))?;
    Ok(payload
        .tools
        .into_iter()
        .map(gateway_tool_to_view)
        .collect())
}

#[cfg(target_arch = "wasm32")]
async fn load_gateway_tools() -> Result<Vec<McpToolView>, String> {
    Err("MCP gateway is not available in browser builds. Using local tools.".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
async fn call_gateway_tool(name: &str, arguments: Value) -> Result<Value, String> {
    let body = json!({
        "name": name,
        "arguments": arguments
    });

    let response = reqwest::Client::new()
        .post(tool_call_endpoint())
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .map_err(|error| format!("gateway call request failed: {error}"))?;

    let response_text = response
        .text()
        .await
        .map_err(|error| format!("gateway response read failed: {error}"))?;

    let payload: Value = serde_json::from_str(&response_text)
        .map_err(|error| format!("gateway response parse failed: {error}: {response_text}"))?;

    if let Some(message) = payload.get("error").and_then(|error| {
        error
            .get("message")
            .and_then(Value::as_str)
            .map(ToString::to_string)
    }) {
        return Err(message);
    }

    payload
        .get("result")
        .cloned()
        .ok_or_else(|| "gateway response missing result field".to_string())
}

#[cfg(target_arch = "wasm32")]
async fn call_gateway_tool(name: &str, _arguments: Value) -> Result<Value, String> {
    Err(format!(
        "MCP gateway is not available in browser builds for tool: {name}"
    ))
}

async fn invoke_mcp_tool_with_fallback(name: String, arguments: Value) -> Result<Value, String> {
    match call_gateway_tool(&name, arguments.clone()).await {
        Ok(result) => Ok(result),
        Err(gateway_error) => match invoke_mcp_tool(&name, arguments).await {
            Ok(result) => Ok(result),
            Err(local_error) => Err(format!("gateway: {gateway_error}; local: {}", local_error)),
        },
    }
}

#[component]
fn ModeSelector(mode: Signal<PlaygroundMode>) -> Element {
    rsx! {
        div { class: "mb-6 flex gap-4",
            h1 { class: "text-3xl font-bold mb-4", "🎮 MCP Tool Playground" }

            div { class: "flex gap-2",
                ModeButton {
                    mode: mode,
                    target_mode: PlaygroundMode::HumanUI,
                    label: "👤 Human UI"
                }
                ModeButton {
                    mode: mode,
                    target_mode: PlaygroundMode::McpInterface,
                    label: "🤖 MCP Interface"
                }
                ModeButton {
                    mode: mode,
                    target_mode: PlaygroundMode::Hybrid,
                    label: "🔀 Hybrid Mode"
                }
            }
        }
    }
}

#[component]
fn ModeButton(mode: Signal<PlaygroundMode>, target_mode: PlaygroundMode, label: String) -> Element {
    let is_active = *mode.read() == target_mode;

    rsx! {
        button {
            class: format!("px-4 py-2 rounded {} {}",
                Styles::primary_button(),
                if is_active { "ring-2 ring-blue-400" } else { "" }
            ),
            onclick: move |_| mode.set(target_mode.clone()),
            "{label}"
        }
    }
}

#[component]
fn McpToolsPanel(
    mcp_tools: Vec<McpToolView>,
    active_tool: Signal<Option<String>>,
    mcp_queries: Signal<Vec<McpQuery>>,
) -> Element {
    rsx! {
        div { class: "bg-gray-800 rounded-lg p-6",
            h2 { class: "text-xl font-bold mb-4 flex items-center gap-2",
                "🔍 Available MCP Tools"
                span { class: "text-sm text-gray-400", "({mcp_tools.len()} tools)" }
            }

            SchemaExportButton {}

            div { class: "grid gap-3 max-h-96 overflow-y-auto",
                {mcp_tools.iter().map(|tool| {
                    rsx! {
                        ToolCard {
                            key: "{tool.tool_name}",
                            tool: tool.clone(),
                            active_tool: active_tool,
                            mcp_queries: mcp_queries
                        }
                    }
                })}
            }
        }
    }
}

#[component]
fn SchemaExportButton() -> Element {
    rsx! {
        div { class: "mb-4",
            button {
                class: "{Styles::primary_button()} text-sm",
                onclick: move |_| {
                    let schema = get_mcp_tools_schema("core");
                    println!("MCP Schema: {:?}", schema);
                },
                "📋 Export MCP Schema"
            }
        }
    }
}

#[component]
fn ToolCardHeader(tool: McpToolView) -> Element {
    rsx! {
        div { class: "flex items-start justify-between",
            div {
                div { class: "font-medium flex items-center gap-2",
                    span { "{tool.emoji}" }
                    span { "{tool.label}" }
                    if tool.mcp_enabled {
                        span { class: "text-xs bg-green-600 px-2 py-1 rounded", "MCP" }
                    }
                }
                div { class: "text-sm text-gray-300 mt-1", "{tool.description}" }
                div { class: "text-xs text-gray-400 mt-2",
                    "Tool: " code { class: "bg-gray-800 px-1 rounded", "{tool.tool_name}" }
                }
            }
        }
    }
}

#[component]
fn ToolCardDetails(tool: McpToolView, mcp_queries: Signal<Vec<McpQuery>>) -> Element {
    let params = json!({});
    let tool_name = tool.tool_name.clone();

    rsx! {
        div { class: "mt-3 pt-3 border-t border-gray-600",
            div { class: "text-sm space-y-2",
                div {
                    strong { "Parameters:" }
                    if tool.parameters.is_empty() {
                        span { class: "text-gray-400 ml-2", "none" }
                    } else {
                        ul { class: "list-disc list-inside ml-2 text-gray-300",
                            {tool.parameters.iter().map(|param| rsx! {
                                li { key: "{param}", "{param}" }
                            })}
                        }
                    }
                }
                div {
                    strong { "Returns:" }
                    span { class: "text-gray-300 ml-2", "{tool.returns}" }
                }
            }

            QuickInvokeButton {
                tool_name: tool_name,
                default_args: params,
                mcp_queries: mcp_queries
            }
        }
    }
}

#[component]
fn QuickInvokeButton(
    tool_name: String,
    default_args: Value,
    mcp_queries: Signal<Vec<McpQuery>>,
) -> Element {
    rsx! {
        button {
            class: "{Styles::primary_button()} text-sm mt-2",
            onclick: move |_| {
                let tool_name = tool_name.clone();
                let arguments = default_args.clone();
                let mut mcp_queries = mcp_queries;

                spawn(async move {
                    let result = invoke_mcp_tool_with_fallback(tool_name.clone(), arguments).await;
                    mcp_queries.write().push(McpQuery {
                        tool_name,
                        parameters: arguments,
                        result: Some(result),
                        timestamp: Utc::now().format("%H:%M:%S").to_string(),
                    });
                });
            },
            "⚡ Quick Invoke"
        }
    }
}

#[component]
fn ToolCard(
    tool: McpToolView,
    active_tool: Signal<Option<String>>,
    mcp_queries: Signal<Vec<McpQuery>>,
) -> Element {
    let is_active = active_tool.read().as_ref() == Some(&tool.tool_name.to_string());

    rsx! {
        div {
            class: format!(
                "p-3 rounded border cursor-pointer transition-colors {}",
                if is_active { "bg-blue-900 border-blue-400" } else { "bg-gray-700 border-gray-600 hover:bg-gray-600" }
            ),
            onclick: move |_| {
                if is_active {
                    active_tool.set(None);
                } else {
                    active_tool.set(Some(tool.tool_name.to_string()));
                }
            },

            ToolCardHeader { tool: tool.clone() }

            if is_active {
                ToolCardDetails { tool: tool.clone(), mcp_queries: mcp_queries }
            }
        }
    }
}

#[component]
fn HumanUIPanel(mcp_tools: Vec<McpToolView>) -> Element {
    rsx! {
        div { class: "bg-gray-800 rounded-lg p-6",
            h2 { class: "text-xl font-bold mb-4", "🎯 Human Interface" }

            div { class: "grid gap-3",
                {mcp_tools.iter().filter(|tool| tool.visible).map(|tool| {
                    rsx! {
                        HumanUIButton {
                            key: "{tool.component_name}",
                            label: format!("{} {}", tool.emoji, tool.label),
                            visible: tool.visible,
                        }
                    }
                })}
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct HumanUIButtonProps {
    label: String,
    visible: bool,
}

#[component]
fn HumanUIButton(props: HumanUIButtonProps) -> Element {
    let _ = props.visible;
    rsx! {
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| {
                println!("activating {label}", label = props.label);
            },
            "{props.label}"
        }
    }
}

#[component]
fn QueryLog(mcp_queries: Signal<Vec<McpQuery>>) -> Element {
    rsx! {
        div { class: "mt-6 bg-gray-800 rounded-lg p-6",
            h2 { class: "text-xl font-bold mb-4", "📜 MCP Query Log" }

            div { class: "space-y-3 max-h-64 overflow-y-auto",
                {mcp_queries.read().iter().rev().map(|query| {
                    rsx! {
                        QueryLogEntry {
                            key: "{query.timestamp}-{query.tool_name}",
                            query: query.clone()
                        }
                    }
                })}
            }
        }
    }
}

#[component]
fn QueryLogEntry(query: McpQuery) -> Element {
    rsx! {
        div {
            class: "p-3 bg-gray-700 rounded border-l-4 border-blue-500",

            div { class: "flex justify-between items-start mb-2",
                span { class: "font-medium", "🔧 {query.tool_name}" }
                span { class: "text-xs text-gray-400", "{query.timestamp}" }
            }

            if let Some(ref result) = query.result {
                match result {
                    Ok(value) => {
                        rsx! {
                            div { class: "text-sm text-green-400",
                                "✅ Success: "
                                code {
                                    class: "bg-gray-800 px-2 py-1 rounded",
                                    "{serde_json::to_string_pretty(value).unwrap_or_default()}"
                                }
                            }
                        }
                    }
                    Err(error) => {
                        rsx! {
                            div { class: "text-sm text-red-400", "❌ Error: {error}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QueryHelpText() -> Element {
    rsx! {
        div { class: "mt-4 text-sm text-gray-400",
            "💡 AI can discover tools via "
            code { class: "bg-gray-700 px-1 rounded", "get_mcp_tools_schema()" }
            " and invoke them via "
            code { class: "bg-gray-700 px-1 rounded", "invoke_mcp_tool(name, params)" }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct PlaceholderConfig {
    pub prefix: String,
    pub function_name: String,
    pub tool_name: String,
    pub example_params: HashMap<String, String>,
}

impl Default for PlaceholderConfig {
    fn default() -> Self {
        let mut params = HashMap::new();
        params.insert("query".to_string(), "hello world".to_string());
        Self {
            prefix: AI_PREFIX.to_string(),
            function_name: TOOL_FUNCTION.to_string(),
            tool_name: EXAMPLE_TOOL.to_string(),
            example_params: params,
        }
    }
}

fn format_placeholder(tool_name: &str, example_query: &str) -> String {
    format!(
        "{}{}('{}', {{'query': '{}'}})",
        AI_PREFIX, TOOL_FUNCTION, tool_name, example_query
    )
}

#[component]
fn ConfigurableQueryInput(query_input: Signal<String>, config: PlaceholderConfig) -> Element {
    let params_str = config
        .example_params
        .iter()
        .map(|(k, v)| format!("'{}': '{}'", k, v))
        .collect::<Vec<_>>()
        .join(", ");

    let placeholder = format!(
        "{}{}('{}', {{{}}})",
        config.prefix, config.function_name, config.tool_name, params_str
    );

    rsx! {
        input {
            class: INPUT_CLASSES,
            placeholder: "{placeholder}",
            value: "{query_input.read()}",
            oninput: move |evt| query_input.set(evt.value())
        }
    }
}

#[component]
fn ExecuteButton(on_execute: EventHandler<String>) -> Element {
    rsx! {
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_execute.call("execute".to_string()),
            "🚀 Execute"
        }
    }
}

fn input_to_json_object(input: &str) -> Option<Value> {
    if !input.contains('{') {
        return Some(Value::Object(Default::default()));
    }

    let body = input.trim();
    let normalized = body.replace('\'', "\"");
    serde_json::from_str(&normalized).ok()
}

fn parse_query(query: &str) -> Option<(String, Value)> {
    if query.starts_with(AI_PREFIX) {
        let body = query.strip_prefix(AI_PREFIX)?;
        if body.starts_with(TOOL_FUNCTION) {
            let re = regex::Regex::new(r"invoke_tool\('([^']+)',\s*(\{.*\})\)").ok()?;
            let caps = re.captures(body)?;
            let tool_name = caps.get(1)?.as_str().to_string();
            let params = input_to_json_object(caps.get(2)?.as_str()).unwrap_or_default();
            return Some((tool_name, params));
        }
    }
    None
}

#[component]
fn QueryInputForm(query_input: Signal<String>, mcp_queries: Signal<Vec<McpQuery>>) -> Element {
    let mut error = use_signal(|| None::<String>);

    let handle_execute = move |_: String| {
        let query = query_input.read().clone();
        if let Some((tool_name, params)) = parse_query(&query) {
            let mut mcp_queries = mcp_queries;
            spawn(async move {
                let result = invoke_mcp_tool_with_fallback(tool_name.clone(), params.clone()).await;
                let query = McpQuery {
                    tool_name,
                    parameters: params,
                    result: Some(result),
                    timestamp: Utc::now().format("%H:%M:%S").to_string(),
                };
                mcp_queries.write().push(query);
            });
            query_input.set(String::new());
            error.set(None);
        } else {
            error.set(Some(
                "Invalid query format. Example: AI: invoke_tool('name', {'query': 'hello'})"
                    .to_string(),
            ));
        }
    };

    rsx! {
        div { class: "flex gap-2",
            ConfigurableQueryInput {
                query_input: query_input,
                config: PlaceholderConfig::default()
            }
            ExecuteButton { on_execute: handle_execute }
            if let Some(err) = error.read().as_ref() {
                div { class: "text-sm text-red-400 self-center", "{err}" }
            }
        }
    }
}

#[component]
fn AIQueryInterface(query_input: Signal<String>, mcp_queries: Signal<Vec<McpQuery>>) -> Element {
    rsx! {
        div { class: "mt-6 bg-gray-800 rounded-lg p-6",
            h2 { class: "text-xl font-bold mb-4", "🤖 AI Query Interface" }

            QueryInputForm { query_input: query_input, mcp_queries: mcp_queries }
            QueryHelpText {}
        }
    }
}

pub async fn handle_mcp_request(request: Value) -> Value {
    match request.get("method").and_then(|m| m.as_str()) {
        Some("tools/list") => {
            let tool_source: String = match load_gateway_tools().await {
                Ok(_) => "gateway".to_string(),
                Err(_) => "local".to_string(),
            };
            let mut schema = get_mcp_tools_schema("core");
            if let Ok(ref mut s) = schema {
                let meta = json!({ "source": tool_source });
                s.as_object_mut()
                    .and_then(|obj| obj.insert("source".to_string(), meta));
            }
            match schema {
                Ok(schema) => schema,
                Err(e) => serde_json::json!({
                    "error": {"code": -1, "message": format!("{:?}", e)}
                }),
            }
        }
        Some("tools/call") => {
            let tool_name = request["params"]["name"].as_str().unwrap_or("");
            let arguments = request["params"]["arguments"].clone();
            match invoke_mcp_tool_with_fallback(tool_name.to_string(), arguments).await {
                Ok(result) => serde_json::json!({
                    "content": [{"type": "text", "text": result.to_string()}]
                }),
                Err(e) => serde_json::json!({
                    "error": {"code": -1, "message": format!("{:?}", e)}
                }),
            }
        }
        _ => serde_json::json!({"error": "Unknown method"}),
    }
}

#[component]
pub fn MCPPlaygroundApp() -> Element {
    let mode = use_signal(|| PlaygroundMode::Hybrid);
    let active_tool = use_signal(|| None::<String>);
    let mcp_queries = use_signal(|| Vec::<McpQuery>::new());
    let query_input = use_signal(|| String::new());
    let mcp_tools = use_signal(|| Vec::<McpToolView>::new());
    let loading = use_signal(|| true);
    let load_error = use_signal(|| None::<String>);

    use_effect(move || {
        let mut mcp_tools = mcp_tools;
        let mut loading = loading;
        let mut load_error = load_error;
        spawn(async move {
            loading.set(true);
            let result = load_gateway_tools().await;
            match result {
                Ok(tools) => {
                    mcp_tools.set(tools);
                    load_error.set(None);
                }
                Err(err) => {
                    load_error.set(Some(err));
                    mcp_tools.set(local_tools());
                }
            }
            loading.set(false);
        });
    });

    let status_text = if let Some(err) = load_error.read().as_ref() {
        format!("Gateway unavailable: {err}")
    } else {
        "Tools loaded".to_string()
    };

    rsx! {
        div { class: "playground-container min-h-screen bg-gray-900 text-white p-6",
            h1 { "HELLO" }
            ModeSelector { mode: mode }
            div { class: "text-sm text-gray-300 mb-4", "{status_text}" }
            if *loading.read() {
                div { class: "text-sm text-gray-400 mb-4", "Loading MCP tools..." }
            }

            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",
                if *mode.read() != PlaygroundMode::HumanUI {
                    McpToolsPanel {
                        mcp_tools: mcp_tools.read().clone(),
                        active_tool: active_tool,
                        mcp_queries: mcp_queries
                    }
                }
                if *mode.read() != PlaygroundMode::McpInterface {
                    HumanUIPanel { mcp_tools: mcp_tools.read().clone() }
                }
            }

            if *mode.read() != PlaygroundMode::HumanUI && !mcp_queries.read().is_empty() {
                QueryLog { mcp_queries: mcp_queries }
            }
            if *mode.read() != PlaygroundMode::HumanUI {
                AIQueryInterface { query_input: query_input, mcp_queries: mcp_queries }
            }
        }
    }
}

crate::register_plugin!(
    "mcp",
    "Model Context Protocol playground",
    crate::plugin::PluginCategory::Meta,
    "🤖",
    || rsx! { div { "plugin" } }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_to_json_object_returns_empty_object_without_braces() {
        assert_eq!(input_to_json_object("embedding_ops"), Some(json!({})));
    }

    #[test]
    fn parse_query_accepts_single_quoted_ai_format() {
        let parsed = parse_query("AI: invoke_tool('embedding_ops', {'query': 'hello world'})");
        let Some((tool_name, params)) = parsed else {
            panic!("expected query to parse");
        };

        assert_eq!(tool_name, "embedding_ops");
        assert_eq!(params, json!({ "query": "hello world" }));
    }

    #[test]
    fn parse_query_rejects_invalid_format() {
        assert!(parse_query("invoke_tool('embedding_ops', {'query': 'hello'})").is_none());
        assert!(parse_query("AI: not_a_tool()").is_none());
    }
}
