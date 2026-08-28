# MCPWatch

**Local baseline monitoring for Model Context Protocol configuration files.**

> **Status:** development preview. No stable release has been published.

MCPWatch keeps a user-selected baseline copy of an MCP configuration and makes later file changes explicit without sending configuration data to a remote service.

## Current preview

Create a baseline:

```bash
mcpwatch init ~/.config/example/mcp.json ./mcp.baseline
```

Check the current config later:

```bash
mcpwatch check ~/.config/example/mcp.json ./mcp.baseline
```

Accept the current file as the new baseline:

```bash
mcpwatch update ~/.config/example/mcp.json ./mcp.baseline
```

The current implementation:

- stores the baseline locally at the path you choose
- writes baseline updates through a temporary file and rename
- compares files byte-for-byte
- reports the approximate first changed line and byte position
- exits `0` when unchanged, `3` when changed, and `2` on usage/read errors
- requires no background daemon

## Scope

The current preview detects **content change only**. It does not yet parse MCP configuration semantics or claim that a changed server, command, endpoint, permission, or tool is safe or unsafe. Structural MCP-aware summaries are planned for a later milestone.

## Build

Requires Rust 1.74 or newer.

```bash
cargo build --locked
cargo test --locked
```

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT © BLC Core Studio
