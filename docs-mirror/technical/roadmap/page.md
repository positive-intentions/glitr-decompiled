# Source: https://glitr.io/docs/technical/roadmap/
# HTTP: 200

Technical
Roadmap

# Glitr Roadmap

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. No messaging system is completely secure. Rather than score Glitr against another app, this page lists the features and practices that would make a messenger as strong as it can be, then shows how close this project is.

## Requirements​


### Technical requirements​

P2P — no central Glitr server for exchanging messages
End to end encryption — intercepted payloads should not be readable
Forward secrecy — a compromised key should not decrypt past messages
Key management — you hold your own keys; there is no central key authority
Encrypted storage — messages stay sealed on the device
Secure signaling — the first connection between peers is established securely
Anonymity — you can talk without revealing who you are
Deniable authentication — participants can still be confident a message is authentic
Keys per contact — each connection has its own keys
Onion style routing — origins stay hidden

### User Experience requirements​

Remove registration — no personal details required to start
Support multimedia — share animations and videos
Offline messaging — send encrypted messages while the other person is offline
Self-destructing messages — optional delete-after-time
Minimize metadata — hide who is talking to whom, and when

### Optimistic requirements​

Open source — so experts can audit the code and users can decide whether to trust it
Minimal infrastructure — fewer services to fail or attack
Regular security audits — find and fix issues promptly

## Status​

Glitr only.
done done
in-progress in-progress
planned planned
not applicable n/a (this iteration dropped a prior approach)

### Transport and architecture​

Item
Description
Status
Notes
P2P
Live WebRTC is peer-to-peer. Native onion chat is peer-to-peer over a hidden service. The git mailbox is still push and poll through a host. Product ICE is STUN-only (no TURN in the messenger bundle). Pair now and onion have no git fallback.
Live vs offline, live links, network
Git backend
The mailbox is a git repository you own.
Git as your mailbox
Minimal infrastructure
No Glitr chat server. You still need a git host, public ICE/STUN, and a CORS proxy on the web. A hosted mailbox and a peer broker are not something to wait on.
Architecture, git hosts
Onion style routing
Native desktop and TUI bootstrap Arti, publish one duplex hidden service per device, pair over an onion QR, and chat on framed links. Git HTTPS can use the same SOCKS proxy. Live WebRTC still cannot go over Tor. The browser cannot run Arti; a web tab can only proxy through a linked native device. Mobile is not a claimed path. Peers can still agree, over an already protected channel, to use different TURN servers — an extra IP mask, not an onion network. A VPN can hide a WebRTC address leak; it has its own tradeoffs.
Network, live links, git mailbox, git remotes, WebRTC and onion routing, WebRTC IP leaking

### Encryption and keys​

Item
Description
Status
Notes
End to end encryption
Payloads are encrypted to a contact so a git host is not reading the thread.
Signal and post-quantum
Cascading cipher
AES → MLS → Signal → PQXDH (inner), then RSA hybrid → ML-KEM (outer). Pairwise MLS per contact.
Signal and post-quantum, Cryptography
Forward secrecy
Double Ratchet is documented. The product is unaudited.
Signal and post-quantum
Key management
You hold the connect password, git tokens, and profile/protocol keys. There is no full key-management UX.
Data structures
Keys per contact
One ProtocolSession (ratchet state) per contact.
Data structures
Encrypted storage
Sealed collections use Argon2id then AES-GCM on flush.
Architecture
Secure signaling
Git-brokered SDP is cascade-encrypted to the contact. Pair now is a plaintext QR/paste bundle (wu1:, out of band). Onion pairing is a plaintext QR/paste bundle (on1:) with hidden-service addresses. ICE/STUN still exist on the WebRTC path.
Live links, network
Deniable authentication
Not claimed on product crypto pages.
—

### Privacy​

Item
Description
Status
Notes
Anonymity
Listed as a requirement. Product docs currently disclaim it.
What Glitr is
Minimize metadata
Bodies are sealed. A host can still see activity, access, and IPs.
Encryption visibility
Remove registration
No Glitr account and no phone-number directory.
What Glitr is
Remove installation
The browser app works without install. PWA, desktop, mobile, and TUI are optional extras.
Install

### Messaging and UX​

Item
Description
Status
Notes
Offline messaging
Git-URL contacts can send while the other person is offline. Pair now and onion have no mailbox fallback.
Live vs offline, network
Support multimedia
Files on live and git; voice and video on an open live link. Not on the TUI. Groups stay on git. Live files cap at 32 MiB.
Live links
Group messaging
Fan-out groups: one mailbox per member, groupId on send/ingest; add/remove members via REST/UI. Crypto is pairwise cascade+MLS per member (not N-party MLS).
Data structures, MLS, Cryptography
Self-destructing messages
Not in the data model.
—

### Trust and documentation​

Item
Description
Status
Notes
Open source
The Signal handshake library is public. The rest of the messenger is not.
Architecture, signal-protocol
Threat model
A dedicated product draft covers assets, platforms, git, live, Tor, devices, and residual risk. Not an audit.
Threat model, report
Formal verification
Not claimed for the messenger. Library proofs: Formal verification.
Formal verification
Regular security audits
Closed-source. No independent audit. In-house review: Security audit.
Security audit
Developer documentation
Architecture, layer pages, records, and Try pages. App source is not published.
Architecture, UI/UX, cryptography, persistence, network, data structures
User documentation
Start, how-tos, and how-it-works.
Documentation

