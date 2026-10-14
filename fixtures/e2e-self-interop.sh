#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CLI="${ROOT}/client/target/release/glitr-compat"
FIX="${ROOT}/fixtures/e2e-run"
test -x "$CLI" || (cd "$ROOT/client" && cargo build --release)
rm -rf "$FIX"
mkdir -p "$FIX/keys"
"$CLI" --password alicepass init --workdir "$FIX/alice" --display-name Alice --handle alice
"$CLI" --password bobpass init --workdir "$FIX/bob" --display-name Bob --handle bob
"$CLI" --password alicepass export-pubkey --workdir "$FIX/alice" --out "$FIX/keys/alice.pem"
"$CLI" --password bobpass export-pubkey --workdir "$FIX/bob" --out "$FIX/keys/bob.pem"
ALICE_ID=$("$CLI" --password alicepass profile --workdir "$FIX/alice" | python3 -c 'import sys,json; print(json.load(sys.stdin)["id"])')
BOB_ID=$("$CLI" --password bobpass profile --workdir "$FIX/bob" | python3 -c 'import sys,json; print(json.load(sys.stdin)["id"])')
"$CLI" --password alicepass add-contact --workdir "$FIX/alice" --peer-profile-id "$BOB_ID" --git-url "$FIX/bob" --display-name Bob --peer-public-key-file "$FIX/keys/bob.pem"
"$CLI" --password bobpass add-contact --workdir "$FIX/bob" --peer-profile-id "$ALICE_ID" --git-url "$FIX/alice" --display-name Alice --peer-public-key-file "$FIX/keys/alice.pem"
ALICE_C=$("$CLI" --password alicepass contacts --workdir "$FIX/alice" | awk 'NR==1{print $1}')
BOB_C=$("$CLI" --password bobpass contacts --workdir "$FIX/bob" | awk 'NR==1{print $1}')
"$CLI" --password alicepass send --workdir "$FIX/alice" --contact-id "$ALICE_C" --body "ping"
"$CLI" --password bobpass poll --workdir "$FIX/bob" --peer-workdir "$FIX/alice" --contact-id "$BOB_C"
"$CLI" --password bobpass inbox --workdir "$FIX/bob" | grep -q ping
echo "PASS self-interop"
