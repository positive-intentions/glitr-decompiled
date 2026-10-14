# Source: https://glitr.io/docs/technical/network/
# HTTP: 200

Technical
Network

# Network

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. Git plus a schema can store a mailbox. A messenger also needs routes, session checks, and a path that does not wait on the next poll. api is that logic layer. webrtc is the live path, with git as the connection broker. tor is the native live path that does not wait on a poll and does not need ICE.
This page is those crates and how the product wires them. The mailbox send/poll story is Git as your mailbox. Delivery ticks and latency are on Why not always instant.
These crates are closed-source. Galleries are the public surface.

## api​

api-core is an in-process messaging API: REST-style routes and middleware over db-core git storage. Router::from_schema reads the messaging schema and registers /resources/* endpoints — list, get, create, update, delete — from each document and collection type. Hand-written routes sit beside those: connect, profile, contacts, groups, send, poll, seal/unseal.
Middleware such as require_session and require_profile runs before a handler. That is how a peer poll is authenticated without a Glitr account server.
glitr-client is the shared session, snapshot, poll, send, and UI adapter. Both whatsup and the api-gallery story use it. The product shell on UI/UX mounts chrome; this crate is what the chrome talks to.
On every target it ships onion URL and QR helpers (tor.rs) with no Arti. Native desktop and TUI add tor_native.rs: that is the process that bootstraps circuits after login. api-core never links Arti. It stores onion contacts, device addresses, a SOCKS proxy for git, and live frames that arrived without a poll.
The git path is push, fetch, and poll. Messages arrive when the other person next fetches your outbound. That is reliable offline and slower than a popular messenger. Delivery names the ticks.
Open the e2e messaging gallery
The gallery home is positive-intentions.github.io/api. Two browser tabs can connect remotes and send on the same client the product uses.

## webrtc​

Git is a mailbox. When both people are at the keyboard, Glitr wants a direct data channel. webrtc-core treats each person’s repository as a signaling bulletin board: publish an offer or answer (already cascaded to the contact), complete ICE, then carry chat frames on the channel.
The isolated gallery is a two-browser QR or paste handshake. Each side prepares a bundled offer (wu1: payload). Scanning produces a reply; scanning the reply finishes the connection. No git remotes are required in that demo.
Open WebRTC meet
The gallery home is positive-intentions.github.io/webrtc. Try path: Live link.

## tor​

Live WebRTC still needs ICE, and some native webviews do not have it. tor-core is an in-process Arti session for those builds: directory bootstrap, a localhost SOCKS5 proxy, one duplex v3 hidden service per device, and length-prefixed framed links to a peer .onion.
After login, native whatsup calls glitr-client to start that session. Arti state lives under a private data directory, not in the mailbox repo. SOCKS is registered on the session so git HTTPS can leave through Tor (socks5h://127.0.0.1:…). The published onion address is stored on this device and on onion contacts. Chat frames on the link use the same recipient cascade as other live bodies.
A local safety number lets both sides check the hello keys. The math lives in api-core as well as tor-core, so a browser can verify a pairing without linking Arti.
The isolated gallery (tor-gallery) is a desktop Dioxus story: bootstrap, fetch, publish an onion, SOCKS, and Tor Meet pairing. The browser build of that gallery only attaches over WebRTC to a native window. There is no GitHub Pages URL; wasm32 cannot open Tor circuits.
Web Glitr does not link tor-core. Onion chrome is hidden. A web tab can only send onion or git-over-Tor frames by proxying through a linked native device on an already-open data channel. Mobile Cargo enables the Tor feature; it is not a claimed product path.

## How the product joins them​

There are three contact kinds.
For git-URL contacts, glitr-client starts establishing a WebRTC connection in the background while git remains the offline path. Signaling rows live on each repo. When the channel is up, new 1:1 text and files ride it — the same recipient cascade as outbound. Groups stay on git. If the channel drops, the next text send is git again; files wait for Live. On a native build the same contact can also use the session SOCKS proxy for that git traffic. Live WebRTC itself does not ride Tor.
Pair now is a separate product path: a plaintext QR or paste bundle (wu1:) exchanged out of band, stored as webrtc:…, with no git mailbox fallback. Details of that path: Direct live links.
Onion is the native third path. Contacts + → Onion exchanges a QR or paste bundle (on1:) with your hidden-service address. The contact is stored as onion://… (or onion://pairing while the invite is unfinished). Like Pair now, there is no git mailbox fallback. When both sides are up, frames go over the duplex onion link.
If a contact can use more than one live hop, WebRTC is tried first, then onion.
Git, WebRTC, and onion share one protocol session per contact so the ratchet does not fork. That record is on Data structures. What those paths mean to an attacker is Threat model.
Next: Data structures — the records in a mailbox, what is public vs sealed, and what sits inside a ciphertext field.