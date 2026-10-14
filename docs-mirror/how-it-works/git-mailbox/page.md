# Source: https://glitr.io/docs/how-it-works/git-mailbox/
# HTTP: 200

How it works
Git as your mailbox

# Git as your mailbox

Glitr does not run a message database in a data center we own. History can stay on the device. When you connect or attach remotes, each person brings a git repository. The app treats that repository as a structured, mostly encrypted mailbox. Remotes are optional; extra remotes are backups of the same local HEAD.

## Who writes where​

You fetch their mailbox. They fetch yours. Nobody writes into the other person’s repo.
Outbound lives on the sender’s repository. The payload is cascaded for the recipient (AES → Signal → PQXDH, then RSA hybrid → ML-KEM). Sending does not wait for their laptop. A background worker commits and pushes. Live 1:1 files stay on the data channel (plus a resume row under file-transfers/). Offline git DMs and group attachments write the file inside the outbound ciphertext envelope instead.
Inbox lives on the recipient’s repository, encrypted at rest, after their app has ingested outbound addressed to them.
Sent history lives on the sender’s repository, encrypted at rest.
Read receipts live on the recipient’s repository as coordination files (git path). Live receipts skip git and travel on the direct link.
Signaling lives on each person’s repository: one row per contact, refreshed while you are connected, deleted on logout. The SDP is cascaded to the recipient (same stack as message bodies); the file is there so git can broker a live link. Pair now can skip this bulletin board and exchange the invite out of band instead.
Peers are not queried live over a special API. Glitr keeps a local clone of each git contact’s repo (fetch only) and polls that clone. Pair-now contacts use a live address only — no peer clone.

## Why git​

You already know how to own it, clone it, and delete it.
Any host that speaks git-over-HTTPS can hold the bytes.
History is an audit log you can copy off-host.
Offline send is naturally “write a note, push when you can.”
Extra remotes keep the same tip on more than one host after a successful origin push.
The cost is the same as any git remote: latency, tokens, host policy, and the fact that a replica is a replica. See Using GitHub, GitLab, and Codeberg.

## Structured records, not a pile of blobs​

Under the files, Glitr stores structured documents — profiles, contacts, messages — so the app can query them like a small database. Some fields are marked to be encrypted; others are left readable because the other person’s client must act on them without your password.
You do not have to edit those files by hand. If you do, you can break the layout. Prefer the app.
Empty repositories are fine. The first connect or attach writes the schema and a version stamp. If two clients disagree on shape, migrations exist; as a user you mostly notice “please update the app.”

## Conflicts​

Two devices editing the same document (same contact row, same message id) can conflict. Conflicts are per document, not “the whole repo is on fire.” Profile and schema stamps are blocking if they diverge — finish one device’s sync before inventing a second identity in the same mailbox.

## What git is not doing​

Git is not encrypting for you. Encryption is a layer Glitr applies before write. Git is not the live path. Git is the store and (for git contacts) the bulletin board for handshakes. The live link is a separate connection that Pair now or git signaling helps you find.
Next: What is encrypted vs what a host can see.