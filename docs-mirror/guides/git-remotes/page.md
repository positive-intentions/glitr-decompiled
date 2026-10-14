# Source: https://glitr.io/docs/guides/git-remotes/
# HTTP: 200

How-tos
Git remotes

# Git remotes on Profile

Connect takes one primary URL (or none, if you started local-only). Extra remotes are backups of the same local history. You add, copy, and remove them on Profile.

## What you see​

Git remotes lists every attached URL with:
Copy (icon) — puts that URL on the clipboard for sharing with a peer.
Remove (icon) — asks for confirmation, then drops that remote. Local history stays. Removing the last remote returns the session to local-only.
Edit (pencil) — when credentials were rejected for that remote; prefills the attach form so you can fix username / token and save again.
The add form stays available: Git URL, username, access token, and CORS proxy (web).

## Attach rules​

A new remote is accepted when:
it is empty, or
it already has this chat schema and its tip is an ancestor of local HEAD (a normal push would fast-forward).
If the remote is the same chat app but its history has diverged (or is unrelated), attach asks you to confirm a force-push: this device’s history replaces the remote tip; remote-only commits are lost. Foreign / non-chat remotes stay refused — force does not apply. Duplicate URLs update credentials when you re-attach the same URL with a new token; they do not create a second row.
Naming: first remote is origin; further remotes are backup-2, backup-3, …. After a successful push to origin, the worker also pushes those backups.
If username / token fail on fetch, attach returns an auth error. Credential fields collapse and Edit reopens them (or use the pencil on a listed remote that needs edit).

## When to use backups​

Same history on a second host so one host outage does not erase the only remote copy.
Mint a new empty repo and attach it after you started local-only — local history is the source of truth and pushes with a normal (not force) push, unless you deliberately confirm overwrite on a divergent same-app remote.
Prefer private repos and tokens you control. See Using GitHub, GitLab, and Codeberg.

## After any remote is attached​

Logout wipe rules match remote-backed sessions: local own-repo and peer clones are wiped; remotes on the host are not. See Connect your repository.

## Related​

Connect your repository
Add a contact — paste a copied URL into Git URL add
Troubleshooting — attach refused, auth Edit