# WebRTC test recipe (recon)

## Automated (this session)

- Unit: `api_core::webrtc::tests::{chat_frame_roundtrip_json,signaling_post_list}` — PASS
- CLI: `glitr-compat signal` → `poll-signals` → `signals` between recon alice/bob — PASS

## Manual browser (human)

1. `glitr-compat serve --port 8787 --web-root ../glitr-web --workdir fixtures/recon-interop/alice`
2. Open two windows to http://127.0.0.1:8787/lab/webrtc.html
3. Offer/answer SDP exchange; send `ChatFrame` JSON on the data channel

STUN: `stun:stun.l.google.com:19302` (`DEFAULT_STUN`).

## Official interop

Not proven. Signaling schema matches `SignalingSession` (messaging v10). Encrypted official signaling ciphertext (cascade) not yet peeled.
