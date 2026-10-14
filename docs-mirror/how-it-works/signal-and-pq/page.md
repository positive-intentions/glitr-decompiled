# Source: https://glitr.io/docs/how-it-works/signal-and-pq/
# HTTP: 200

How it works
Signal and post-quantum

# Signal Protocol and the post-quantum path

Glitr’s encryption story has two timescales: computers we have today, and computers we may have later. Recipient-addressed payloads (git outbound, signaling SDP, live chat bodies, live file payloads) run a cascading cipher: AES → Signal → PQXDH on the inside, then RSA hybrid → ML-KEM on the outside.
Password sealing of your mailbox stays outside that cascade.

## Signal Protocol (open source)​

The classical handshake and ratchet — the machinery that lets two people agree on keys and keep advancing them so old messages stay gone if a key leaks later — is the Signal Protocol.
That library is the exception to Glitr’s closed product: you can inspect it.
Try it: signal.positive-intentions.com
Source: github.com/positive-intentions/signal-protocol
It is a research-grade implementation. Treat it as not a substitute for a formal audit of the whole messenger. It is still the piece we are willing to put under lights.

## Why Signal is not the whole story​

A messaging app that only did today’s elliptic-curve handshake would be finished for 2016. Glitr also needs post-quantum key agreement: algorithms expected to remain hard even if large quantum computers arrive.
That path uses:
ML-KEM (FIPS 203, the 1024-parameter set) as a standalone hybrid envelope in the cascade.
PQXDH — Signal’s post-quantum extended Diffie–Hellman: classical X25519 combined with ML-KEM-1024, so you do not throw away today’s hardness while you add tomorrow’s.
You can press the buttons:
ML-KEM
PQXDH
These experiences are spec-faithful educational implementations. They are not a claim of wire compatibility with the Signal messenger app, and they are not a warranty.
Stacking Signal + standalone ML-KEM + PQXDH is defense-in-depth. PQXDH already includes X25519, ML-KEM-1024, and a Double Ratchet. The extra Signal and ML-KEM layers are not a spec-recommended profile; they are extra independent wraps on the same payload.

## Password encryption at rest​

Separate from the handshake: the encryption password you type at connect derives a key (memory-hard hashing, then authenticated encryption) that seals documents in your mailbox. That is “the host should not read the file contents,” not “this is Signal.”
The encryption try path still has a gallery cascade (including a password layer) so you can see layers. The product path is mailbox sealing plus the recipient cascade (AES → Signal → PQXDH, then RSA hybrid → ML-KEM) plus the live link.

## How to talk about this without lying​

Safe sentences:
Messages are encrypted so a git host is not reading the thread.
Recipient payloads run AES → Signal → PQXDH, then RSA hybrid → ML-KEM.
The handshake work includes an open Signal Protocol library.
A post-quantum KEM and PQXDH are part of the shipped cascade, not only demos.
Unsafe sentences we will not use:
“Unaudited end-to-end forever against nation states.”
“Compatible with the Signal app.”
“The entire messenger is open source.”