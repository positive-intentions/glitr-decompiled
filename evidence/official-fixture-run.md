# Official fixture run

## Tooling

- Binary: `target/glitr-tui-linux` (sha256 in digests.txt)
- Driver: `fixtures/official/drive_official_tui.py` (bootstrap), pyte+pexpect send script (session)
- Passwords (local only): `official-alice-pass`, `official-bob-pass`
- Plaintext: `OFFICIAL_GOLDEN_1`

## Steps (observed)

1. Spawn TUI; at profile picker type absolute path `fixtures/official/alice` (or bob).
2. Enter encryption password when prompted; leave git remote empty (local mailbox).
3. After UI shows Chats: mouse-click `2:Contacts` (SGR), press `a` for Add contact.
4. Form fields (Tab between, Enter submit): Display name → Peer git URL → Username → Token.
5. Add bob path as Peer git URL; Submit.
6. Open contact, `m` / `i`, type `OFFICIAL_GOLDEN_1`, Enter send.

## Artifacts

| Path | Role |
| --- | --- |
| `fixtures/official/alice/` | Full official mailbox (includes secrets — do not publish) |
| `fixtures/official/bob/` | Peer mailbox |
| `fixtures/official/golden/outbound.json` | Golden outbound row |
| `fixtures/official/golden/plaintext.txt` | Known plaintext |
| `fixtures/official/golden/*-profile-public.json` | Public profiles + bundles |
| `evidence/official-outbound-ciphertext-shape.json` | Shape map |

## Digests

```
EOF
sha256sum /home/raju/repositories/rea/target/glitr-investigation/fixtures/official/golden/* 2>/dev/null
0a84326b1ad464f761e12476c646d6859834bc6a914954c894c445acc2e070c7  /home/raju/repositories/rea/target/glitr-investigation/fixtures/official/golden/alice-profile-public.json
30ab46de203e9249744dad6897f229d9db508e78e1e138de50a512e0133568f9  /home/raju/repositories/rea/target/glitr-investigation/fixtures/official/golden/bob-profile-public.json
271368eb6e9c5a29a85794baf07de0d7caa7777d5f2c12fb7748c9121aa9b6a1  /home/raju/repositories/rea/target/glitr-investigation/fixtures/official/golden/outbound.json
60f2ee4b67cda6284b6140e25011f647004db6b32d847a3eb3f2ec272a1538cb  /home/raju/repositories/rea/target/glitr-investigation/fixtures/official/golden/plaintext.txt
