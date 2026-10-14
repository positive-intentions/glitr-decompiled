# Source: https://glitr.io/docs/technical/formal-verification/
# HTTP: 200

Technical
Formal verification
Formal verification

# Formal verification

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. Five libraries in this stack have production tests and formal-verification scaffolding: signal-protocol (X3DH and the Double Ratchet), ml-kem (ML-KEM-1024 hybrid wrapper), pqxdh (PQXDH v3 composition plus a linked handshake→session path), mls (RFC 9420 façade over mls-rs; stub symbolic/extract pipelines), and crypto (password envelopes in crypto-core; modeled recipient-cascade composition in crypto-cascade). Signal Protocol is the only open-source repo in the product (source). The others are closed-source. This section reports what was checked — not a proof that a production function is a theorem.
F*, Rocq, and Lean do not check the libcrux- or X25519-backed production functions.

## How to read this section​

Page
Role
Verification report
Shared summary, then sections for developers, cryptographers, and security reviewers
Signal Protocol
X3DH and Double Ratchet models, extraction, tests
ML-KEM
Hybrid wrapper, ProVerif, pinned libcrux lattice proofs
PQXDH
Handshake, hybrid binding, linked session models
MLS
RFC 9420 suite-1 façade over mls-rs; stub ProVerif/extract jobs
crypto
Password envelope (crypto-core) and cascade composition (crypto-cascade)
Demos: Signal gallery, ML-KEM gallery, PQXDH gallery, MLS gallery, crypto gallery. Signal source: positive-intentions/signal-protocol.

## What “formal verification” means here​

Three activities share the label. They do not prove the same thing.
Tests run on the production build. Extraction type-checks a stub build. The arrow from tests to extract is omitted on purpose: passing tests does not validate the extract path.
Rust llvm-cov is the core crate (and crypto-cascade). WASM bindings are Node/wasm-pack, not the rust gate. coverage(off) is reserved for proven-unreachable helpers: signal/pqxdh hkdf_derive_short (HKDF expand of ≤64 bytes); crypto-core chunk_prefix_len and read_u32_be; cascade u32_from_be4. ml-kem has none.
Kind
Tool
What it checks
Executable tests
Rust, Jest, wasm-pack
Concrete behaviour of the production build
Symbolic protocol model
ProVerif
An abstract process (DH, KEM, HKDF, AEAD as functions) against secrecy and correspondence queries
Extraction + type-check
hax into F*, Rocq, Lean 4
Control flow of the stub crate (crypto replaced by axioms). Not the production backend.
Upstream primitive proofs
libcrux’s hax/F* suite
Portable ML-KEM field arithmetic at a pinned crate version
Production builds still call real primitives (X25519, AES-GCM, libcrux ML-KEM). The verified Rust build turns those calls into stubs so extraction does not pull the concrete crates. There is no machine-checked refinement from the stub to the production backend.

## Claim vocabulary​

Every result on these pages uses one of six words:
Word
Meaning
Tested
An executable test failed or passed on the production build
Modeled
A ProVerif query was discharged on an abstract process
Type-checked
Extracted stub code or a hand-written spec module checked in F*, Rocq, or Lean. Not functional correctness. The three backends do not prove the same lemmas.
Assumed
An axiom: DH commutativity, KEM correctness, AEAD round-trip, CSPRNG, …
Cited
Recorded from pinned third-party proofs (libcrux), not re-proved here
Open
Stated as a goal, or the model cannot express it yet
A theorem that says “decrypt recovers encrypt” under AEAD axioms is a statement about the glue, not a proof that AES-GCM is secure.

## Assumed primitives​

Unless an upstream suite is cited, the four libraries treat these as axioms:
Diffie–Hellman and signatures (signal-protocol)
HKDF-SHA256
AES-256-GCM
Operating-system randomness
ML-KEM lattice math (ml-kem cites pinned libcrux; it does not re-prove Kyber)

## How this relates to the product​

Recipient encryption stacks AES, Signal, PQXDH, RSA hybrid, and ML-KEM. Library tests, models, and type-checks do not compose into a proof of the Glitr messenger, git mailbox delivery, or a stolen host. crypto-cascade models layer order and wire v3 as composition only. The messenger does not claim formal verification.
Product attackers (git host, CORS proxy, STUN, stolen device) are engineering analysis on Threat model. They are not in these ProVerif models. An in-house review of the assembled client is on Security audit. Library models are not that review, and that review is not an independent audit.
See Cryptography and Roadmap.
Next: the verification report — shared summary, then sections for developers, cryptographers, and security reviewers.