# AGENTS.md

## Project

MCP server for SearXNG search. Single-crate Rust binary using `rust-mcp-sdk` 0.9 with `#[mcp_tool]` macros and `tool_box!` dispatch.

## Toolchain

- Rust 1.93+, edition 2024
- `openssl` is **vendored** — builds without system OpenSSL but needs `cc` and build essentials
- Version bumps via `.vampus.yml` (tool: [vampus](https://github.com/fabn/vampus))

## Commands

```
cargo build          # build
cargo run            # run
cargo test           # no tests yet
cargo clippy -- -D warnings  # lint
cargo fmt            # format
```

## Architecture

```
src/
├── main.rs          # Entry: env → config → transport (stdio/http) → boot
├── config.rs        # AppConfig from env vars with defaults
├── handler.rs       # ServerHandler impl (list_tools, call_tool via tool_box dispatch)
├── error.rs         # thiserror-based error types
├── searxng.rs       # reqwest client + SearXNG response types
└── tools.rs         # 10 tools via #[mcp_tool] macro + tool_box! enum
```

## Environment Variables

| Variable | Default | Required | Description |
|---|---|---|---|
| `SEARXNG_URL` | — | Yes | SearXNG instance base URL |
| `MCP_TRANSPORT` | `stdio` | No | `stdio` or `http` |
| `MCP_HOST` | `0.0.0.0` | No | HTTP bind address |
| `MCP_PORT` | `3005` | No | HTTP bind port |
| `SEARXNG_LANG` | `es` | No | Default search language (ISO 639-1) |
| `SEARXNG_FORMAT` | `json` | No | API response format |

## MCP Tools

10 tools via `tool_box!(SearxngTools, [...])`: `search`, `search_images`, `search_videos`, `search_news`, `search_music`, `search_it`, `search_science`, `search_files`, `search_social`, `search_map`.

Each accepts: `query` (required), `language`, `pageno`, `time_range`, `safesearch`. Uses SearXNG defaults for engines. Results formatted as markdown text.

## Key Patterns (from rust-mcp-sdk 0.9)

- Tools: `#[mcp_tool(...)]` + `JsonSchema` derive + `tool_box!` macro
- Dispatch: `SearxngTools::try_from(params).map_err(CallToolError::new)?`
- Results: `CallToolResult::text_content(vec![TextContent::from(text)])`
- Handler: `handler.to_mcp_server_handler()` for `McpServerOptions`
- Stdio: `StdioTransport::new(TransportOptions::default())?` → `server_runtime::create_server()` → `server.start().await`
- HTTP: `hyper_server::create_server(details, handler, HyperServerOptions { host, port, ..Default::default() })` → `server.start().await`
- Need `use rust_mcp_sdk::McpServer` in scope for `.start()` method
- Protocol: `ProtocolVersion::V2025_11_25.into()`
- Capabilities: `ServerCapabilities { tools: Some(ServerCapabilitiesTools { list_changed: None }), ..Default::default() }`

## Dependencies

- `rust-mcp-sdk` (0.9) — MCP Server SDK with stdio + streamable-http transports, macro support
- `reqwest` (0.12) — HTTP client for SearXNG API calls
- `tokio` (full) — async runtime
- `serde` / `serde_json` — serialization
- `tracing` + `tracing-subscriber` (env-filter) — logging
- `anyhow` / `thiserror` — error handling
- `async-trait` — trait impl for `ServerHandler`
