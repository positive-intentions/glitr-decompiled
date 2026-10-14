# Source: https://glitr.io/docs/technical/data-structures/
# HTTP: 200

Technical
Data structures

# Mailbox records and how they relate

Research, not an auditThese pages are research and development and may not match the live app. Do not treat this as an audit. Shared for testing and demo only. Please use responsibly. Glitr does not rent you a database. It stores structured documents in a git repository and queries them like a small app schema. This page is the record model: what kinds of files exist, which ones a contact can fetch, and what a ciphertext field actually holds.
You should not edit these files by hand.
Three layers, so the password is not confused with message encryption:
The store — a schema stamp, documents vs collections, optional at-rest sealing.
The messaging types — profile, contacts, devices, groups, outbound, inbox, sent, receipts, signaling, protocol sessions, file transfers.
The wire inside ciphertext — cascade (current) or an older hybrid envelope.
The user-facing mailbox story is Git as your mailbox. What a host can infer is encryption visibility.

## The store​

On first connect, Glitr writes a schema stamp into the repository (schema.json and schemaVersion.json). Those stamps stay readable. They name the app (messaging) and a version number. If two clients disagree on shape, a migration can rewrite files; as a user you mostly notice “please update the app.” How that store is built — GraphQL over git files, optional sealing, migrations — is on Persistence.
Kind
On disk
Messaging examples
Document
One singleton file
profile-public.json, profile-secrets.json
Collection
A folder of rows, one file per id
outbound/{id}.json, contacts/{id}.json, devices/{id}.json
Sealed type
Same path, but the file is a password envelope
Contacts, groups, inbox, sent, secrets, protocol sessions, file transfers
Sealing (@encrypted in the schema) is not end-to-end messaging. On flush, Glitr derives a key from your connect password (Argon2id) and wraps the JSON in AES-GCM. GraphQL inside the running app still sees plaintext. A clone of your repo without that password sees only the envelope.
Types without sealing are ordinary JSON. Contacts are meant to fetch those.

## What is visible vs sealed​

Visible on the git remote
Sealed with your connect password
profile-public.json
profile-secrets.json
outbound/
contacts/
readreceipts/
groups/
signaling/
inbox/
devices/
sent/
protocol-sessions/
file-transfers/
Peers poll your public profile, outbound, read receipts, and signaling. They do not open your sealed folders. devices/ is local JSON (including this device’s onion address); a clone of the repo can read it, but a peer poll does not fetch it.

## Messaging types (v8)​

The app schema is messaging, version 8. Fields below are in plain language — not a GraphQL listing. Older mailboxes migrate in place (attachment fields stay empty until you send a file; group fields stay empty until you create or ingest a group; onion and device fields stay empty until you pair on those paths).
Profile ids are how two repositories address each other (senderId, recipientId, peerProfileId). Contact ids are local only: “this person in my address book.”

### ProfilePublic — one file, peer-visible​

Your public identity. Holds an RSA public key and protocol bundles: the Signal, PQXDH, and ML-KEM public material a contact needs to encrypt the first message to you.

### ProfileSecrets — one file, sealed​

Your display name, handle, RSA private key, and matching protocol secrets. Never something a peer poll reads.

### Contact — collection, sealed​

Someone you added: their profile id, the address you use to reach them, a display name, optional git credentials, last poll time, and connection status. After a successful profile fetch, the row also caches peerPublicKey and peerProtocolBundles so later encrypts do not have to re-read their public profile.
gitUrl is that reachability field. A mailbox contact is an https://… git remote. Pair now stores webrtc:…. Onion pairing stores onion://…, or onion://pairing while the QR invite is unfinished. Direct-peer URLs (webrtc: and onion:) skip git poll — there is no mailbox to fetch.
localOnionAddress is your published hidden-service address for that row (copied so the UI can show it). It is not the peer’s onion. Peer device onions live in peerDevices. A safety number is computed locally from both hello keys so you can check the pair; it is never sent on the wire. The same math lives in api-core, so a browser can verify without Arti.

### Device — collection, local​

This device, and later your other devices. Holds a label, platform, and onionAddress once native Tor has published a duplex hidden service. Peers do not poll this folder; they see a cached copy on the contact (peerDevices). Device files are ordinary JSON, not password-sealed.

### OutboundMessage — collection, peer-visible​

Mail on your repo. Addressed to a recipientId. The payload that matters is ciphertext (cascade or hybrid). A body field may exist as a local convenience; peers decrypt the ciphertext, not that column. ackReadReceiptIds notes receipts you have already taken into account. Optional groupId marks a fan-out copy that belongs to a group thread.

### ReadReceipt — collection, peer-visible​

Lives on the reader’s repository. Points at an outbound messageId and who marked it read. Live receipts can skip git and travel on the data channel instead (live links).

### InboxMessage — collection, sealed​

