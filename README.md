# mcp-searxng

> MCP server that provides privacy-respecting web search capabilities through [SearXNG](https://github.com/searxng/searxng).

A lightweight Rust MCP (Model Context Protocol) server that connects AI assistants to a SearXNG instance, enabling search across multiple categories: general web, images, videos, news, music, IT, science, files, social media, and maps.

## Features

- **10 specialized search tools** — one per SearXNG category for precise results
- **Two transport modes** — stdio for Claude Desktop/Cline, HTTP for remote access
- **Configurable defaults** — language, safe search, time range, and pagination
- **Privacy-first** — your own SearXNG instance, no third-party tracking
- **Minimal footprint** — single binary, ~5 MB after strip, Alpine-based Docker image
- **Markdown output** — results formatted as clean, readable markdown

## Prerequisites

- [Rust](https://www.rust-lang.org/) 1.93+ (edition 2024)
- A running [SearXNG](https://docs.searxng.org/) instance (self-hosted or public)
- `build-essential` / `build-base` for compiling OpenSSL from source (vendored)

## Quick Start

### Build and run

```bash
cargo build --release

SEARXNG_URL=https://your-searxng-instance.com cargo run
```

### Use with Docker

```bash
docker run --rm -e SEARXNG_URL=https://your-searxng-instance.com atareao/mcp-searxng
```

Or build locally:

```bash
just build
docker run --rm -e SEARXNG_URL=https://your-searxng-instance.com atareao/mcp-searxng:latest
```

## Configuration

| Variable | Default | Required | Description |
|---|---|---|---|
| `SEARXNG_URL` | — | Yes | SearXNG instance base URL |
| `MCP_TRANSPORT` | `stdio` | No | Transport mode: `stdio` or `http` |
| `MCP_HOST` | `0.0.0.0` | No | HTTP bind address |
| `MCP_PORT` | `3005` | No | HTTP bind port |
| `SEARXNG_LANG` | `es` | No | Default search language (ISO 639-1) |
| `SEARXNG_FORMAT` | `json` | No | API response format |

## Usage

### MCP Tools

Each tool accepts the same parameters:

| Parameter | Type | Required | Description |
|---|---|---|---|
| `query` | string | Yes | The search query |
| `language` | string | No | Language code (e.g. `en`, `es`). Overrides the default. |
| `pageno` | number | No | Page number (default: 1) |
| `time_range` | string | No | Time filter: `day`, `month`, or `year` |
| `safesearch` | number | No | Safe search level: `0` (off), `1` (moderate), `2` (strict) |

| Tool | Description |
|---|---|
| `search` | General web search across all categories |
| `search_images` | Search for images |
| `search_videos` | Search for videos |
| `search_news` | Search for news articles |
| `search_music` | Search for music |
| `search_it` | Search for IT and technology topics |
| `search_science` | Search for scientific and academic topics |
| `search_files` | Search for downloadable files |
| `search_social` | Search social media content |
| `search_map` | Search for locations and map data |

### Integrate with Claude Desktop

Add the server to your Claude Desktop configuration (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "searxng": {
      "command": "docker",
      "args": ["run", "--rm", "-i", "-e", "SEARXNG_URL=https://your-searxng-instance.com", "atareao/mcp-searxng"]
    }
  }
}
```

Or with a local binary:

```json
{
  "mcpServers": {
    "searxng": {
      "command": "/path/to/mcp-searxng",
      "env": {
        "SEARXNG_URL": "https://your-searxng-instance.com"
      }
    }
  }
}
```

### HTTP Transport

For remote or networked access, use HTTP transport:

```bash
MCP_TRANSPORT=http MCP_PORT=3005 SEARXNG_URL=https://your-searxng-instance.com ./target/release/mcp-searxng
```

The server will listen on `0.0.0.0:3005` by default.

## Development

This project uses [just](https://github.com/casey/just) as a command runner:

```bash
just list    # Show available commands
just build   # Build Docker image
just lint    # Run clippy
just fmt     # Check formatting
just push    # Push Docker image
```

Or use Cargo directly:

```bash
cargo build
cargo run
cargo clippy -- -D warnings
cargo fmt
```

## Architecture

```
src/
├── main.rs      # Entry: env → config → transport (stdio/http) → boot
├── config.rs    # AppConfig from environment variables
├── handler.rs   # ServerHandler implementation (tool dispatch)
├── error.rs     # Error types (thiserror)
├── searxng.rs   # HTTP client + SearXNG response types
└── tools.rs     # 10 MCP tools via #[mcp_tool] macros + tool_box! dispatch
```

## License

MIT
