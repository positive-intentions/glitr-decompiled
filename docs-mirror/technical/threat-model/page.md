# Source: https://glitr.io/docs/technical/threat-model/
# HTTP: 200

Technical
Threat model
Threat model

# Threat model

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. This folder is the product threat model for Glitr — the messenger whose client crate is whatsup. It names what we try to protect, who we treat as an attacker, and where the shipped platforms differ.
It is engineering analysis, not an audit and not a theorem. Library ProVerif models stay on Formal verification. Those models are a Dolev–Yao network attacker. They do not include a git host, a CORS proxy, or a stolen device. This folder talks about those parties in product language. Code-backed findings on the assembled client stay on Security audit — an in-house review, not an independent audit.
The user-facing one-liner is unchanged: protect message content from the host; do not expect the host to be unaware that you chat. That split is on encryption visibility. These pages expand it.

## Scope​

In scope: the assembled client (whatsup, whatsup-ui) and the workspace crates it calls — glitr-client, api-core, db-core, git-core, crypto-cascade, crypto-core, webrtc-core, tor-core, and the handshake libraries inside the recipient cascade.
Out of scope as a security claim: anonymity, wire compatibility with the Signal app, formal verification of the messenger, and safety against a compromised endpoint or a host who already has your encryption password.
How those limits land in practice: Residual risk.

## Two crypto jobs​

Do not mix these when you read a row.
Job
Unlock
What it is for
Recipient cascade
The contact’s public bundles and the shared protocol session
Encrypt a payload to a person (outbound, signaling, live bodies, live file bytes)
Password at rest
The encryption password you type at connect
Seal your mailbox documents so a clone without the password is an envelope
The cascade is AES → MLS → Signal → PQXDH (inner), then RSA hybrid → ML-KEM (outer). Stacking is defense-in-depth, not a spec-recommended profile. Architecture draws both jobs. Cryptography names the libraries.

## How to read this section​

Every table cell is one of four words. None of them means “proved.”
Word
Meaning
Documented
Stated as product behavior on these pages or the layer docs they link
Mitigated
A control exists for that attacker and that asset (cascade, sealing, TOFU, allowlists, …)
Partial
A control exists and is incomplete, platform-specific, or easy to bypass
Open
No product control, or the control is not a claimed path
In scope means we discuss the attacker here. Out of scope means we do not claim a property. Residual means even if the documented controls work, this risk remains.

## Trust boundaries​

api-core runs in process. There is no Glitr account server and no Glitr chat host. The website does not hold mailboxes. What sits outside the process is still an attacker surface. Optional Clerk sign-in (distribution builds) is a client-side UI gate only — it does not replace mailbox unlock or peer crypto identity.
The architecture product map is the same stack without the attacker labels. Record visibility — which folders a peer poll can read — is on Data structures.

## How to read the pages​

Page
Role
Report
Dated executive view: Glitr bar vs ideal-messenger gaps
Assets and attackers
What we protect, who we discuss, STRIDE mapped onto Glitr
Platforms
Web, desktop, mobile, and TUI as first-class surfaces
Git mailbox
Host, CORS, metadata, terms of service
Live links
WebRTC, Pair now, STUN/ICE, out-of-band bundles
Tor
Onion pairing, SOCKS for git, web-via-native proxy
Devices and storage
Keys, passwords, multi-device merge
Residual risk
Non-goals, leftover risk, audit status
Implementation findings (IDs, severities) are the security-audit report, not this folder.
Next: the report — what is addressed, what is outstanding, and the ideal-messenger gap list.