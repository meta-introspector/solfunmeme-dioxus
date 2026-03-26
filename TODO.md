# Dioxus MCP Integration TODO

## Current Task
- [x] Add backend MCP gateway for ITIR Python tools in Dioxus non-WASM builds.
- [x] Prefer gateway tools in `MCPPlaygroundApp`.
- [x] Implement query/call flow with safe fallback to local `rrust_kontekst_base` registry.
- [x] Add docs/notes describing endpoint and behavior.

## Follow-up
- [ ] Add same-origin proxy for web builds (`/api/itir-mcp/*` hosted on Dioxus app server).
- [ ] Surface richer gateway failure diagnostics in the MCP UI.
- [ ] Add integration tests around tool-list parsing and gateway call error path.
