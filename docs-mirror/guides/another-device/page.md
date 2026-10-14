# Source: https://glitr.io/docs/guides/another-device/
# HTTP: 200

How-tos
Another device

# Move to another device

Glitr does not have an account you export as a file from us. What you need depends on how you used the first device.

## If you used a git mailbox​

Your identity for messaging is:
The git repository URL (and any backup remotes you also want)
A token that can read and write it
The encryption password
A new phone, laptop, or browser profile is a new local clone. Connect with those facts and the conversation reappears as git fetch + decrypt.

### On the new device​

Open Glitr (or install the PWA there).
Connect with the same primary URL and same encryption password.
Use a token that still has write access. You can mint a new token; you cannot skip the password.
Optionally attach the same backup remotes from Profile after connect.
The first connect clones. Large history takes longer. Chats paint as documents decrypt.

### Contacts on the new device​

Contacts live in your repository (encrypted). After connect and sync, they should return with the rest of the mailbox. You should not have to re-add everyone unless you never synced them, or you logged into a different repo by mistake.
Each device keeps its own local clones of peer repositories. The new device will fetch those peers again.

### Two devices at once​

Both can be connected to the same mailbox. They are two checkouts of one repo.
Push/pull can race. Glitr treats document conflicts per record; you may be asked to resolve a clash if both sides edited the same row before syncing.
Live handshake is per running app. Two devices may not both stay Live with the same peer in a tidy way. Prefer one active sender if you are debugging ticks.

## If you only used Talk now (local-only)​

Local-only history lives in that device’s workdir. It does not appear on a new device until you:
Attach an empty (or same-history) remote on the first device and let it push, then
Connect that URL on the new device with the same encryption password,
or you Pair now again with peers (live history does not replace a mailbox clone).

## What does not travel​

Session storage and the on-device clone from the old browser. Logout on a remote-backed session wipes local data only; local-only logout keeps the workdir.
Tokens you did not save. If the old token was only in that browser, mint a new one on the host.
The encryption password, unless you stored it yourself.

## Losing a device​

Revoke the token that device used (if any). The repository on the host is still the mailbox — clone it elsewhere with a new token and the password. If the lost device might still be online, treat it like a lost laptop: rotate tokens, and assume anything that device could decrypt is in play until you stop trusting that password. If there was never a remote, the lost device may have held the only copy.