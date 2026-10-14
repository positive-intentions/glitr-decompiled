# Source: https://glitr.io/docs/technical/security-audit/
# HTTP: 200

Technical
Security audit
Security audit

# Security audit

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. This folder is a draft in-house implementation review of the assembled Glitr client as it stands. It starts at whatsup and follows only the crates that shell calls. It is not locked to a release tag.
It is not an independent security audit. It does not close the roadmap row for regular security audits. In-house or AI-authored notes are not a substitute. The threat-model leftover already says the same thing.
The user-facing one-liner is unchanged: protect message content from the host; do not expect the host to be unaware that you chat. The app is shared for testing and demo. Do not put sensitive details in it.

## How this relates to the other folders​

Folder
What it is
What it is not
Threat model
Product attackers, assets, STRIDE, leftover risk
A findings catalog with IDs
Formal verification
Library tests, ProVerif, stub type-checks
A proof of send, poll, git, or a stolen device
This folder
Code-backed findings on the assembled client
A third-party attestation, a pentest of a live host, or a score against another app
Finding IDs cite threat-model rows when the leftover was already named. New implementation evidence is labeled as such.

## Scope​

In scope: whatsup, whatsup-ui, and the in-process stack they dispatch into — glitr-client, api-core, db-core, git-core, crypto-cascade, crypto-core, signal-protocol-core, pqxdh-core, ml-kem-core, webrtc-core, tor-core.
Out of scope as a claim: anonymity, Signal-app wire compatibility, formal verification of the messenger, safety against a compromised endpoint, and a host who already has the encryption password.
Out of scope as a review unit: gallery apps (webrtc-gallery, api-gallery, crypto-gallery, mls-gallery) and the mls-core library until it is wired into whatsup, except to mark a control a gallery has that whatsup does not ship. Hosted HTTP headers for GitHub Pages are not in this repo.
How the review was done: Methodology.

## How to read this section​

Threat-model words (Documented / Mitigated / Partial / Open) still apply when a row maps onto that folder. This folder adds:
Word
Meaning
Critical / High / Medium / Low / Info
Impact if the finding is real on a shipped path
Confirmed
Observed in current code
Mitigated
Reserved for a later draft if a Confirmed claim stops holding (unused here)
Accepted
Known residual already on the threat-model leftover list
Out of scope
Not used by whatsup
None of those words means “proved.” None of them means “independently audited.”
Evidence on these pages is a file path, a function name, an impact sentence, and residual risk. There are no exploit recipes, payloads, or reproduction procedures.

## Component inventory​

api-core runs in process. There is no Glitr account server and no Glitr chat host.
Piece
Role in the review
whatsup
Shell: GUI, TUI, live, onion, device pair, PWA
whatsup-ui
Presentation. No cascade or protocol code
glitr-client
ApiHandle, SavedConnect, web storage, proxy, Tor glue
api-core
Routes, session, cascade wiring, hello, devices
db-core / git-core
Sealed mailbox documents; push / poll
crypto-cascade
Product encrypt / decrypt (encrypt_for_peer)
Handshake cores
Signal, PQXDH, ML-KEM, crypto-core primitives
webrtc-core
Signaling bundles, chat frames, peer RPC
tor-core
Arti session, SOCKS, hidden service

## Trust boundaries​

The same stack without attacker labels is on Architecture. Record visibility is on Data structures.

## How to read the pages​

Page
Role
Report
Dated executive view: counts, Glitr-bar mapping, you may say / may not
Methodology
What was reviewed, what was not done
Standards comparison
Coverage map vs ASVS/MASVS/SSDF, protocol specs, and third-party audit maturity
Findings
Master catalog with IDs
Cryptography
Cascade as wired, prekeys, groups, formal-verification gap
MLS pairwise review
Pairwise MLS layer + GroupSession faithfulness review
Identity and pairing
TOFU, hello, demo keys, device-pair snapshot
Git mailbox
Host metadata, CORS, tokens, static prekeys
Live and WebRTC
STUN, SDP, DC trust, peer RPC, proxy allowlist
Tor
SOCKS, onion, WebRTC split trust
Client and storage
glitr:connect, CSP, service worker, rendering
Remediation
Recommended fixes mapped to IDs
Next: the report — what the review found, what you may say, and what stays Open.