## How we got here​

The feature matrix below compares three positive-intentions messengers. If you lined them up on one axis, the useful label would be code transparency — not a simple open vs closed split. Cybersecurity hides details by complexity even when the source is public.

### positive-intentions/chat​

The first iteration: an open-source, browser-only messenger with no registration and no required install. It is plain JavaScript, uses the browser's built-in crypto, and can run from index.html or a static host such as GitHub Pages. A hosted copy exists to help people start. This code predates AI-assisted slop — there was slop before that too. Explaining it while it was being built was hard because the subject is dense.
Research side-projects and proofs of concept branched from this work. They are listed below, and they show up again in the feature matrix.
Key learnings:
Code quality and documentation needed to improve throughout.
Getting started had to be easier.
"Pure JavaScript" is not enough.
Branding was weak.
"Build it and they will come" does not work in cybersecurity.
More:
Chat project
Source
Demo

### Enkrypted Chat​

Feedback from the first iteration led to a second app, structured so pieces could be built and tested in isolation. It uses Webpack 5 module federation. The result is much more complex than iteration-1, and harder to explain.
That work also produced several micro-apps, compared in the feature matrix below.
Key learnings:
A browser-only JavaScript stack has real limits.
A self-authored security audit or formal verification does not count — especially if AI wrote it.
Open source is not sustainable at this stage.
Better UX earns more useful feedback than a demo, source, and docs alone.
More:
Getting started
Intro
Demo
Whitepaper

### Proofs of concept​

These are research side-projects that fed later work. They are not Glitr features.
File — dim useFS, demo
Crypto — source, demo
Signal Protocol — source, demo

### Glitr​

This site is the third iteration: a different stack in the same direction. Glitr is a Dioxus app — a browser PWA, with desktop and mobile targets — whose mailbox is a git remote you own. That remote brokers WebRTC when both people are online. See Architecture. It is the latest positive-intentions messenger, and it is still unfinished.
A text TUI exists for poll and send. Voice, video, and live pairing stay on the graphical app.
This iteration aims to be the strongest version of the idea so far. It is a work in progress.
More:
Documentation
Architecture
UI/UX
Cryptography
Persistence
Network

## Feature Matrix​


### Glossary​

The matrix compares three iterations of the same idea.
Iteration
App
iteration-1
positive-intentions/chat
iteration-2
Enkrypted Chat
iteration-3
Glitr
shipped as documented
partial, limited, or self-authored only
absent / not claimed
Glitr is the only iteration with desktop packages, mobile packages, and a TUI. Only iteration-1 can be self-hosted as an app. Self-host here means hosting the client, not the mailbox.

### Matrix​


#### Transport and architecture​

Feature
iteration-1
iteration-2
iteration-3
P2P
PeerJS backend
Git backend
Minimal infrastructure
Onion style routing

#### Encryption and keys​

Feature
iteration-1
iteration-2
iteration-3
End to end encryption
Cascading cipher
Forward secrecy
Key management
Keys per contact
Encrypted storage
Secure signaling
Deniable authentication

#### Privacy​

Feature
iteration-1
iteration-2
iteration-3
Anonymity
Minimize metadata
Remove registration
Remove installation

#### Messaging and UX​

Feature
iteration-1
iteration-2
iteration-3
Offline messaging
Support multimedia
Group messaging
Self-destructing messages

#### Platforms​

Feature
iteration-1
iteration-2
iteration-3
Web
Desktop
Mobile
TUI
Selfhosting

#### Trust and documentation​

Feature
iteration-1
iteration-2
iteration-3
Open source
Threat model
Formal verification
Regular security audits
Developer documentation
User documentation

## FAQ​

Why git? Self-hosting is the better fit. A generic host (GitHub, GitLab, and similar) is how most people start, and that use may break the host's terms — they may throttle, block, or close the repo. Read Using GitHub, GitLab, and Codeberg. Extra remotes stay in sync as backups of the same history; see Git remotes.
Serverless WebRTC? A connection can be made without a backend, as described in this issue.
Demo
Ready for production? No. The product aims at a secure experience, but it cannot be audited or reviewed. Shared for testing, feedback, and demo purposes only.
EU Chat Control? There is no Glitr account, no central chat server, and cryptography runs on the client. This older post is about a previous iteration; the mechanics are still similar.
Threat model? A dedicated product draft is on Threat model. The dated executive view is the report. It is engineering analysis, not an audit. The product is still unfinished.
An in-house implementation review is on Security audit. It does not count as an independent audit.
The messenger is closed-source and unaudited, so you should not have to take anyone's word for it. The app does not require sensitive details. Do not use any when you try it.
Audit feedback is further reading, not a product audit.
Open source? Glitr is not open source yet. That goes against the usual cybersecurity advice, and it is a goal, not a current claim. Older iterations and the Signal handshake library are public. glitr.io itself is closed-source.
AI? The first messenger (chat) was written without it. AI is part of the workflow now. A self-authored audit, especially one written with AI, still does not count. The Security audit folder is that kind of note.
Where can I find out more? https://positive-intentions.com/
https://www.reddit.com/r/positive_intentions
Reach out for clarity instead of treating the docs as an audit.