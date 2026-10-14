# Source: https://glitr.io/docs/technical/persistence/
# HTTP: 200

Technical
Persistence

# Persistence

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. The mailbox is files in a git repository you own. Three crates make that usable as an app store: git talks to the workdir, crypto seals documents at rest, and db queries structured records as GraphQL.
This page is those mechanics. Recipient encryption (Signal, ML-KEM, PQXDH) stays on Cryptography. The messaging record shapes — profile, contacts, outbound, inbox — stay on Data structures.
These crates are closed-source. Galleries are the public surface.

## git​

git-core is a GitRepo trait with two backends. Desktop uses gitoxide (gix) on the native filesystem. The browser uses isomorphic-git and origin-private storage, reached from Rust through wasm-bindgen.
gitoxide does not yet run as a whole on wasm32-unknown-unknown (index, refs, config, tempfile, transport), and it memory-maps packfiles — OPFS has no mmap. The trait keeps stories platform-agnostic until that lands upstream.
Open the git gallery
Story
What it exercises
Live
Init
GitRepo::init in a throwaway workdir
gui/git/init
Files
write / read / edit / remove / list
gui/git/files
Status
mutations then GitRepo::status
gui/git/status
Clone
authenticated clone, commit, branches, log, diff, fetch, pull, push
gui/git/clone
Storage
files as a store
gui/git/storage
Sync
remotes staying in step
gui/git/sync
Clone on the web needs a CORS proxy (the gallery default is a public demo proxy). Connection fields persist in localStorage. Desktop skips the proxy.
On native desktop and TUI, git remotes can instead leave through the session SOCKS proxy (socks5h://127.0.0.1:…) that Tor registers after login (POST /sync/socks). The browser still uses the CORS proxy; it does not run Arti. Arti’s own cache and keys live under a separate data directory (positive-intentions/api), not inside the mailbox repo. How that session is started is on Network.
GitHub, GitLab, and Codeberg may treat this mailbox use as against their terms. They may throttle, block, or close the repo. A first-party git server as part of the service is a later possibility, not something to wait on. Background: this thread. How to start on a host you already have: Using GitHub, GitLab, and Codeberg.
Try path: Your storage.

## crypto​

crypto-core is the utility crate the store imports: hashing, AES-256-GCM, RSA-OAEP-4096, Argon2id password envelopes, a CSPRNG, and a cascade manager. It is a Rust-native primitive set. It is not interoperable with the older JS cryptography app (different KDF, envelope, and hashing conventions).
Area
Algorithm
Random
getrandom (Web Crypto in WASM)
Hashing
SHA-256, SHA-512, SHA3-512 over raw bytes
Symmetric
AES-256-GCM (12-byte nonce, 16-byte tag)
Asymmetric
RSA-OAEP-4096 with SHA-256 (SPKI / PKCS#8 DER)
Password
Argon2id (19 MiB, t=2, p=1) → AES-256-GCM, versioned envelope
Cascade
Ordered layers. AES and password live in core; Signal, PQXDH, and ML-KEM appear in the gallery.
Product at-rest sealing is Argon2id then AES-GCM on flush. That job is separate from the recipient cascade. Architecture draws the split. Tests and models for the envelope and the cascade composition are on crypto formal verification.
The gallery’s multi-protocol story can stack Signal, PQXDH, and ML-KEM so you can see layers. The product recipient path does not add a password layer.
Open the crypto gallery
Story
Live
Random
gui/primitives/random
Hashing
gui/primitives/hashing
AES-GCM
gui/primitives/aes-gcm
RSA-OAEP
gui/primitives/rsa-oaep
Password encryption
gui/primitives/password-encryption
ML-KEM-1024
gui/primitives/ml-kem-1024
Signal
gui/primitives/signal
PQXDH
gui/primitives/pqxdh
Multi-protocol cascade
gui/cascade/multi-protocol
Try path: Encryption tools.

## db​

db-core is a small GraphQL engine over a StorageBackend. Memory is the default. The git feature uses git-core. Types marked @encrypted (GraphQL SDL) or "encrypted": true (JSON schema) are password-wrapped through crypto-core on flush.
The developer API is GraphQL query, mutation, and polling subscription strings. There is no commit, push, or pull on that surface. The app loads a schema (Schema::from_json_str or Schema::from_sdl); git stores JSON row files plus schema.json and schemaVersion.json. An empty repo is bootstrapped from the app schema.
Layout is hybrid JSON: singletons as one file, collections as folders of row files, nested collections under parents. Conflicts are per document when two clients edit the same JSON row. Schema-stamp conflicts are blocking. Multiple remotes act as equal-peer backups: sync pushes the same snapshot to each reachable remote, seeds empty ones, and surfaces mismatches.
When the store is behind the app schema, Migration::from_diff plus Database::open_with_migrations rewrites files. from_diff matches types and fields by name. Drop ops delete data.
Open the db gallery
Story
Live
Chat database
gui/database/chat-database
Schema designer
gui/database/schema-designer
Migrations
gui/database/migrations
Record shapes for the messenger (v8) are on Data structures. Try path: Encrypted records.
Next: Network — schema-driven routes, git as a WebRTC broker, and native Tor.