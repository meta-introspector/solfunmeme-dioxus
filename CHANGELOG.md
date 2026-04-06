# Changelog

## Unreleased

### Added
- Added an ITIR MCP gateway server in Dioxus (`src/mcp_gateway.rs`) with tool list/schema/call endpoints.
- Wired `MCPPlaygroundApp` to fetch tools and invoke calls through the gateway when available, with a local `rrust_kontekst_base` fallback.
- Added startup behavior to run the gateway automatically on non-WASM targets.

### Notes
- No API-breaking changes were introduced.
- Default gateway bind remains `127.0.0.1:3939`, path `/api/itir-mcp`.