Your local copy of a decrypted inbound line: plaintext body, who sent it, which outbound id it came from, which contact it is filed under. Optional attachment fields (attachmentName, attachmentMime, attachmentSize, attachmentData) hold a file after unseal (live or git). attachmentData is local bytes (base64) inside this sealed document. On the git path the file also travels inside the outbound ciphertext envelope. Optional groupId files the line under a fan-out group.

### SentMessage — collection, sealed​

Your local archive of what you sent: plaintext, contact id, outbound id, timestamps, and when it was marked read. The same optional attachment fields as inbox, plus optional groupId.

### Group — collection, sealed​

A fan-out thread you created or ingested: title and member profile ids (JSON). There is no shared group repository. Each send protects the chat plaintext with shared N-party MLS (GroupProtocolSession), then delivers one outer-sealed envelope per other member (welcome / commit / app). Membership changes use POST/DELETE /groups/:id/members and advance the MLS epoch on the next send.

### FileTransfer — collection, sealed​

In-progress (or recently finished) live file send/receive. Stores the cascade ciphertext, how many bytes have been acked, phase, and which contact it belongs to. Used to resume after a data-channel drop. Peers never fetch this folder. Live transfers still use this resume state. Smaller files can also ride the git send envelope.

### SignalingSession — collection, peer-visible​

One row per contact while you are connected. Holds cascaded WebRTC handshake material (offer or answer). Logout deletes the row.

### ProtocolSession — collection, sealed​

One row per contact. Stores ratchet state (Signal, PQXDH, and pairwise MLS) so later messages do not redo the handshake. Git, WebRTC, and onion share this session so the ratchet does not fork.

### GroupProtocolSession — collection, sealed​

One row per Glitr group. Stores N-party MLS group export (GroupSession) used for group chat payloads. Distinct from pairwise ProtocolSession.
Git cannot consume one-time prekeys off a peer repository the way a live prekey server would. The signed prekey is the last-resort path. Handshake wording: Signal and post-quantum.

## Lifecycle of a git message​

Send writes Outbound (readable ciphertext) and Sent (sealed plaintext). Their poll copies addressed outbound into Inbox. Marking read writes ReadReceipt on their repo. Your later poll can remove the outbound file. That is how ticks move from sent to delivered to read on the git path.

## Layout in a repository​

Each collection row is {folder}/{id}.json. Schema stamps stay plaintext. Sealed folders still have those paths; the file contents are envelopes.

## Schema versions​

v1 — profiles, contacts, outbound, receipts, inbox, sent.
v2 — adds signaling so git can broker a live link.
v3 — adds public protocol bundles, private protocol secrets, and per-contact ProtocolSession so the cascade can ratchet.
v4 — optional attachment fields on inbox/sent, plus a sealed FileTransfer collection so live file sends can persist and resume.
v5 — sealed Group rows and optional groupId on outbound, inbox, and sent. A send to a group encrypts one copy per member.
v6 — caches peerPublicKey and peerProtocolBundles on each Contact so encrypt can use the last fetched profile without another read.
v7 — adds localOnionAddress on each Contact.
v8 — adds a Device collection (onionAddress on this device) and contact address fields (gitMailbox, peerDevices).
Old outbound rows that still look like { wrappedKey, nonce, ciphertext } still decrypt (hybrid fallback). New sends use the cascade envelope.

## What sits in ciphertext​

Outbound, signaling, live chat bodies, and live file payloads share the same recipient encryption. Detection is by shape, not by folder name. A live file is sealed as one cascade blob; the data channel then carries slices of that ciphertext in file frames, not a second encryption layer.
Cascade (current) — a JSON object with v: 3 and scheme: "cascade". It holds only the outermost ML-KEM ciphertext and that layer's public headers (outerParameters). Signal and PQXDH handshake blobs ride inside the RSA + ML-KEM wrap on a first message. Layer names and plaintext length stay off the wire. Later messages ratchet using the stored ProtocolSession. A leftover v: 2 envelope (handshake and sizes in the clear) still decrypts after the same identity check.
Inner plaintext (v: 1) — after decrypt, a 1:1 text send is still raw UTF-8. A group send or attachment is a JSON object { v: 1, body, … } with optional groupId, groupTitle, groupMemberIds, and attachment fields (attachmentName, attachmentMime, attachmentSize, attachmentData). That inner envelope is what ingest files under a group and what fills inbox attachment columns.
Hybrid (v1 fallback) — { wrappedKey, nonce, ciphertext }. RSA-OAEP-4096 wraps a one-shot AES-256-GCM key. Used when a peer has no protocol bundles yet, and still accepted on ingest so old rows open.
Password envelopes on sealed files are a different JSON (kdf, aead, salt, nonce, ciphertext). They never travel as the recipient payload.

## How to read this without mixing layers​

A git host sees files and traffic. Sealed folders are envelopes. Outbound and signaling are ciphertext meant for a contact, not for the host. See what a host can see and the Threat model.
Your password opens your mailbox documents. It does not encrypt a message to someone.
The cascade is how a payload is addressed to a person. Architecture of that stack: Architecture and Signal and post-quantum.