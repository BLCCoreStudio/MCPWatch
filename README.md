# MCPWatch

**Focused local configuration-drift research for Model Context Protocol setups.**

> **Companion research status:** MCPWatch's baseline and drift-detection direction has been integrated into [MCPDoctor](https://github.com/BLCCoreStudio/MCPDoctor). This repository remains public as a focused implementation reference and development history; new integrated MCP diagnostics and drift work targets MCPDoctor.

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

## Why this repository still exists

MCPWatch is intentionally retained rather than deleted or republished. It preserves the smaller drift-monitoring experiment and existing links while MCPDoctor becomes the main product for MCP configuration health, executable diagnostics, security signals, and drift checks.

For active integration work, use **MCPDoctor**.

## Scope

This focused implementation detects **content change only**. It does not claim that a changed server, command, endpoint, permission, or tool is safe or unsafe. MCP-aware interpretation belongs in MCPDoctor as that capability is implemented and tested.

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
