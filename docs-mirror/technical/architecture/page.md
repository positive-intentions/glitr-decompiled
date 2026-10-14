# Source: https://glitr.io/docs/technical/architecture/
# HTTP: 200

Technical
Architecture

# How Glitr is assembled

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. Glitr is a messenger whose “cloud” is a git repository you own. This page is the product map: which pieces talk to which, and what each one is for.
It is not a protocol specification. Treat this as a picture of how the shipped stack fits together, not a claim of wire compatibility with the Signal app.
For the user story of mailboxes, ticks, and live links, start at Git as your mailbox. For what a host can see, see encryption visibility. The product threat model — assets, platforms, and residual risk — is Threat model.

## The pieces​

These are the parts of the product path. Signal, ML-KEM, and PQXDH appear as libraries inside the recipient cascade, not as separate apps you run.
Piece
What it does
whatsup
The Glitr client (web PWA, desktop, mobile, TUI). It hosts connect, chats, contacts, live pairing, and calls. UI/UX
whatsup-ui
The chat chrome: lists, threads, ticks, pairing panels, and the terminal frontend. UI/UX
glitr-client
Shared session, poll, send, and UI adapter used by whatsup and the api-gallery story. Network
api-core
The in-process messaging API: connect, profile, contacts, groups, send, poll, seal/unseal. It orchestrates storage and crypto. Network
db-core
A small GraphQL store over files. Documents and collections live in your git workdir. Types marked for sealing are password-wrapped on flush. Persistence
git-core
Git for that workdir. On a desktop it uses a native git library. In the browser it uses isomorphic-git and origin-private storage. Persistence
crypto-cascade
Encrypt to a person: AES → MLS → Signal → PQXDH (inner), then RSA hybrid → ML-KEM (outer). Pairwise MLS per contact. Cryptography
crypto-core
The primitives: AES, RSA, Argon2id password envelopes, and a generic cascade manager. Password sealing for your files lives here. It is not part of the recipient stack. Persistence
mls (sibling library)
RFC 9420 suite-1 façade (mls-core). Pairwise MLS inside 1:1 cascade; Glitr groups use N-party GroupSession with outer delivery seals. Cryptography, MLS formal verification
webrtc-core
The live path: git-brokered signaling, a data channel, chat frames (text, receipts, and file offer/chunk/ack/complete/cancel/resume) that use the same cascade, adaptive chunking with backpressure, and a git fallback for text and files when the channel drops (git-URL contacts). Groups stay on git. Network
tor-core
Native Arti client. After login it bootstraps circuits, publishes one duplex hidden service per device, opens framed onion chat, and offers localhost SOCKS so git HTTPS can leave through Tor. The browser never links it. Network
glitr-website
This documentation site.
glitr-domain
Hosting for glitr.io and app.glitr.io. It is not a chat server.
The UI talks to api-core. api-core writes records through db-core, encrypts recipient payloads with crypto-cascade, and uses webrtc-core when both people are online. Native whatsup starts tor-core through glitr-client; api-core never links Arti. glitr-website and glitr-domain only publish this site.

## Built as layers​

The product map above is the assembled stack. The crates were built in this order, each with a public gallery:
UI/UX — whatsup shell and whatsup-ui chrome.
Cryptography — Signal, ML-KEM, and PQXDH.
Persistence — git, a crypto utility, and a GraphQL store.
Network — schema-driven routes, git as a WebRTC broker, and native Tor.

## Git mailbox: send and poll​

You never write into someone else’s repository. You write outbound on yours. They keep a fetch-only clone of yours and ingest what is addressed to them. The product story is in Git as your mailbox; this is the same flow named by pieces.
Send also keeps a sent copy on your side (sealed at rest). Their poll writes inbox (also sealed). If they mark the message read, a read receipt appears on their repo; your later poll can drop the outbound row. Delivery ticks and latency live on Why not always instant.

## Live path​

Git is a mailbox, not an instant messenger. When both of you are here, Glitr publishes a signaling row on each repo (offer or answer, already cascaded to the other person), completes a WebRTC handshake, and carries new chat bodies and file attachments on the data channel — the same cascade, not plaintext over DTLS. A file is sealed first, then the ciphertext is chunked; the data channel itself stays a text JSON frame (chunks are slices of that ciphertext).
Local sent and inbox still update so the thread does not forget what you just said. Incomplete file transfers persist in a sealed file-transfers collection so a dropped channel can resume. If the channel dies, the next text send is git again; files wait for Live. Details: Direct live links.
On native desktop and TUI there is a third live path that does not use ICE: onion pairing. Each device publishes a duplex hidden service. A QR or paste bundle (on1:) exchanges the .onion address. Chat frames on that link use the same cascade. There is no git mailbox fallback — if the circuit is down, delivery waits. Linux WebKitGTK often has no RTCPeerConnection; onion is the WebRTC-free path there.
Git, WebRTC, and onion share one protocol session per contact so the ratchet does not fork — see Data structures.

## Two crypto jobs​

These must stay separate.
Recipient cascade
Password at rest
Job
Encrypt a payload to a contact
Seal your documents so a host is not reading the file
Unlock
Their public bundles and the shared session
The encryption password you type at connect
Where
Outbound, signaling, live chat bodies, live file payloads
Contacts, inbox, sent, file transfers, profile secrets, protocol sessions
Stacking Signal plus a standalone ML-KEM wrap plus PQXDH is defense-in-depth, not a spec-recommended profile. PQXDH already includes X25519, ML-KEM-1024, and a ratchet. The extra layers are independent wraps on the same payload. Full wording: Signal and post-quantum. The encryption try path can toggle a password layer for teaching; the product recipient path never does.

## Where it runs​

Cores are Rust libraries. They run in a desktop process and, compiled to WebAssembly, in the browser. tor-core is the exception: Arti, SOCKS, and hidden services stay in the native process. WASM never links it.
The browser app uses those WASM builds plus origin-private storage for the git workdir. Onion chrome is hidden. A web tab can only use Tor by proxying through a linked native device.
Try pages under Try Glitr open the product app for messaging, and Dioxus galleries for live, storage, and crypto. The galleries are how you press the buttons; they are not a second messenger.
glitr.io is this static documentation site. It does not hold mailboxes.
Next: the layer pages, starting at UI/UX. Record shapes stay on Data structures.