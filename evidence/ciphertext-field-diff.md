# Ciphertext field diff: official vs reconstruction

**Evidence:** `fixtures/official/golden/outbound.json` (plaintext `OFFICIAL_GOLDEN_1`), recon `fixtures/alice/outbound/*.json`.

## Official cascade envelope (observed)

JSON string in `outbound.ciphertext`:

| Field | Type | Notes |
| --- | --- | --- |
| `v` | number | `3` |
| `scheme` | string | `"cascade"` |
| `finalCiphertext` | string (b64) | 4626 bytes decoded |
| `outerParameters` | string (b64) | 1608 bytes decoded |

`outerParameters` decoded layout (**derived** from length-prefix walk):

| Offset | u32 BE length | Payload |
| --- | --- | --- |
| 0 | 12 | AES-GCM nonce (**inferred**) |
| 16 | 16 | AES-GCM tag (**inferred**) |
| 36 | 1568 | ML-KEM-1024 ciphertext (**observed** size match FIPS 203 / docs) |

Total 4+12+4+16+4+1568 = 1608.

## Reconstruction-only fields (must remove for P0)

| Field | Status |
| --- | --- |
| `compat` | invent — remove |
| `hybrid` | invent — remove |
| `ciphertext` (inner key name) | wrong — official uses `finalCiphertext` |
| `outer_parameters` (snake + object) | wrong — official uses `outerParameters` **base64 string** |

## Password envelope (observed official sealed file)

Keys: `version`, `kdf`, `aead`, `salt`, `nonce`, `ciphertext` (not `v`).

Argon2id m=19456 KiB, t=2, p=1 — **observed** via successful unseal of official `profile-secrets.json`.

## protocolBundles (observed)

Top keys: `signal`, `pqxdh`, `mlkemPublic`, `mlsKeyPackage`.

## protocolSecrets (observed keytree only)

See `official-secrets-keytree.json` / `official-protocol-session-keytree.json`. Includes `mlkemPrivate` (3168 ints), signal/pqxdh/mls material.

## Gate status

- Official golden ciphertext captured: **yes** → `fixtures/official/golden/`
- Field map written: **yes**
- Recon emit matches official keys (no `compat`/`hybrid`): **yes** (unit + `official-interop.sh`)
- Inner cascade after ML-KEM-AES still **unknown** without Ghidra / inner peel of a key-matched official golden
