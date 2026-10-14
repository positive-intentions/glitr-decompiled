# REA readiness

- CLI package: rea-agents@6.4.0
- Cursor registration: aligned (`npx -y rea-agents@6.4.0 mcp`)
- Node for analysis: 24.18.0 (via nvm; session default was 20.x)
- Ghidra: not configured (GHIDRA_INSTALL_DIR unset)
- Hopper: not available
- Host: Ubuntu 22.04 (informational unsupported_host)
- MCP in this agent session: user-rea previously failed live discovery; prefer CLI until reconnect
- Strategy: target-free CLI (`inspect_rust_binary`, `inspect_wasm_artifact`, `analyze_javascript_application`, `inspect_android_package`, `inspect_binary_layout`) + string/symbol extraction from artifacts

Doctor JSON: `rea-doctor.json`
Setup dry-run: `rea-setup-dry-run.json` (no Cursor write actions planned)
