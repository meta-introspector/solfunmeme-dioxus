use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

use axum::{
    extract::{Json, Path as AxumPath, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;
use std::process::Stdio;
use tokio::time::timeout;

pub const MCP_GATEWAY_ADDR_ENV: &str = "DIOXUS_MCP_GATEWAY_ADDR";
pub const ITIR_MCP_ROOT_ENV: &str = "ITIR_MCP_ROOT";
pub const PYTHON_EXECUTABLE_ENV: &str = "ITIR_PYTHON_EXECUTABLE";
pub const MCP_BRIDGE_TIMEOUT_MS: u64 = 10_000;

const DEFAULT_GATEWAY_ADDR: &str = "127.0.0.1:3939";

static BRIDGE_SESSION: OnceLock<Arc<Mutex<BridgeSession>>> = OnceLock::new();

#[derive(Debug, Clone)]
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

#[derive(Debug, Serialize, Deserialize)]
struct BridgeSessionConfig {
    #[serde(default)]
    op: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    payload: Value,
}

#[derive(Debug)]
struct BridgeSession {
    child: Option<Child>,
    stdin: Option<BufWriter<ChildStdin>>,
    stdout: Option<BufReader<ChildStdout>>,
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
        .route("/version", get(version_handler))
        .with_state(config.clone());

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
            Json(json!({"error": {"code": "input_error", "message": "tool name is required"}})),
        ));
    }

    match call_itir_mcp_tool(&payload.name, arguments).await {
        Ok(result) => Ok(Json(json!({ "result": result }))),
        Err(error) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": {
                    "code": "tool_error",
                    "message": error,
                },
            })),
        )),
    }
}

#[derive(Debug, Serialize)]
struct HealthStatus {
    ok: bool,
    service: String,
    version: String,
    protocol: String,
    tools: usize,
    status: String,
}

#[derive(Debug, Serialize)]
struct VersionStatus {
    service: String,
    version: String,
    protocol: String,
}

async fn health_handler() -> Json<Value> {
    let payload = run_py_bridge("health", None, None).await.unwrap_or_else(|error| {
        json!({
            "ok": false,
            "service": "itir-mcp-gateway",
            "status": "degraded",
            "error": error,
            "tools": 0,
            "version": "unavailable",
            "protocol": "unavailable"
        })
    });

    let status = payload
        .get("ok")
        .and_then(Value::as_bool)
        .unwrap_or_default();
    let service = payload
        .get("service")
        .and_then(Value::as_str)
        .unwrap_or("itir-mcp");

    if status {
        let version = payload.get("version").and_then(Value::as_str).unwrap_or("unknown");
        let protocol = payload.get("protocol").and_then(Value::as_str).unwrap_or("unknown");
        let tools = payload
            .get("tools")
            .and_then(Value::as_u64)
            .and_then(|value| usize::try_from(value).ok())
            .unwrap_or(0);
        Json(serde_json::to_value(HealthStatus {
            ok: true,
            service: service.to_string(),
            status: "ok".to_string(),
            version: version.to_string(),
            protocol: protocol.to_string(),
            tools,
        })
        .unwrap_or_else(|_| json!({"ok": false, "service": "itir-mcp-gateway", "status": "degraded"})))
    } else {
        Json(payload)
    }
}

async fn version_handler() -> Json<Value> {
    let payload = run_py_bridge("info", None, None).await.unwrap_or_else(|error| {
        json!({
            "error": error,
            "service": "itir-mcp-gateway",
            "version": "unavailable",
            "protocol": "unavailable"
        })
    });

    if let Some(error) = payload.get("error") {
        return Json(json!({"error": error, "service": "itir-mcp-gateway"}));
    }

    if let (Some(version), Some(protocol)) = (
        payload.get("version").and_then(Value::as_str),
        payload.get("protocol").and_then(Value::as_str),
    ) {
        Json(serde_json::to_value(VersionStatus {
            service: payload
                .get("service")
                .and_then(Value::as_str)
                .unwrap_or("itir-mcp")
                .to_string(),
            version: version.to_string(),
            protocol: protocol.to_string(),
        })
        .unwrap_or_else(|_| json!({"service": "itir-mcp", "version": version, "protocol": protocol})))
    } else {
        Json(payload)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ItirToolModel {
    name: String,
    title: String,
    description: String,
    input_schema: Value,
}

async fn request_itir_mcp_tools() -> Result<Vec<ItirToolModel>, (StatusCode, String)> {
    let response = run_py_bridge("list", None, None).await.map_err(|error| {
        (StatusCode::BAD_GATEWAY, format!("bridge tool list failed: {error}"))
    })?;
    let tools = response.get("tools").ok_or_else(|| {
        (StatusCode::INTERNAL_SERVER_ERROR, "failed to decode tool list: missing tools field".to_string())
    })?;
    let raw = serde_json::to_string(tools).map_err(|error| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to copy tool list: {error}"),
        )
    })?;
    serde_json::from_str::<Vec<ItirToolModel>>(&raw).map_err(|error| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("failed to decode tool list: {error}"),
        )
    })
}

async fn call_itir_mcp_tool(name: &str, arguments: Value) -> Result<Value, String> {
    let response = run_py_bridge("call", Some(name), Some(&arguments)).await?;
    if let Some(result) = response.get("result").and_then(Value::as_object) {
        Ok(Value::Object(result.clone()))
    } else if let Some(result) = response.get("result") {
        Ok(result.clone())
    } else {
        Err("bridge response missing result field".to_string())
    }
}

