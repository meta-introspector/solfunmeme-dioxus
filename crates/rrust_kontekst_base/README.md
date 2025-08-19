# `rrust_kontekst_base`

This crate defines the foundational types and traits for the Micro-Component Protocol (MCP) used in the Solfunmeme project.

## Purpose

It provides the essential building blocks for creating and managing introspectable components, serving as the base layer for the `rrust_kontekst` procedural macro and other MCP-related functionalities.

## Core Functionalities

-   **`McpConfig`**: Configuration structure for MCP components.
-   **`McpToolInfo`**: Metadata structure describing an MCP tool or component.
-   **`register_mcp_tool`**: A function for registering MCP tools with the global registry.
-   **`McpError`**: Error types specific to MCP operations.

## Usage (Conceptual)

```rust
// use rrust_kontekst_base::{McpConfig, McpToolInfo, register_mcp_tool, McpError};

// fn main() {
//     let config = McpConfig::default();
//     let tool_info = McpToolInfo { /* ... */ };
//     // register_mcp_tool(&tool_info, |params| { /* ... */ });
// }
```
