#!/usr/bin/env bash
# Recon↔recon git-mailbox bidirectional check (official-shaped cascade wire).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${ROOT}/client/target/release/glitr-compat"
FIX="${ROOT}/fixtures/recon-interop"
PASS=recon-pass

if [[ ! -x "$BIN" ]]; then
  echo "building glitr-compat..."
  (cd "${ROOT}/client" && cargo build --release)
  BIN="${ROOT}/client/target/release/glitr-compat"
fi

rm -rf "$FIX"
mkdir -p "$FIX"

"$BIN" --password "$PASS" init --workdir "$FIX/alice" --display-name Alice --handle alice
"$BIN" --password "$PASS" init --workdir "$FIX/bob" --display-name Bob --handle bob

ALICE_ID=$("$BIN" --password "$PASS" profile --workdir "$FIX/alice" | python3 -c 'import sys,json; print(json.load(sys.stdin)["id"])')
BOB_ID=$("$BIN" --password "$PASS" profile --workdir "$FIX/bob" | python3 -c 'import sys,json; print(json.load(sys.stdin)["id"])')

CID_A=$("$BIN" --password "$PASS" add-contact --workdir "$FIX/alice" --peer-profile-id "$BOB_ID" --git-url "$FIX/bob" --display-name Bob | sed -n 's/.*contact id=\([^ ]*\).*/\1/p')
CID_B=$("$BIN" --password "$PASS" add-contact --workdir "$FIX/bob" --peer-profile-id "$ALICE_ID" --git-url "$FIX/alice" --display-name Alice | sed -n 's/.*contact id=\([^ ]*\).*/\1/p')

"$BIN" --password "$PASS" send --workdir "$FIX/alice" --contact-id "$CID_A" --body "RECON_P0_ALICE_TO_BOB"
"$BIN" --password "$PASS" send --workdir "$FIX/bob" --contact-id "$CID_B" --body "RECON_P0_BOB_TO_ALICE"

"$BIN" --password "$PASS" poll --workdir "$FIX/bob" --peer-workdir "$FIX/alice" --contact-id "$CID_B"
"$BIN" --password "$PASS" poll --workdir "$FIX/alice" --peer-workdir "$FIX/bob" --contact-id "$CID_A"

echo "--- alice inbox ---"
"$BIN" --password "$PASS" inbox --workdir "$FIX/alice"
echo "--- bob inbox ---"
"$BIN" --password "$PASS" inbox --workdir "$FIX/bob"

# Structural check: no reconstruction markers
python3 - <<PY
import json, pathlib
for who in ("alice","bob"):
  for p in pathlib.Path("$FIX", who, "outbound").glob("*.json"):
    ct=json.loads(json.load(open(p))["ciphertext"])
    assert ct["v"]==3 and ct["scheme"]=="cascade"
    assert "finalCiphertext" in ct and "outerParameters" in ct
    assert "compat" not in ct and "hybrid" not in ct
    print(who, p.name, "OK official-shaped")
print("INTEROP PASS")
PY
