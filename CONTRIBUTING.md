# Contributing

Contributions around safe local snapshots, structural MCP diffs, secret minimization, tests, and documentation are welcome.

Before opening a pull request:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Changes that persist MCP configuration data must document what is stored and why. Follow `SECURITY.md` for vulnerability reports.
