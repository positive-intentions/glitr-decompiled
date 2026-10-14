# Source: https://glitr.io/docs/how-it-works/encryption-visibility/
# HTTP: 200

How it works
Encryption and visibility

# What is encrypted vs what a host can see

Encryption in Glitr is real, and it is not a cloak of invisibility. This page is the honest split.

## Sealed​

Message bodies, attachment bytes on inbox/sent, in-progress file-transfer rows, and similar documents marked to be encrypted at rest in the mailbox.
Your local working copy, unlocked with the encryption password you entered at connect.
Handshake material that is cascaded to the recipient (AES → Signal → PQXDH, then RSA hybrid → ML-KEM) before it is written to signaling (the other person can open it; the host should see ciphertext).
Sent and inbox records stored as sealed documents on the respective repos.
A host administrator browsing files should not be able to read “see you at 6.” They can still see file names, sizes, dates, and that the repo is busy.

## Intentionally readable (coordination)​

Some files are left readable so the other client can do its job without your password:
Read receipts on the git path — so the sender can turn ticks blue after a fetch.
Signaling envelopes as files on disk — the inner SDP is encrypted to the peer; the fact of a file per contact is visible.
Public profile material needed to verify a contact (the thing that moves Pending → Active).
If everything were sealed with only your password, nobody else could ever start a conversation with you.

## What a host can infer anyway​

Even with bodies sealed:
That you have a mailbox repository.
How often it changes, and roughly how large it is.
Who you gave read access to (collaborators, tokens).
IP addresses when you push and fetch (normal git hosting logs).
On the web, a CORS proxy also sits on the path and sees git HTTP.
Threat model in one line: protect message content from the host; do not expect the host to be unaware that you chat. The full product draft — attackers, platforms, live, Tor, devices, residual — is Threat model.

## What we can see​

We do not host your repository. We do not receive a copy of your token. If you use a public CORS proxy or a public try experience, that infrastructure sees what any web proxy sees. The Glitr website itself is a documentation site.

## Passwords and keys​

The encryption password is yours. Lose it, lose sealed documents.
Git tokens are host credentials. Revoke them on the host.
Live links are between devices. A network observer on the live path sees that a connection exists; chat bodies and file chunks are application-layer cascaded (same stack as git outbound), not only DTLS. File frames are slices of that ciphertext, not a second wrap.

## Open handshake​

The Signal Protocol implementation used for classical handshake work is open source. You can read it, run it, and audit it:
Live Signal experience
Source
The rest of Glitr is not published as source. Open Signal is a trust anchor for the handshake, not a promise that every layer of the messenger is public.
Post-quantum pieces (ML-KEM, PQXDH) are documented for users who want the “future computers” story: Signal Protocol and the post-quantum path.