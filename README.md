# Glitr investigation workspace

Evidence-only reconstruction of Glitr from:

- Binaries in `../` (`glitr-tui-linux`, `glitr-linux.deb`, `glitr-android.apk`, `glitr-web/`)
- Public docs on [glitr.io/docs](https://glitr.io/docs/)

## Layout

| Path           | Role                                                              |
| -------------- | ----------------------------------------------------------------- |
| `docs-mirror/` | Scraped glitr.io pages                                            |
| `evidence/`    | Digests, REA output, string/symbol inventories, extracted GraphQL |
| `decompiled/`  | Faithful crate-shaped Rust reconstruction                         |
| `client/`      | Compatible git-mailbox CLI                                        |
| `fixtures/`    | Synthetic mailboxes produced by our client                        |

## Claim tags

Use in notes and `// EVIDENCE:` comments:

- **observed** — from binary bytes or mirrored docs
- **derived** — from schema/layout rules
- **inferred** — plausible behavior not fully proven
- **unknown** — not established

## Build

```bash
cd decompiled && cargo test -p crypto-core -p crypto-cascade -p api-core
cd ../client && cargo build --release
```

## Test (no agent needed)

See **[HOWTO-TEST.md](HOWTO-TEST.md)** — recon↔recon interop script, WebRTC lab, and `glitr-compat serve` for official `glitr-web`.

```bash
./fixtures/official-interop.sh
```

## Interop note

Official binary embeds messaging schema **version 10** (observed). Docs describe v8. Reconstruction targets v10 from the TUI binary SDL extract. Recon cascade uses official outer wire (ML-KEM-1024); full official inner Signal/PQXDH/MLS remains unknown without Ghidra.
