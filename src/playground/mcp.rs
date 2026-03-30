// playground.rs - MCP Tool Orchestration Surface
use crate::model::lean::style::Styles;
use dioxus::prelude::*;
use rrust_kontekst_base::{get_mcp_tools, get_mcp_tools_schema, invoke_mcp_tool, McpToolInfo};
use serde_json::Value;
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
    HumanUI,      // Traditional UI mode
    McpInterface, // AI/MCP query interface
    Hybrid,       // Both modes visible
}

#[derive(Clone, Debug, PartialEq)]
pub struct McpQuery {
    pub tool_name: String,
    pub parameters: Value,
    pub result: Option<Result<Value, String>>,
    pub timestamp: String,
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
    mcp_tools: Vec<McpToolInfo>,
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
            let some_str =  "Test";
                    let schema = get_mcp_tools_schema(some_str);
                    println!("MCP Schema: {}", serde_json::to_string_pretty(&schema).unwrap());
                },
                "📋 Export MCP Schema"
            }
        }
    }
}

#[component]
fn ToolCardHeader(tool: McpToolInfo) -> Element {
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
fn ToolCardDetails(tool: McpToolInfo, mcp_queries: Signal<Vec<McpQuery>>) -> Element {
    rsx! {
        div { class: "mt-3 pt-3 border-t border-gray-600",
            div { class: "text-sm space-y-2",
                div {
                    strong { "Parameters:" }
                    ul { class: "list-disc list-inside ml-2 text-gray-300",
                        {tool.parameters.iter().map(|param| rsx! {
                            li { key: "{param}", "{param}" }
                        })}
                    }
                }
                div {
                    strong { "Returns:" }
                    span { class: "text-gray-300 ml-2", "{tool.returns}" }
                }
            }

            QuickInvokeButton { tool: tool.clone(), mcp_queries: mcp_queries }
        }
    }
}

#[component]
fn QuickInvokeButton(tool: McpToolInfo, mcp_queries: Signal<Vec<McpQuery>>) -> Element {
    rsx! {
        button {
            class: "{Styles::primary_button()} text-sm mt-2",
            onclick: move |_| {
                let tool_name = tool.tool_name.to_string();
                spawn(async move {
                    let result = invoke_mcp_tool(&tool_name, Value::Object(Default::default())).await;
                    let query = McpQuery {
                        tool_name: tool_name.clone(),
                        parameters: Value::Object(Default::default()),
                        result: Some(result.map_err(|e| format!("{:?}", e))),
                        timestamp: chrono::Utc::now().format("%H:%M:%S").to_string(),
                    };
                    mcp_queries.write().push(query);
                });
            },
            "⚡ Quick Invoke"
        }
    }
}

#[component]
fn ToolCard(
    tool: McpToolInfo,
    active_tool: Signal<Option<String>>,
    mcp_queries: Signal<Vec<McpQuery>>,
) -> Element {
    let is_active = active_tool.read().as_ref() == Some(&tool.tool_name.to_string());

    rsx! {
        div {
            class: format!("p-3 rounded border cursor-pointer transition-colors {}",
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
fn HumanUIPanel(mcp_tools: Vec<McpToolInfo>) -> Element {
    rsx! {
        div { class: "bg-gray-800 rounded-lg p-6",
            h2 { class: "text-xl font-bold mb-4", "🎯 Human Interface" }

            div { class: "grid gap-3",
                {mcp_tools.iter().filter(|tool| tool.visible).map(|tool| {
                    rsx! {
                        HumanUIButton {
                            key: "{tool.component_name}",
                            tool: tool.clone()
                        }
                    }
                })}
            }
        }
    }
}

#[component]
fn HumanUIButton(tool: McpToolInfo) -> Element {
    rsx! {
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| {
                println!("Activating UI component: {}", tool.component_name);
            },
            "{tool.emoji} {tool.label}"
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
fn QueryResult(result: Result<Value, String>) -> Element {
    match result {
        Ok(value) => {
            rsx! {
                div { class: "text-sm text-green-400",
                    "✅ Success: "
                    code {
                        class: "bg-gray-800 px-2 py-1 rounded",
                        "{serde_json::to_string_pretty(&value).unwrap_or_default()}"
                    }
                }
            }
        }
        Err(error) => {
            rsx! {
                div { class: "text-sm text-red-400",
                    "❌ Error: {error}"
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

#[component]
fn StyledInput(placeholder: String, value: String, oninput: EventHandler<FormEvent>) -> Element {
    rsx! {
        input {
            class: INPUT_CLASSES,
            placeholder: "{placeholder}",
            value: "{value}",
            oninput: move |evt| oninput.call(evt)
        }
    }
}

fn input_value(query_input: Signal<String>) -> String {
    query_input.read().clone()
}

fn input_handler(mut query_input: Signal<String>) -> EventHandler<FormEvent> {
    EventHandler::new(move |evt: FormEvent| query_input.set(evt.value()))
}
#[component]
fn QueryInput(
    query_input: Signal<String>,
    config: Option<PlaceholderConfig>,
    available_tools: Option<Vec<String>>,
    on_execute: EventHandler<String>,
) -> Element {
    let config = config.unwrap_or_default();
    let placeholder = if let Some(tools) = available_tools {
        format_placeholder(
            tools.first().unwrap_or(&config.tool_name),
            config
                .example_params
                .get("query")
                .unwrap_or(&"hello world".to_string()),
        )
    } else {
        format_placeholder(
            &config.tool_name,
            config
                .example_params
                .get("query")
                .unwrap_or(&"hello world".to_string()),
        )
    };

    rsx! {
        div { class: "flex gap-2",
            input {
                class: INPUT_CLASSES,
                placeholder: "{placeholder}",
                value: "{query_input.read()}",
                oninput: move |evt| query_input.set(evt.value())
            }
            ExecuteButton { on_execute }
        }
    }
}

// #[component]
// fn QueryInputOld(
//     query_input: Signal<String>,
//     config: Option<PlaceholderConfig>,
//     available_tools: Option<Vec<String>>,
//     on_execute: EventHandler<String>,
// ) -> Element {
//     let config = config.unwrap_or_default();
//     let placeholder = if let Some(tools) = available_tools {
//         format_placeholder(tools.first().unwrap_or(&config.tool_name), &config.example_params)
//     } else {
//         format_placeholder(&config.tool_name, &config.example_params)
//     };

//     rsx! {
//         div { class: "flex gap-2",
//             input {
//                 class: INPUT_CLASSES,
//                 placeholder: "{placeholder}",
//                 value: "{query_input.read()}",
//                 oninput: move |evt| query_input.set(evt.value())
//             }
//             ExecuteButton { on_execute }
//         }
//     }
// }

// #[component]
// fn QueryInput(query_input: Signal<String>) -> Element {
//     rsx! {
//         StyledInput {
//             placeholder: AI_PLACEHOLDER,
//             value: input_value ( query_input ),
//             oninput: input_handler ( query_input )
//         }
//     }
// }

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
                QueryResult { result: result.clone() }
            }
        }
    }
}

// #[component]
// fn AIQueryInterface(query_input: Signal<String>) -> Element {
//     rsx! {
//         div { class: "mt-6 bg-gray-800 rounded-lg p-6",
//             h2 { class: "text-xl font-bold mb-4", "🤖 AI Query Interface" }

//             QueryInputForm { query_input: query_input }
//             QueryHelpText {}
//         }
//     }
// }

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

#[component]
fn PlaceholderBuilder(tool_name: Option<String>, example_query: Option<String>) -> Element {
    let tool = tool_name.unwrap_or_else(|| EXAMPLE_TOOL.to_string());
    let query = example_query.unwrap_or_else(|| EXAMPLE_QUERY.to_string());

    let placeholder = format!(
        "{}{}('{}', {{'query': '{}'}})",
        AI_PREFIX, TOOL_FUNCTION, tool, query
    );

    rsx! {
        input {
            class: "flex-1 px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white",
            placeholder: "{placeholder}",
            // ... other props
        }
    }
}
// #[component]
// fn QueryInputWithDynamicPlaceholder(
//     query_input: Signal<String>,
//     available_tools: Vec<String>
// ) -> Element {
//     let selected_tool = available_tools.first().unwrap_or(&EXAMPLE_TOOL.to_string()).clone();

//     rsx! {
//         input {
//             class: "flex-1 px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white",
//             placeholder: format_placeholder(&selected_tool, EXAMPLE_QUERY),
//             value: "{query_input.read()}",
//             oninput: move |evt| query_input.set(evt.value())
//         }
//     }
// }

fn format_placeholder(tool_name: &str, example_query: &str) -> String {
    format!(
        "{}{}('{}', {{'query': '{}'}})",
        AI_PREFIX, TOOL_FUNCTION, tool_name, example_query
    )
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
            prefix: "AI: ".to_string(),
            function_name: "invoke_tool".to_string(),
            tool_name: "embedding_ops".to_string(),
            example_params: params,
        }
    }
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
            class: "flex-1 px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white",
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

// #[component]
// fn QueryInputForm(query_input: Signal<String>) -> Element {
//     let config = PlaceholderConfig::default(); // or create custom config

//     let handle_execute = move |_: String| {
//         let query = query_input.read().clone();
//         if !query.is_empty() {
//             spawn(async move {
//                 println!("Processing AI query: {}", query);
//             });
//             query_input.set(String::new());
//         }
//     };

//     rsx! {
//         div { class: "flex gap-2",
//             ConfigurableQueryInput {
//                 query_input: query_input,
//                 config: config
//             }
//             ExecuteButton { on_execute: handle_execute }
//         }
//     }
// }

// #[component]
// fn QueryInputForm2(query_input: Signal<String>) -> Element {
//     let handle_execute = move |_: String| {
//         let query = query_input.read().clone();
//         if !query.is_empty() {
//             spawn(async move {
//                 println!("Processing AI query: {}", query);
//             });
//             query_input.set(String::new());
//         }
//     };

//     rsx! {
//         div { class: "flex gap-2",
//             QueryInput { query_input: query_input }
//             ExecuteButton { on_execute: handle_execute }
//         }
//     }
// }

// #[component]
// fn QueryInputForm3(query_input: Signal<String>) -> Element {
//     let mut params = HashMap::new();
//     params.insert("query".to_string(), "hello world".to_string());

//     let config = PlaceholderConfig {
//         prefix: "AI: ".to_string(),
//         function_name: "invoke_tool".to_string(),
//         tool_name: "embedding_ops".to_string(),
//         example_params: params,
//     };

//     let handle_execute = move |_: String| {
//         let query = query_input.read().clone();
//         if !query.is_empty() {
//             spawn(async move {
//                 println!("Processing AI query: {}", query);
//             });
//             query_input.set(String::new());
//         }
//     };

//     rsx! {
//         div { class: "flex gap-2",
//             ConfigurableQueryInput {
//                 query_input: query_input,
//                 config: config
//             }
//             ExecuteButton { on_execute: handle_execute }
//         }
//     }
// }

// MCP Server Integration (for external AI agents)
pub async fn handle_mcp_request(request: Value) -> Value {
    match request.get("method").and_then(|m| m.as_str()) {
        Some("tools/list") => {
            let some_str = "Test";
            let res = get_mcp_tools_schema(some_str);
            match res {
                Ok(res) => res,
                Err(e) => serde_json::json!({
                    "error": {"code": -1, "message": format!("{:?}", e)}
                }),
            }
        }
        Some("tools/call") => {
            let tool_name = request["params"]["name"].as_str().unwrap_or("");
            let arguments = request["params"]["arguments"].clone();

            match invoke_mcp_tool(tool_name, arguments).await {
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
fn ErrorToast(error: Signal<Option<String>>) -> Element {
    if let Some(err) = error.read().as_ref() {
        rsx! {
            div { class: "fixed bottom-4 right-4 bg-red-600 text-white p-4 rounded",
                "Error: {err}"
                button {
                    class: "ml-4 text-sm",
                    onclick: move |_| error.set(None),
                    "Dismiss"
                }
            }
        }
    } else {
        rsx! { div {} }
    }
}

fn parse_query(query: &str) -> Option<(String, Value)> {
    if query.starts_with(AI_PREFIX) {
        let body = query.strip_prefix(AI_PREFIX)?;
        if body.starts_with(TOOL_FUNCTION) {
            // Simplified regex or manual parsing for tool name and params
            let re = regex::Regex::new(r"invoke_tool\('([^']+)',\s*(\{.*\})\)").ok()?;
            let caps = re.captures(body)?;
            let tool_name = caps.get(1)?.as_str().to_string();
            let params: Value = serde_json::from_str(caps.get(2)?.as_str()).ok()?;
            return Some((tool_name, params));
        }
    }
    None
}

#[component]
fn SchemaExportButtonNew() -> Element {
    let mut show_schema = use_signal(|| false);
    let some_str = "Test";
    let schema = serde_json::to_string_pretty(&get_mcp_tools_schema(some_str)).unwrap_or_default();

    rsx! {
        div { class: "mb-4",
            button {
                class: "{Styles::primary_button()} text-sm",
                onclick: move |_| show_schema.set(true),
                "📋 View MCP Schema"
            }
            if *show_schema.read() {
                div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center",
                    div { class: "bg-gray-800 p-6 rounded-lg max-w-2xl max-h-[80vh] overflow-y-auto",
                        pre { class: "text-sm", "{schema}" }
                        button {
                            class: "{Styles::primary_button()} mt-4",
                            onclick: move |_| show_schema.set(false),
                            "Close"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QueryInputForm(query_input: Signal<String>, mcp_queries: Signal<Vec<McpQuery>>) -> Element {
    let mut error = use_signal(|| None::<String>);

    let handle_execute = move |_: String| {
        let query = query_input.read().clone();
        if let Some((tool_name, params)) = parse_query(&query) {
            spawn(async move {
                let result = invoke_mcp_tool(&tool_name, params.clone()).await;
                let query = McpQuery {
                    tool_name,
                    parameters: params,
                    result: Some(result.map_err(|e| format!("{:?}", e))),
                    timestamp: chrono::Utc::now().format("%H:%M:%S").to_string(),
                };
                mcp_queries.write().push(query);
            });
            query_input.set(String::new());
        } else {
            error.set(Some("Invalid query format".to_string()));
        }
    };

    rsx! {
        div { class: "flex gap-2",
            ConfigurableQueryInput {
                query_input: query_input,
                config: PlaceholderConfig::default()
            }
            ExecuteButton { on_execute: handle_execute }
            ErrorToast { error }
        }
    }
}

///

// #[component]
// pub fn PlaygroundApp2() -> Element {
//     let mut mode = use_signal(|| PlaygroundMode::Hybrid);
//     let mut active_tool = use_signal(|| None::<String>);
//     let mut mcp_queries = use_signal(|| Vec::<McpQuery>::new());
//     let mut query_input = use_signal(|| String::new());

//     let mcp_tools = get_mcp_tools("core");

//     rsx! {
//         div { class: "playground-container min-h-screen bg-gray-900 text-white p-6",

//             ModeSelector { mode: mode }

//             div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",

//                 if *mode.read() != PlaygroundMode::HumanUI {
//                     McpToolsPanel {
//                         mcp_tools: mcp_tools.clone(),
//                         active_tool: active_tool,
//                         mcp_queries: mcp_queries
//                     }
//                 }

//                 if *mode.read() != PlaygroundMode::McpInterface {
//                     HumanUIPanel { mcp_tools: mcp_tools.clone() }
//                 }
//             }

//             if *mode.read() != PlaygroundMode::HumanUI && !mcp_queries.read().is_empty() {
//                 QueryLog { mcp_queries: mcp_queries }
//             }

//             if *mode.read() != PlaygroundMode::HumanUI {
//                 AIQueryInterface { query_input: query_input }
//             }
//         }
//     }
// }

#[component]
pub fn MCPPlaygroundApp() -> Element {
    let mode = use_signal(|| PlaygroundMode::Hybrid);
    let active_tool = use_signal(|| None::<String>);
    let mcp_queries = use_signal(|| Vec::<McpQuery>::new());
    let query_input = use_signal(|| String::new());

    let mcp_tools = get_mcp_tools("core")?;

    rsx! {
        div { class: "playground-container min-h-screen bg-gray-900 text-white p-6",

           h1 {
           "HELLO",
           },
            ModeSelector { mode: mode }

            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6",

                if *mode.read() != PlaygroundMode::HumanUI {
                    McpToolsPanel {
                        mcp_tools: mcp_tools.clone(),
                        active_tool: active_tool,
                        mcp_queries: mcp_queries
                    }
                }

                if *mode.read() != PlaygroundMode::McpInterface {
                    HumanUIPanel { mcp_tools: mcp_tools.clone() }
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
