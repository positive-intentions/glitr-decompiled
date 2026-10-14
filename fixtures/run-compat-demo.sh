#!/usr/bin/env bash
# Reproduce alice→bob git-mailbox send/poll using glitr-compat only.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CLI="${ROOT}/client/target/release/glitr-compat"
FIX="${ROOT}/fixtures"
test -x "$CLI" || { echo "build client first: (cd $ROOT/client && cargo build --release)"; exit 1; }
exec bash -c '
FIX="'"$FIX"'"; CLI="'"$CLI"'"
# abbreviated — see evidence/FINDINGS.md for the full recorded run
echo "See fixtures/alice and fixtures/bob from the last investigation run."
ls -la "$FIX/alice" "$FIX/bob" 2>/dev/null || echo "run the agent fixture script first"
'
