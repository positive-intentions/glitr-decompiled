# How to test the Glitr reconstruction

You should not need to re-prompt the agent for these steps. After build, use the commands below.

## 1. Build

```bash
cd target/glitr-investigation/decompiled && cargo test -p crypto-core -p crypto-cascade -p api-core
cd ../client && cargo build --release
```

## 2. Git-mailbox recon↔recon (P0 shape)

```bash
./fixtures/official-interop.sh
```

Expect `INTEROP PASS` and both inboxes showing the peer plaintext. Outbound ciphertext must use `finalCiphertext` / `outerParameters` with no `compat` / `hybrid` keys.

## 3. Official glitr-web UI + recon API

```bash
cd client
cargo run --release -- --password investigation serve \
  --port 8787 \
  --web-root ../../glitr-web \
  --workdir ../fixtures/recon-interop/alice   # after running official-interop.sh
```

Open:

- http://127.0.0.1:8787/ — official shipped WASM UI (`target/glitr-web`)
- http://127.0.0.1:8787/lab/webrtc.html — recon WebRTC data-channel lab
- http://127.0.0.1:8787/api/profile — recon profile JSON

The official WASM app talks to its own storage; use it to explore UI chrome. Recon mailbox ops remain on `glitr-compat` CLI / `/api/*`.

## 4. WebRTC lab (browser)

1. Open `/lab/webrtc.html` in two browser profiles/windows.
2. Window A: role `offer` → Create local offer/answer → copy Local SDP.
3. Window B: role `answer` → paste into Remote SDP → Set remote → copy Local SDP back to A → Set remote.
4. When the data channel opens, Send chat. Frames are `ChatFrame`-shaped JSON (`{"type":"chat","body":"..."}`).

Optional git signaling (mailbox-backed):

```bash
glitr-compat signal --workdir alice --recipient-id <bob-id> --role offer --kind offer --sdp '...'
glitr-compat poll-signals --workdir bob --peer-workdir alice
glitr-compat signals --workdir bob --recipient-id <bob-id>
```

## 5. Official TUI interop (best-effort)

Official cascade inner layers (Signal/PQXDH/MLS) are not fully recovered without Ghidra. Recon uses official **outer** wire (ML-KEM-1024 + `ML-KEM-1024-AES-GCM`) plus RSA hybrid inner. Official may reject recon ciphertext until those inner layers match.

Golden fixtures: `fixtures/official/golden/` (public). Secrets stay under `fixtures/official/sensitive/` (gitignored).

## Known gaps

- Tor onion path deferred
- Full ratatui chrome parity deferred
- Official↔recon decrypt of full cascade inner stack: UNKNOWN without Ghidra
