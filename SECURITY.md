# Security Policy

MCPWatch is under active development and should be treated as a development-preview change detector, not a complete MCP security analyzer.

The current implementation stores a byte-for-byte baseline copy of a user-selected MCP configuration at the local path you provide. Baseline creation and updates are written through a temporary file and rename. `check` compares the current file with that baseline locally and does not upload configuration data.

Because baseline files can contain the same sensitive endpoint, command, credential, or environment information as the source configuration, choose their storage location and permissions accordingly. MCPWatch currently detects content changes only; it does not determine whether a changed MCP server, command, endpoint, permission, or tool is safe or unsafe.

Please report suspected vulnerabilities privately through GitHub private vulnerability reporting when available or another appropriate private channel. Never attach real access tokens, API keys, private keys, or private configuration secrets.