fn bridge_state() -> &'static Arc<Mutex<BridgeSession>> {
    BRIDGE_SESSION.get_or_init(|| Arc::new(Mutex::new(BridgeSession::new())))
}

impl BridgeSession {
    fn new() -> Self {
        Self {
            child: None,
            stdin: None,
            stdout: None,
        }
    }

    fn ensure_started(&mut self) -> Result<(), String> {
        if let Some(child) = self.child.as_mut() {
            if let Ok(Some(_)) = child.try_wait() {
                self.child = None;
                self.stdin = None;
                self.stdout = None;
            }
        }
        if self.child.is_some() {
            return Ok(());
        }
        let root = locate_itir_mcp_root().ok_or_else(|| {
            "Could not locate itir-mcp checkout. Set ITIR_MCP_ROOT to the itir-mcp directory."
                .to_string()
        })?;

        let mut cmd = Command::new(locate_python_exec());
        cmd.current_dir(&root);
        cmd.env("PYTHONPATH", root.join("src"));
        cmd.args(["-m", "itir_mcp", "--bridge"]);
        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::null());

        let mut child = cmd.spawn().map_err(|error| format!("python bridge spawn failed: {error}"))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| "python bridge started without stdin".to_string())?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "python bridge started without stdout".to_string())?;

        self.child = Some(child);
        self.stdin = Some(BufWriter::new(stdin));
        self.stdout = Some(BufReader::new(stdout));
        Ok(())
    }

    async fn call(&mut self, request: &Value) -> Result<Value, String> {
        self.ensure_started()?;

        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| "python bridge session stdin unavailable".to_string())?;
        let stdout = self
            .stdout
            .as_mut()
            .ok_or_else(|| "python bridge session stdout unavailable".to_string())?;

        timeout(Duration::from_millis(MCP_BRIDGE_TIMEOUT_MS), async {
            let request = serde_json::to_string(request)
                .map_err(|error| format!("failed to serialize bridge request: {error}"))?;
            stdin.write_all(request.as_bytes()).await.map_err(|error| {
                format!("failed to write bridge request: {error}")
            })?;
            stdin.write_all(b"\n").await.map_err(|error| {
                format!("failed to write bridge delimiter: {error}")
            })?;
            stdin.flush().await.map_err(|error| format!("failed to flush bridge request: {error}"))?;

            let mut output = String::new();
            let read = stdout.read_line(&mut output).await.map_err(|error| {
                format!("failed to read bridge response: {error}")
            })?;
            if read == 0 {
                return Err("bridge closed before response".to_string());
            }

            let output = output.trim();
            serde_json::from_str::<Value>(output)
                .map_err(|error| format!("bridge response parse error: {error}; output={output}"))
        })
        .await
        .map_err(|_| "bridge request timed out".to_string())?
    }
}

async fn run_py_bridge(
    op: &str,
    name: Option<&str>,
    payload: Option<&Value>,
) -> Result<Value, String> {
    let payload = payload.cloned().unwrap_or_else(|| json!({}));
    let request = serde_json::to_value(BridgeSessionConfig {
        op: op.to_string(),
        name: name.map(str::to_string),
        payload,
    })
    .map_err(|error| format!("failed to build bridge request: {error}"))?;

    let mut session = bridge_state().lock().await;
    match session.call(&request).await {
        Ok(response) => handle_bridge_response(response),
        Err(error) => {
            session.child = None;
            session.stdin = None;
            session.stdout = None;
            session.call(&request).await.and_then(handle_bridge_response).or(Err(error))
        }
    }
}

fn handle_bridge_response(response: Value) -> Result<Value, String> {
    let ok = response.get("ok").and_then(Value::as_bool).unwrap_or(false);
    if !ok {
        let code = response
            .get("error")
            .and_then(|error| error.get("code"))
            .and_then(Value::as_str)
            .unwrap_or("tool_error");
        let message = response
            .get("error")
            .and_then(|error| error.get("message"))
            .and_then(Value::as_str)
            .unwrap_or("unknown bridge error");
        return Err(format!("{code}: {message}"));
    }

    Ok(response)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bind_addr_uses_env_when_present() {
        env::set_var(MCP_GATEWAY_ADDR_ENV, "127.0.0.1:9999");
        assert_eq!(gateway_bind_addr(), "127.0.0.1:9999");
        env::remove_var(MCP_GATEWAY_ADDR_ENV);
    }

    #[test]
    fn default_bind_addr_uses_fallback_when_env_missing() {
        env::remove_var(MCP_GATEWAY_ADDR_ENV);
        assert_eq!(gateway_bind_addr(), DEFAULT_GATEWAY_ADDR);
        assert_eq!(
            gateway_base_url(),
            format!("http://{DEFAULT_GATEWAY_ADDR}/api/itir-mcp")
        );
    }

    #[tokio::test]
    async fn call_tool_handler_rejects_empty_tool_name() {
        let result = call_tool_handler(
            State(GatewayConfig::default()),
            Json(ToolCallRequest {
                name: "   ".to_string(),
                arguments: None,
            }),
        )
        .await;

        let Err((status, body)) = result else {
            panic!("expected empty tool name to be rejected");
        };

        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body.0["error"]["message"], "tool name is required");
    }
}
