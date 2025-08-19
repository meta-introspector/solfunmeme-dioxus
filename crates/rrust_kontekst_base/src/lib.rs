// rrust_kontekst_base/src/lib.rs - MCP (Model Context Protocol) integration
// use rrust_kontekst_base::*
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::future::Future;
use std::pin::Pin;
use serde_json::Value;
use serde::Serialize;
use dioxus_logger::tracing::info;
// Type alias for MCP handlers
type McpHandler = fn(Value) -> Pin<Box<dyn Future<Output = Result<Value, McpError>> + Send>>;

// Thread-safe registry for MCP tools
static MCP_REGISTRY: OnceLock<RwLock<HashMap<String, (McpToolInfo, McpHandler)>>> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct McpToolInfo {
    pub component_name: &'static str,
    pub tool_name: &'static str,
    pub menu_type: &'static str,
    pub label: &'static str,
    pub emoji: &'static str,
    pub description: &'static str,
    pub visible: bool,
    pub order: i32,
    pub mcp_enabled: bool,
    pub parameters: &'static [&'static str],
    pub returns: &'static str,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum McpError {
    InvalidParams,
    ExecutionError(String),
    NotFound,
    RegistryLocked,
}

impl std::fmt::Display for McpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McpError::InvalidParams => write!(f, "Invalid parameters provided"),
            McpError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            McpError::NotFound => write!(f, "Tool not found"),
            McpError::RegistryLocked => write!(f, "Registry is locked"),
        }
    }
}

impl std::error::Error for McpError {}

/// Initialize the MCP registry
fn get_or_init_registry() -> &'static RwLock<HashMap<String, (McpToolInfo, McpHandler)>> {
    MCP_REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register an MCP tool with thread safety
pub fn register_mcp_tool(info: &'static McpToolInfo, handler: McpHandler) -> Result<(), McpError> {

    //info!("Register MCP tool: {} -> {}", info.tool_name, info.description);
    
    let registry = get_or_init_registry();
    
    match registry.write() {
        Ok(mut map) => {
            map.insert(info.tool_name.to_string(), (info.clone(), handler));
      //      info!("Registered MCP tool: {} -> {}", info.tool_name, info.description);

            Ok(())
        }
        Err(_) => Err(McpError::RegistryLocked)
    }
}

/// Get MCP tools by menu type
pub fn get_mcp_tools(menu_type: &str) -> Result<Vec<McpToolInfo>, McpError> {
    let registry = get_or_init_registry();
    
    match registry.read() {
        Ok(map) => {
            let tools: Vec<McpToolInfo> = map
                .values()
                .filter_map(|(info, _)| {
                    if info.menu_type == menu_type && info.visible && info.mcp_enabled {
                        Some(info.clone())
                    } else {
                        None
                    }
                })
                .collect();
            Ok(tools)
        }
        Err(_) => Err(McpError::RegistryLocked)
    }
}

/// Generate MCP tools schema for AI
pub fn get_mcp_tools_schema(menu_type: &str) -> Result<Value, McpError> {
    let tools = get_mcp_tools(menu_type)?;
    
    let tool_schemas: Vec<Value> = tools
        .into_iter()
        .map(|tool| {
            let properties: serde_json::Map<String, Value> = tool
                .parameters
                .iter()
                .map(|p| {
                    (
                        p.to_string(),
                        serde_json::json!({
                            "type": "string",
                            "description": p
                        })
                    )
                })
                .collect();
            
            serde_json::json!({
                "name": tool.tool_name,
                "description": format!("{} {}", tool.emoji, tool.description),
                "inputSchema": {
                    "type": "object",
                    "properties": properties,
                    "required": []
                }
            })
        })
        .collect();

    info!("schema: {:?}", tool_schemas);
    Ok(serde_json::json!({
        "tools": tool_schemas
    }))
}

/// Invoke an MCP tool by name
pub async fn invoke_mcp_tool(tool_name: &str, params: Value) -> Result<Value, McpError> {
    let registry = get_or_init_registry();
    
    let handler = {
        match registry.read() {
            Ok(map) => {
                match map.get(tool_name) {
                    Some((_, handler)) => *handler,
                    None => return Err(McpError::NotFound),
                }
            }
            Err(_) => return Err(McpError::RegistryLocked),
        }
    };
    
    // Call the handler
    handler(params).await
}

/// Get all registered tools (for debugging/admin purposes)
pub fn list_all_tools() -> Result<Vec<String>, McpError> {
    let registry = get_or_init_registry();
    
    match registry.read() {
        Ok(map) => Ok(map.keys().cloned().collect()),
        Err(_) => Err(McpError::RegistryLocked)
    }
}

/// Configuration for the MCP component (used by the macro)
#[derive(Debug, Clone, Serialize)]
pub struct McpConfig {
    pub menu_type: String,
    pub label: String,
    pub emoji: String,
    pub description: String,
    pub visible: bool,
    pub order: i32,
    pub mcp_enabled: bool,
    pub tool_name: String,
    pub parameters: Vec<String>,
    pub returns: String,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            menu_type: "core".to_string(),
            label: String::new(),
            emoji: "🔧".to_string(),
            description: String::new(),
            visible: true,
            order: 0,
            mcp_enabled: true,
            tool_name: String::new(),
            parameters: Vec::new(),
            returns: "Operation completed".to_string(),
        }
    }
}

impl McpConfig {
    /// Create a new config with default values based on function name
    pub fn new(fn_name: &str) -> Self {
        Self {
            label: fn_name.to_string(),
            tool_name: fn_name.to_lowercase().replace("component", ""),
            ..Default::default()
        }
    }
    
    // FIXME Critical: Box::leak causes permanent memory leaks.
    /// Convert to McpToolInfo for registration
    pub fn to_tool_info(&self, component_name: &'static str) -> McpToolInfo {
        // Convert Vec<String> to &'static [&'static str] - this is a limitation
        // In practice, you might want to use a different approach or Box<[&str]>
        let static_params: &'static [&'static str] = &[]; // Simplified for now
        
        McpToolInfo {
            component_name,
            tool_name: Box::leak(self.tool_name.clone().into_boxed_str()),
            menu_type: Box::leak(self.menu_type.clone().into_boxed_str()),
            label: Box::leak(self.label.clone().into_boxed_str()),
            emoji: Box::leak(self.emoji.clone().into_boxed_str()),
            description: Box::leak(self.description.clone().into_boxed_str()),
            visible: self.visible,
            order: self.order,
            mcp_enabled: self.mcp_enabled,
            parameters: static_params,
            returns: Box::leak(self.returns.clone().into_boxed_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_registry_operations() {
        let info = McpToolInfo {
            component_name: "test",
            tool_name: "test_tool",
            menu_type: "core",
            label: "Test Tool",
            emoji: "🧪",
            description: "A test tool",
            visible: true,
            order: 0,
            mcp_enabled: true,
            parameters: &["param1"],
            returns: "test result",
        };
        
        let handler: McpHandler = |_params| {
            Box::pin(async {
                Ok(serde_json::json!({"result": "test"}))
            })
        };
        
        // Test registration
        assert!(register_mcp_tool(&info, handler).is_ok());
        
        // Test retrieval
        let tools = get_mcp_tools("core").unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].tool_name, "test_tool");
        
        // Test invocation
        let result = invoke_mcp_tool("test_tool", serde_json::json!({})).await;
        assert!(result.is_ok());
    }
}

