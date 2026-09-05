# MCPWatch

> **Project status: sunset / discontinued.**

MCPWatch was a focused local configuration-drift experiment for Model Context Protocol setups. Its baseline and drift-detection direction was later integrated into MCPDoctor, which has itself now been discontinued as BLCCoreStudio reduces overlapping experimental projects.

The repository remains public for historical reference and to preserve existing links and commit history, but **no further feature development or routine maintenance is planned**.

## Historical scope

MCPWatch explored a local-only workflow for:

- saving a user-selected configuration baseline;
- comparing later configuration changes byte-for-byte;
- reporting the approximate first changed line and byte position;
- updating the baseline explicitly rather than through a background daemon;
- keeping configuration data on the local machine.

The project detected content changes only; it did not determine whether a changed server, command, endpoint, permission, or tool was safe or unsafe.

## Historical source

Previous implementation details, tests, documentation, and development history remain available through the Git history.

## License

MIT © BLC Core Studio
