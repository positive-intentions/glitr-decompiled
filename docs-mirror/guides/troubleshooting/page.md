# Source: https://glitr.io/docs/guides/troubleshooting/
# HTTP: 200

How-tos
Troubleshooting

# Troubleshooting

Start from the symptom. Most failures are credentials, CORS, a contact URL pointing at the wrong repo, a Pair now handshake that never completed, or a live path the network will not allow.

## Login fails (local-only)​

Encryption password is required.
On the web, the git helper (git-web.js) must load even for a local workdir.
Extensions that block scripts or storage can break Start on this device.

## Login fails on the web (git remote)​

Check, in order:
HTTPS git URL, not SSH.
Token still valid, with write on that repo — not only public_repo, not expired.
CORS proxy reachable. If the default proxy is down, connect will fail even with a perfect token.
Browser console: the git helper must load. Extensions that block scripts or storage will break connect.
Desktop builds skip the browser CORS rule; if web fails and desktop works, it is almost certainly proxy/CORS.

## Attach remote refused​

Foreign / non-chat remotes cannot be attached (force does not apply). Same-app remotes that diverged open a confirm dialog to force-push this device’s history onto the remote. Empty remotes and remotes already this device’s history attach without confirm. See Git remotes on Profile.

## Connect asks to adopt remote​

If this device already has a local workdir whose history diverges from the remote at login, confirm to discard local-only commits and reclone from the remote. See Connect your repository.

## Attach says authentication failed / Edit​

Username or token was rejected on fetch. Tap Edit (or the pencil on that remote) to reopen credential fields, fix them, and attach again. Re-attaching the same URL updates stored credentials.

## Contact stays Pending​

They have not published a public profile yet, or you cannot fetch their repo.
They should finish profile setup and stay connected once so it can push.
Confirm you pasted their repo URL, not yours.
If their repo is private, your contact token must be able to read it.
Wait for poll, open the chat (that retries), or tap Retry.
Pair-now contacts skip git verify — if Pair now failed, see below, not Pending.

## Pair now / not going Live (QR)​

Both tabs must stay open. Check that the live / QR helpers loaded in the browser console. Invite then reply must be exchanged (do not scan your own code). Camera or file decode needs a secure context. Paste still works without a camera.

## Cannot send (composer disabled)​

Disabled only for Pending and Unreachable (git contacts). Active, Connecting, and Live can send. Connecting uses git; Live uses the direct link. Pair-now Live needs the data channel; if it dropped, re-pair.

## Stuck on a single tick​

Local write succeeded; your push has not. Open the poll toast:
write denied — the host accepted the username and refused git-receive-pack. Classic GitHub token needs repo; fine-grained needs Contents: Read and write. Fetch (read) can still work, which is why the app looks “half connected.” Use Edit on Profile remotes if attach/auth failed earlier.
Network 403 on receive-pack is the same class of bug. Glitr will not hammer the proxy retrying receive-pack after a denial.
Slow push timings mean the host or proxy is just slow — wait, then Poll now.
Local-only sessions have no remote push; single tick forever on Talk now usually means you expected git delivery without remotes.

## No message on the other side​

Git URL contacts
On the receiver, the contact URL must be the sender’s repo.
Sender ticks should be at least double gray (pushed).
Receiver should Poll now (or wait the countdown).
Receiver must not be logged into the sender’s mailbox by mistake.
Pair-now contacts
Both tabs must stay Live. Closing a tab ends delivery; there is no git outbound fallback.

## Not going Live (git poll)​

Both apps must stay open and poll. Signaling is encrypted to the peer’s public key and stored on git; if verify never finished, handshake cannot start.
Status should move Connecting → Live. A later poll should not flip one side to a hard failure just because the same offer is still on the board (it is applied once).
Both stuck Connecting with ICE checking: the network cannot form the peer connection. Use git. That is a successful messenger on a hostile network, not a broken install.
Close the Pair now panel if you want git signaling to resume for that contact.
Browser console: the live helper must load.

## Poll feels slow​

The toast lists phases: peer fetch, ingest, own sync/push. A large fetch is contact clones; a large push is publishing you. After a write denial, Glitr should not waste a full classify fetch on every idle poll.

## Refresh logged you out​

Logout clears session credentials on purpose. A normal refresh, with credentials still in that tab’s session storage, should auto-reconnect and paint from the local copy. Local-only logout keeps the workdir; remote-backed logout wipes local clones.

## Still stuck​

Re-read Talk now, Connect your repository, and Your first conversation as a checklist. The messaging try experience is the same protocol with two tabs — useful for isolating “is it my host?” from “is it this device?”