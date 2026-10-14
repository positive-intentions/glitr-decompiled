# Web GUI test recipe

## Automated (this session)

```text
curl http://127.0.0.1:8787/            → 200 (official glitr-web index)
curl http://127.0.0.1:8787/lab/webrtc.html → 200
curl http://127.0.0.1:8787/api/profile → recon ProfilePublic JSON
```

## Human

```bash
cd target/glitr-investigation/client
cargo run --release -- --password recon-pass serve \
  --port 8787 \
  --web-root ../../glitr-web \
  --workdir ../fixtures/recon-interop/alice
```

- Official WASM UI: explore chrome/connect flows (its own storage).
- Recon mailbox: CLI + `/api/*` + WebRTC lab.

See [HOWTO-TEST.md](../HOWTO-TEST.md).
