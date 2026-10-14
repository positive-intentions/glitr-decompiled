# Source: https://glitr.io/docs/technical/cryptography/
# HTTP: 200

Technical
Cryptography

# Cryptography

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. Recipient encryption is three libraries composed into one cascade. signal-protocol is the classical handshake and ratchet. ml-kem is ML-KEM-1024. pqxdh combines those two toward Signal’s post-quantum extended Diffie–Hellman. A separate mls library scaffolds RFC 9420 group messaging research; it is not in the product cascade yet.
This page is those libraries. Password sealing of your mailbox files lives in the crypto crate on Persistence. The product cascade — AES, then Signal, then PQXDH, then RSA hybrid, then ML-KEM — is specified on Signal and post-quantum.
signal-protocol is the only open-source library in this stack (source). ml-kem, pqxdh, mls, and crypto are closed-source. Tests and proofs are on Formal verification. These implementations are educational / research. Signal/PQXDH are not wire-compatible with the Signal app; MLS aims at RFC 9420 suite-1 mls-rs wire.

## signal-protocol​

The classical handshake and ratchet: X3DH key agreement, the Double Ratchet, identity keys, signed prekeys, one-time prekeys, and AES-256-GCM message encryption. Rust core, compiled to WebAssembly, with TypeScript bindings.
Open the Signal experience — the public React storybook.
Source
Formal verification — tests, ProVerif, extract depth (same claim words as ml-kem).
A second, local-only Dioxus gallery (signal-protocol-gallery) reuses whatsup-ui chrome and calls signal-protocol-core directly. It does not replace the React deploy, and it has no GitHub Pages URL.
Formal verification of that library (ProVerif, hax/F*, Rocq, Lean) is documented on Signal Protocol formal verification. Treat it as research-grade work on the handshake library, not an audit of the messenger.
Try path: Signal Protocol.

## ml-kem​

Signal alone does not meet a post-quantum requirement. ml-kem is a pure-Rust ML-KEM-1024 (FIPS 203) wrapper for this stack.
Piece
Role
ml-kem-core
Thin wrapper over libcrux-ml-kem 0.0.10 — the same crate and pin as libsignal — plus hybrid AES-256-GCM.
ml-kem-wasm
wasm-bindgen façade.
ml-kem-gallery
Dioxus demos on whatsup-ui chrome.
ml-kem is closed-source. Tests, models, and type-checks for the wrapper are reported on ML-KEM formal verification; lattice math stays in libcrux. The crate targets the 1024 parameter set (Signal PQXDH production): 1568-byte public key, 3168-byte expanded secret key, 1568-byte ciphertext, 32-byte shared secret. Secret keys are the expanded FIPS form.
Hybrid encrypt in core: encapsulate, HKDF-SHA256 with info ML-KEM-1024-AES-GCM, then AES-256-GCM. This crate is a different parameter set and key encoding from the older JS ML-KEM-768 layer.
Open the ML-KEM gallery
Story
Live
Key generation
gui/ml-kem/key-generation
Encapsulate
gui/ml-kem/encapsulate
Hybrid encrypt
gui/ml-kem/hybrid-encrypt

## pqxdh​

pqxdh is the composition step: signal-protocol-core (X25519 ECDH, HKDF-SHA-256, Ed25519, Double Ratchet) plus ml-kem-core (ML-KEM-1024), aimed at Signal PQXDH revision 3.
Documented deviations from the spec:
Spec
This crate
XEdDSA on a single X25519 identity
Separate X25519 (DH) and Ed25519 (signatures)
SHA-256 or SHA-512
HKDF-SHA-256 via signal-protocol-core
libsignal encodings / protobufs
Simple EncodeEC / EncodeKEM with type tags
Parameters: curve X25519, hash SHA-256, info PQXDH-v1, pqkem ML-KEM-1024, AEAD AES-256-GCM. It is a spec-faithful educational implementation. It is not libsignal wire-compatible.
Formal verification of that composition (ProVerif handshake and linked session, hax stubs) is on PQXDH formal verification.
Open the PQXDH gallery
Story
Live
Key generation
gui/pqxdh/key-generation
Prekey bundle
gui/pqxdh/prekey-bundle
Handshake
gui/pqxdh/handshake
Initial message
gui/pqxdh/initial-message
Session
gui/pqxdh/session
Errors
gui/pqxdh/errors
Try path: Post-quantum.

## mls​

mls is an RFC 9420 façade over AWS mls-rs (not OpenMLS). It is closed-source. Product 1:1 runs pairwise MLS inside encrypt_for_peer. Glitr groups use a separate N-party GroupSession for chat plaintext; each member still receives an individually sealed delivery envelope (seal_outer_for_peer).
Crate
Role
mls-core
Suite-1 façade: identity, key package, create/add/welcome, application encrypt/decrypt
mls-wasm
wasm-bindgen façade
mls-gallery
Dioxus demos on whatsup-ui chrome
Parameter
Value
Suite
MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519 (id 1)
Default crypto
mls-rs-crypto-rustcrypto
Optional
sibling-crypto (X25519 via signal-protocol-core + local AES-128-GCM)
Tests and stub verify jobs: MLS formal verification.
Open the MLS gallery
Demo
Link
Key generation
gui/mls/key-generation
Create group
gui/mls/create-group
Add member
gui/mls/add-member
Encrypt decrypt
gui/mls/encrypt-decrypt
Errors
gui/mls/errors

## How the product uses them​

The shipped recipient path stacks these libraries as defense-in-depth: AES → MLS → Signal → PQXDH on the inside, then RSA hybrid → ML-KEM on the outside. PQXDH already includes X25519, ML-KEM-1024, and a ratchet. The extra Signal, pairwise MLS, and standalone ML-KEM wraps are independent layers on the same payload — not a spec-recommended profile. MLS key packages publish on protocolBundles; welcome bytes ride in the first-message handshake pack; group state persists in the per-contact session.
Full wording: Signal and post-quantum. The cascade manager and password envelopes are on Persistence. Modeled cascade composition (not a messenger proof) is on crypto formal verification.
Next: Persistence — git as storage, a crypto utility, and a GraphQL store over files.