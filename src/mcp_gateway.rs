use std::env;
use std::path::{Path, PathBuf};

use axum::{
    extract::{Json, Path as AxumPath, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::process::Command;

pub const MCP_GATEWAY_ADDR_ENV: &str = "DIOXUS_MCP_GATEWAY_ADDR";
pub const ITIR_MCP_ROOT_ENV: &str = "ITIR_MCP_ROOT";
pub const PYTHON_EXECUTABLE_ENV: &str = "ITIR_PYTHON_EXECUTABLE";

const DEFAULT_GATEWAY_ADDR: &str = "127.0.0.1:3939";

#[derive(Debug, Clone, Copy)]
pub struct GatewayConfig {
    pub bind_addr: String,
    pub api_prefix: &'static str,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            bind_addr: env::var(MCP_GATEWAY_ADDR_ENV)
                .unwrap_or_else(|_| DEFAULT_GATEWAY_ADDR.to_string()),
            api_prefix: "/api/itir-mcp",
        }
    }
}

#[derive(Debug, Serialize)]
struct ItirToolSpec {
    name: String,
    title: String,
    description: String,
    input_schema: Value,
}

#[derive(Debug, Serialize)]
struct ItirToolsResponse {
    tools: Vec<ItirToolSpec>,
}

#[derive(Debug, Deserialize)]
struct ToolCallRequest {
    name: String,
    arguments: Option<Value>,
}

impl GatewayConfig {
    fn api_base(&self) -> String {
        format!("http://{}{}", self.bind_addr, self.api_prefix)
    }
}

pub fn gateway_bind_addr() -> String {
    GatewayConfig::default().bind_addr
}

pub fn gateway_base_url() -> String {
    GatewayConfig::default().api_base()
}

pub async fn run_mcp_gateway(config: GatewayConfig) -> Result<(), String> {
    let app = Router::new()
        .route("/tools", get(list_tools_handler))
        .route("/tools/:name", get(get_tool_schema_handler))
        .route("/call", post(call_tool_handler))
        .route("/health", get(health_handler))
        .with_state(config);

    let listener = tokio::net::TcpListener::bind(config.bind_addr.as_str())
        .await
        .map_err(|error| format!("failed to bind MCP gateway: {error}"))?;

    axum::serve(listener, app)
        .await
        .map_err(|error| format!("MCP gateway failed: {error}"))
}

pub async fn run_default_mcp_gateway() -> Result<(), String> {
    run_mcp_gateway(GatewayConfig::default()).await
}

async fn list_tools_handler(
    State(_config): State<GatewayConfig>,
) -> Result<Json<ItirToolsResponse>, (StatusCode, String)> {
    let tools = request_itir_mcp_tools()
        .await?
        .into_iter()
        .map(|tool| ItirToolSpec {
            name: tool.name,
            title: tool.title,
            description: tool.description,
            input_schema: tool.input_schema,
        })
        .collect::<Vec<_>>();

    Ok(Json(ItirToolsResponse { tools }))
}

async fn get_tool_schema_handler(
    AxumPath(name): AxumPath<String>,
    State(_config): State<GatewayConfig>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let tools = request_itir_mcp_tools().await?;

    for tool in tools {
        if tool.name == name {
            return Ok(Json(tool.input_schema));
        }
    }

    Err((StatusCode::NOT_FOUND, format!("Unknown tool: {name}")))
}

async fn call_tool_handler(
    State(_config): State<GatewayConfig>,
    Json(payload): Json<ToolCallRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let arguments = payload.arguments.unwrap_or_else(|| json!({}));

    if payload.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": {"message": "tool name is required"}})),
        ));
    }

    match call_itir_mcp_tool(&payload.name, arguments).await {
        Ok(result) => Ok(Json(json!({ "result": result }))),
        Err(error) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": {
                    "message": error,
                },
            })),
        )),
    }
}

async fn health_handler() -> Json<Value> {
    Json(json!({ "ok": true, "service": "itir-mcp-gateway" }))
}

