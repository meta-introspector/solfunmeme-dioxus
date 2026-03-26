# Solfunmeme Dioxus Context Snapshot

Date: 2026-03-26

- User request: move MCP integration work into the Dioxus layer, after server-side MCP exposure exists for ITIR.
- Decision: implement a backend HTTP gateway in Dioxus for MCP tool list/schema/call, then consume it from the MCP playground UI with local registry fallback.
- Active surface: `solfunmeme-dioxus/src/mcp_gateway.rs`, `solfunmeme-dioxus/src/playground/mcp.rs`, `solfunmeme-dioxus/src/main.rs`.
- Constraint: do not run MCP stubs directly in browser/WASM; use backend service for tool transport.
- Open risk: the SPA path expects an MCP endpoint; browser and desktop paths differ in host/port assumptions unless a same-origin proxy is added later.
