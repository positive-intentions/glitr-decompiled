# Source: https://glitr.io/docs/how-it-works/delivery/
# HTTP: 200

How it works
Why not always instant

# Why messages are not always instant

Popular messengers hide a data center in the middle. You send; their server holds the line until the other phone wakes. Instant (or close) is easy when someone else is always online on your behalf.
Glitr refuses that shape. The always-online party is your git host, and git hosts are not chat servers. They accept a push when you make one. They do not ring a peer.

## The clock you are on​

You encrypt the line and push it to your repo.
They may be offline. Nothing waits on them.
They later fetch your repo and decrypt what is addressed to them.
They write inbox plus a read receipt on their repo.
Your next poll fetches that receipt and turns ticks blue.
Each step is a network round trip you already understand from git. Polling (about every ten seconds, plus Poll now) is how the app notices new tips. That is why a quiet contact is seconds, not the blink of a typing indicator from a socket in a data center.

## When it is instant​

Both apps running, handshake complete, Live on the contact. Then the sequence above is skipped for that send — text and files ride the data channel. Close a laptop and you are back on the clock for text (git-URL contacts). Files wait for Live again.

## Ticks are the honest UI​

A single tick is not a lie: it means “on this device.” Double gray means “on the mailbox” (or acked live). Blue means “they opened it.” If a product showed a single check and called it delivered to a person who is camping, it would be guessing. Glitr waits for git (or the live ack).

## Design trade​

You gain a mailbox you can clone. You lose the feeling that a corporation is holding your unread line in RAM. If you need the feeling more than the mailbox, you will prefer a conventional messenger — and you will be renting their uptime again.
How to live with the trade: Send, receive, and read receipts.