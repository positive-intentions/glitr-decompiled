# Glitr investigation findings

Evidence boundary: `target/` binaries + [glitr.io docs](https://glitr.io/docs/) only. No other machine paths or source trees were opened. Embedded CI paths inside binaries were used as **artifact strings** only.

## Artifact identity (observed)

| File | SHA-256 |
| --- | --- |
| `glitr-tui-linux` | `3cf9c44ccd0f7d99a1d6be307d9d2c1767e2360c80b243f93f318dfcedb773b0` |
| `glitr-linux.deb` | `53f5c733d9e49e3a096b192a0522a6b5bbde45aec9322f7ef42a4d47ec0ee64e` |
| `glitr-android.apk` | `4e2d3174a776fd3361d6dbc4eb8a64d69640f69e5767235e128e0aa7826a1196` |
| `glitr-web` build | version `56498d4ea157` @ `2026-10-06T12:21:02Z` |

REA CLI `inspect-artifact` on the TUI: ELF, 71208736 bytes. Deep native decompile unavailable (no Ghidra/Hopper/pwntools). JADX unavailable for APK code.

## Product map (docs + binary)

**Observed** crate tree from embedded debug paths (`evidence/embedded-all-crate-paths.txt`): `api-core`, `glitr-client`, `crypto-cascade`, `crypto-core`, `db-core`, `git-core`, `ml-kem-core`, `mls-core`, `pqxdh-core`, `signal-protocol-core`, `tor-core`, `webrtc-core`, `whatsup-ui`, …

## Schema version (docs vs binary)

| Source | Claim |
| --- | --- |
| Docs Data structures | messaging **v8** |
| Binary SDL extract | messaging **v10** (`evidence/messaging.v10.graphql`) |

**Authoritative for reconstruction:** v10 from the TUI binary.

Sealed collections (`@encrypted`): contacts, inbox, sent, groups, protocol-sessions, group-protocol-sessions, file-transfers, profile-secrets. Visible: profile-public, outbound, readreceipts, signaling, devices.

## Crypto (observed / derived)

| Layer | Evidence | Reconstruction status |
| --- | --- | --- |
| Password at rest | Official fixtures: `version`/`kdf`/`aead` ints (1/1/1); Argon2id 19MiB t=2 p=1 → AES-256-GCM | **Implemented** official shape; unit-tested |
| Hybrid v1 | Docs + binary `wrappedKey/nonce/ciphertext`, RSA-OAEP-4096 | Implemented; round-trip tested; official uses base64 DER keys |
| Cascade v3 outer | Golden: `{v:3,scheme:"cascade",finalCiphertext,outerParameters}`; outerParameters BE 12+16+1568; HKDF info `ML-KEM-1024-AES-GCM`; libcrux-ml-kem 0.0.10 | **Implemented**; recon emits official field names (no `compat`/`hybrid`) |
| Cascade inner | Docs AES→MLS→Signal→PQXDH then RSA | Recon uses **RSA hybrid JSON** as inner after ML-KEM peel. Official Signal/PQXDH/MLS byte layout still **UNKNOWN** without Ghidra |
| protocolBundles | Observed keys: `signal`,`pqxdh`,`mlkemPublic`,`mlsKeyPackage` | Documented; recon generates real ML-KEM + placeholder classical material |
| Live frames | Binary `ChatFrame::{Hello,Chat,Receipt,File*}` | Typed + WebRTC lab encode |

## Session rubric (this run)

| Question | Answer |
| --- | --- |
| Tooling native analysis? | **No** — Ghidra/`GHIDRA_INSTALL_DIR` unset (see `tooling-status.md`) |
| Official golden path? | **Yes** — `fixtures/official/golden/` + field diff |
| Official→recon decrypt? | **Not established** — inner cascade UNKNOWN; outer shape recovered |
| Recon→official? | **Not established** — same inner gap; wire shape matches golden keys |
| Recon↔recon decrypt? | **Yes** — `fixtures/official-interop.sh` → `INTEROP PASS` |
| `protocolBundles` documented? | **Yes** — keytree + profile setup |
| Cascade inner still unknown? | **Yes** (Signal/PQXDH/MLS) |
| WebRTC testable? | **Yes** — signaling CLI + `/lab/webrtc.html` (recon↔recon) |
| Web GUI openable? | **Yes** — `glitr-compat serve` serves `target/glitr-web` + lab |
| FINDINGS.md updated? | **Yes** (this file) |

## Compatible client results

Built: `client/target/release/glitr-compat`.

Recon interop (`fixtures/recon-interop/` via `fixtures/official-interop.sh`):

1. Init alice/bob with schema v10 + real ML-KEM bundles
2. Add contacts (auto-load peer `profile-public.json` from local path)
3. Bidirectional send/poll — plaintext recovered both ways
4. Outbound JSON uses official cascade keys only

Web lab: `glitr-compat serve --web-root ../glitr-web --workdir <mailbox>` → see [HOWTO-TEST.md](../HOWTO-TEST.md).

## Deferred (documented only)

- Tor onion connect / meet flows
- Full ratatui TUI chrome parity
- Android APK UI beyond evidence mining
- Official full-cascade inner ratchet (needs Ghidra or more golden inner peels)

## Remaining unknowns (priority)

1. Official cascade **inner** bytes after ML-KEM-AES peel (Signal/PQXDH/MLS)
2. Soft-id mismatch: official `profile-public.id` ≠ `profile-secrets.id` while RSA/ML-KEM keys match (observed; messaging uses public id)
3. Official TUI peer-cache pollution when XDG isolation is incomplete (stale `peerProfileId`)
4. Exact `mlsKeyPackage` / PQXDH wire encodings beyond observed key names

## Deliverables map

| Path | Contents |
| --- | --- |
| `docs-mirror/` | Scraped glitr.io pages |
| `evidence/` | Digests, REA JSON, strings, SDL, this ledger |
| `decompiled/` | Rust crate reconstruction |
| `client/` | `glitr-compat` CLI (+ `serve`) |
| `fixtures/official/` | Official TUI goldens + drivers |
| `fixtures/recon-interop/` | Recon bidirectional mailboxes |
| `fixtures/test-lab/` | WebRTC lab HTML |
| `HOWTO-TEST.md` | Human test recipe |
