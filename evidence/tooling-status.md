# Tooling status (Phase 0)

- Date: 2026-10-14T09:18:28Z
- Node: v24.18.0
- REA CLI: rea-agents@6.4.0 (npx)
- GHIDRA_INSTALL_DIR: unset
- ghidraRun: missing
- analyzeHeadless: missing
- Hopper: not probed beyond doctor (missing_analysis_engine)
- pwntools: unavailable (No module named 'pwn')
- pyelftools: unavailable (No module named 'elftools')
- JADX / REA_JADX_MCP_JAR: unset

## Decision
Native function analysis: **no** (Ghidra unset). Proceed Phase 1 goldens without Ghidra.
