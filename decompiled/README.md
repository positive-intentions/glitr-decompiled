# Decompiled Glitr crates (reconstruction)

Faithful **crate-shaped** reconstruction from:

- Mirrored docs under `../docs-mirror/`
- Embedded debug paths / symbols / GraphQL SDL in `../evidence/`

This is **not** a line-by-line dump of the 68MB+ binaries. Modules mirror observed crate boundaries; wire formats that remain UNKNOWN are marked in source.

## Crates

| Crate | Role |
| --- | --- |
| `messaging-schema` | Messaging schema **v10** types (binary SDL) |
| `crypto-core` | AES-GCM, Argon2id envelopes, RSA hybrid |
| `crypto-cascade` | Cascade v3 envelope + reconstruction encrypt |
| `git-core` | Workdir git via system `git` |
| `db-core` | File store + sealing |
| `api-core` | Connect / profile / contacts / send / poll |
| `glitr-client` | `ApiHandle` + `SavedConnect` |

## Build

```bash
cargo test --workspace
```