#[derive(Debug, Serialize, Deserialize)]
struct ItirToolModel {
    name: String,
    title: String,
    description: String,
    input_schema: Value,
}

async fn request_itir_mcp_tools() -> Result<Vec<ItirToolModel>, (StatusCode, String)> {
    match run_py_bridge("list", None, None).await {
        Ok(json_value) => serde_json::from_value(json_value).map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to decode tool list: {error}"),
            )
        }),
        Err(error) => Err((StatusCode::BAD_GATEWAY, error)),
    }
}

async fn call_itir_mcp_tool(name: &str, arguments: Value) -> Result<Value, String> {
    run_py_bridge("call", Some(name), Some(&arguments)).await
}

fn locate_itir_mcp_root() -> Option<PathBuf> {
    if let Ok(raw) = env::var(ITIR_MCP_ROOT_ENV) {
        let candidate = PathBuf::from(raw);
        if candidate.is_dir() {
            return Some(candidate);
        }
    }

    let current_dir = env::current_dir().ok()?;
    let candidate_paths = [
        current_dir.join("ITIR-suite/itir-mcp"),
        current_dir
            .parent()
            .map(|parent| parent.join("ITIR-suite/itir-mcp"))
            .unwrap_or_else(|| current_dir.clone()),
        Path::new("/home/c/Documents/code/ITIR-suite/itir-mcp").to_path_buf(),
    ];

    for candidate in candidate_paths {
        if candidate.is_dir() {
            return Some(candidate);
        }
    }

    None
}

fn locate_python_exec() -> String {
    env::var(PYTHON_EXECUTABLE_ENV)
        .or_else(|_| env::var("PYTHON_EXECUTABLE"))
        .unwrap_or_else(|_| "python3".to_string())
}

async fn run_py_bridge(
    op: &str,
    name: Option<&str>,
    payload: Option<&Value>,
) -> Result<Value, String> {
    let root = locate_itir_mcp_root().ok_or_else(|| {
        "Could not locate itir-mcp checkout. Set ITIR_MCP_ROOT to the itir-mcp directory."
            .to_string()
    })?;

    let mut cmd = Command::new(locate_python_exec());
    cmd.current_dir(&root);
    cmd.env("PYTHONPATH", root.join("src"));
    cmd.arg("-c");
    cmd.arg(
        [
            "import json",
            "import sys",
            "from itir_mcp import build_default_registry",
            "",
            "registry = build_default_registry()",
            "tools = registry.list_tools()",
            "",
            "if sys.argv[1] == 'list':",
            "    out = [",
            "        {",
            "            'name': tool.name,",
            "            'title': tool.title,",
            "            'description': tool.description,",
            "            'input_schema': tool.input_schema,",
            "        }",
            "        for tool in tools",
            "    ]",
            "    print(json.dumps(out))",
            "    sys.exit(0)",
            "",
            "if sys.argv[1] != 'call' or len(sys.argv) < 4:",
            "    print(json.dumps({'error': 'invalid method'}))",
            "    sys.exit(1)",
            "",
            "payload = json.loads(sys.argv[3])",
            "name = sys.argv[2]",
            "out = registry.invoke(name, payload)",
            "print(json.dumps(out))",
        ]
        .join("\n")
        .as_str(),
    );
    cmd.arg(op);
    if let Some(tool_name) = name {
        cmd.arg(tool_name);
    }
    if let Some(body) = payload {
        cmd.arg(body.to_string());
    }

    let output = cmd
        .output()
        .await
        .map_err(|error| format!("python failed: {error}"))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!("python bridge error: {err}{stdout}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(stdout.trim())
        .map_err(|error| format!("python output parse error: {error}; output={stdout}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bind_addr_uses_env_when_present() {
        env::set_var(MCP_GATEWAY_ADDR_ENV, "127.0.0.1:9999");
        assert_eq!(gateway_bind_addr(), "127.0.0.1:9999");
        env::remove_var(MCP_GATEWAY_ADDR_ENV);
    }
}
