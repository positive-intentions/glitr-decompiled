# Source: https://glitr.io/docs/guides/git-hosts/
# HTTP: 200

How-tos
Git hosts

# Using GitHub, GitLab, and Codeberg

Glitr stores a mailbox in your git repository. That is a deliberate choice: you can clone it, back it up, delete it, and move hosts. It is also an unusual choice, and you should go in with eyes open.

## What Glitr needs from a host​

HTTPS clone, fetch, and push.
A credential (usually a personal access token) that can read and write contents.
For private mailboxes, a way for a contact to read your repo (collaborator or a read token you give them).
Any host that speaks git-over-HTTPS can work. GitHub, GitLab, and Codeberg are the ones people try first.

## Tokens​

Create a token scoped as tightly as the host allows:
One repository, not the whole account, if the host offers that.
Contents read/write. You do not need Issues, Actions, or org admin.
Short expiry if you are only trying Glitr.
Revoke it when you stop using that device.
Never commit the token into the mailbox repository.

## Private vs public​

Private is the default you should want. A public mailbox publishes ciphertext and coordination files to the world. Ciphertext is not the same as “nobody knows I chat.” Traffic patterns, repo size, and timestamps leak.
Contacts need read access to a private mailbox. That is a small, intentional share — not “make it public so the URL works.”

## Terms of service​

Git hosts are built for source code and collaboration, not as a general-purpose encrypted mail server. Using GitHub, GitLab, Codeberg, or similar as the store for a messenger may violate their terms. They may throttle, block tokens, or close the repository.
Glitr does not currently run a git server for you. A hosted mailbox as part of the product is not something you should wait on.
Until then:
Prefer a host whose terms you have actually read.
Prefer a host you could replace (the data is git; migration is clone + new remote).
Do not bet a crisis-sensitive channel on a free consumer account staying in policy.

## CORS on the web​

Browser Glitr cannot talk to most git hosts without a CORS proxy. The default public proxy is for demos. It sees git HTTP. For a mailbox you care about, use a desktop build or a proxy you operate.

## What we will not do for you​

We will not file a ticket with your host. We will not reset a password. We will not retrieve a token from a browser you already logged out of. The host is your vendor; Glitr is the client.