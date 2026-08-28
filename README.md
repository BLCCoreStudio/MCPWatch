# MCPWatch

**Monitor local MCP configurations for unexpected server, tool, permission, and endpoint changes.**

> **Status:** early development. No stable release has been published.

MCPWatch is intended to maintain a local baseline for selected Model Context Protocol configuration files and make later changes explicit for review.

## Planned v0.1

- explicit user-selected configuration paths
- local baseline snapshots
- detect content changes without sending config data elsewhere
- later structural summaries for server, command, endpoint, and permission changes
- clear separation between file-change detection and security judgment
- no background daemon required for the initial release

The current repository contains a development scaffold only. Snapshot persistence and structural MCP comparison are not implemented yet.

## Build

Requires Rust 1.74 or newer.

```bash
cargo build
cargo test
```

## Security

See [SECURITY.md](SECURITY.md).

## License

MIT © BLC Core Studio
